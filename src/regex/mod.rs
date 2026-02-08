//! High-level regex API
//!
//! This module provides a safe, idiomatic Rust API built on top of
//! the c2rust-translated QuickJS regex engine.

// Allow attributes for c2rust-generated code (util, unicode, engine modules)
#![allow(internal_features)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(clippy::all)]

mod opcodes;
mod flags;
mod error;

// Bytecode buffer utility (used by compiler)
mod util;

// Pure Rust interpreter
mod interpreter;

// Pure Rust compiler (custom JS parser)
mod compiler;

// Selective applicative functor for regex static analysis
pub mod selective;

// Pike VM: thread-list based execution (linear time, no backtracking)
pub mod pikevm;

// Bit-parallel VM: wide-word interpreter for same bytecode (O(N/64) per byte)
pub mod bitvm;

// Legacy C engine modules — only needed for benchmark comparison via find_at_c_engine()
#[allow(dead_code)]
mod unicode;
#[allow(dead_code)]
pub(crate) mod engine;

pub use opcodes::OpCode;
pub use flags::{Flags, InvalidFlag};
pub use error::{Error, Result, ExecResult};
pub use pikevm::Scratch;

use std::ptr;

use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};
use memchr::{memchr, memchr2, memchr3, memmem};

// ============================================================================
// SIMD-accelerated character class matching
// ============================================================================

/// SIMD-accelerated digit finding for x86_64 with AVX2
#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
mod simd_x86 {
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    /// Find the first digit (0-9) in a byte slice using AVX2 SIMD.
    /// Returns the index of the first digit, or None if no digit found.
    #[inline]
    #[target_feature(enable = "avx2")]
    pub unsafe fn find_first_digit_avx2(bytes: &[u8]) -> Option<usize> {
        let len = bytes.len();
        if len == 0 {
            return None;
        }

        let ptr = bytes.as_ptr();
        let mut offset = 0;

        // Process 32 bytes at a time with AVX2
        if len >= 32 {
            // Create constants: we want to find bytes where (b - '0') <= 9
            // Since _mm256_cmpgt_epi8 does signed comparison, we use a trick:
            // Add 118 to transform '0'..'9' (48-57) to 166-175
            // Then compare > 165 to find non-digits, invert for digits
            // Actually, simpler: use unsigned saturation
            //
            // Better approach: use the range check trick
            // For unsigned comparison: (x - lo) <= (hi - lo)
            // '0' = 48, '9' = 57, so check (b - 48) <= 9
            // But we need unsigned comparison...
            //
            // Trick: For signed bytes, if we XOR with 0x80, we flip the sign bit
            // This converts unsigned ordering to signed ordering
            // But simpler: use saturating arithmetic
            //
            // Simplest approach for digits:
            // 1. Subtract '0' with wrapping (digits become 0-9, others wrap)
            // 2. Compare with max value that when added doesn't overflow for 0-9
            //    Use: cmpgt(9, sub(b, '0')) -- but signed, need adjustment
            //
            // Best approach for unsigned range check with signed intrinsics:
            // Check if (b - '0') <= 9 using: max_epu8(b - '0', 9) == 9
            // This works because max_epu8 is unsigned!

            let zero_char = _mm256_set1_epi8(b'0' as i8);
            let nine = _mm256_set1_epi8(9);

            while offset + 32 <= len {
                let chunk = _mm256_loadu_si256(ptr.add(offset) as *const __m256i);
                // Subtract '0' - digits become 0-9, others become large values (wrapping)
                let sub = _mm256_sub_epi8(chunk, zero_char);
                // Unsigned max with 9 - for digits (0-9), result is 9; for others, result is > 9
                let maxed = _mm256_max_epu8(sub, nine);
                // Compare equal to 9 - digits match, others don't
                let is_digit = _mm256_cmpeq_epi8(maxed, nine);
                // Get bitmask
                let mask = _mm256_movemask_epi8(is_digit) as u32;

                if mask != 0 {
                    return Some(offset + mask.trailing_zeros() as usize);
                }
                offset += 32;
            }
        }

        // Handle remaining bytes with SSE2 (16 at a time)
        if offset + 16 <= len {
            let zero_char = _mm_set1_epi8(b'0' as i8);
            let nine = _mm_set1_epi8(9);

            while offset + 16 <= len {
                let chunk = _mm_loadu_si128(ptr.add(offset) as *const __m128i);
                let sub = _mm_sub_epi8(chunk, zero_char);
                let maxed = _mm_max_epu8(sub, nine);
                let is_digit = _mm_cmpeq_epi8(maxed, nine);
                let mask = _mm_movemask_epi8(is_digit) as u32;

                if mask != 0 {
                    return Some(offset + mask.trailing_zeros() as usize);
                }
                offset += 16;
            }
        }

        // Handle remaining bytes scalar
        for i in offset..len {
            let b = *ptr.add(i);
            if b.wrapping_sub(b'0') <= 9 {
                return Some(i);
            }
        }

        None
    }
}

/// SIMD-accelerated digit finding for x86_64 with SSE2 (baseline)
#[cfg(all(target_arch = "x86_64", not(target_feature = "avx2")))]
mod simd_x86 {
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    /// Find the first digit using SSE2 (available on all x86_64)
    #[inline]
    #[target_feature(enable = "sse2")]
    pub unsafe fn find_first_digit_sse2(bytes: &[u8]) -> Option<usize> {
        let len = bytes.len();
        if len == 0 {
            return None;
        }

        let ptr = bytes.as_ptr();
        let mut offset = 0;

        // Process 16 bytes at a time with SSE2
        if len >= 16 {
            let zero_char = _mm_set1_epi8(b'0' as i8);
            let nine = _mm_set1_epi8(9);

            while offset + 16 <= len {
                let chunk = _mm_loadu_si128(ptr.add(offset) as *const __m128i);
                let sub = _mm_sub_epi8(chunk, zero_char);
                let maxed = _mm_max_epu8(sub, nine);
                let is_digit = _mm_cmpeq_epi8(maxed, nine);
                let mask = _mm_movemask_epi8(is_digit) as u32;

                if mask != 0 {
                    return Some(offset + mask.trailing_zeros() as usize);
                }
                offset += 16;
            }
        }

        // Handle remaining bytes scalar
        for i in offset..len {
            let b = *ptr.add(i);
            if b.wrapping_sub(b'0') <= 9 {
                return Some(i);
            }
        }

        None
    }
}

/// SIMD-accelerated digit finding for aarch64 with NEON
#[cfg(target_arch = "aarch64")]
mod simd_aarch64 {
    use std::arch::aarch64::*;

    /// Find the first digit using NEON (ARM64)
    #[inline]
    pub unsafe fn find_first_digit_neon(bytes: &[u8]) -> Option<usize> {
        let len = bytes.len();
        if len == 0 {
            return None;
        }

        let ptr = bytes.as_ptr();
        let mut offset = 0;

        // Process 16 bytes at a time with NEON
        if len >= 16 {
            let zero_char = vdupq_n_u8(b'0');
            let nine = vdupq_n_u8(9);

            while offset + 16 <= len {
                let chunk = vld1q_u8(ptr.add(offset));
                // Subtract '0' with wrapping
                let sub = vsubq_u8(chunk, zero_char);
                // Compare <= 9 (unsigned): use vcleq_u8
                let is_digit = vcleq_u8(sub, nine);

                // Find first match - NEON doesn't have movemask, so we reduce
                // Use horizontal max to check if any match
                let has_match = vmaxvq_u8(is_digit);
                if has_match != 0 {
                    // Find the exact position
                    let mut mask_bytes = [0u8; 16];
                    vst1q_u8(mask_bytes.as_mut_ptr(), is_digit);
                    for (i, &m) in mask_bytes.iter().enumerate() {
                        if m != 0 {
                            return Some(offset + i);
                        }
                    }
                }
                offset += 16;
            }
        }

        // Handle remaining bytes scalar
        for i in offset..len {
            let b = *ptr.add(i);
            if b.wrapping_sub(b'0') <= 9 {
                return Some(i);
            }
        }

        None
    }
}

// Inline SIMD digit finder for x86_64 with AVX2 (32 bytes at a time)
#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
#[inline(always)]
fn find_first_digit_simd(bytes: &[u8]) -> Option<usize> {
    // AVX2 is available at compile time - use 32-byte processing
    unsafe { find_first_digit_avx2_inline(bytes) }
}

// Fallback SSE2 for x86_64 without AVX2
#[cfg(all(target_arch = "x86_64", not(target_feature = "avx2")))]
#[inline(always)]
fn find_first_digit_simd(bytes: &[u8]) -> Option<usize> {
    // SSE2 is guaranteed on x86_64, use 16-byte processing
    unsafe { find_first_digit_sse2_inline(bytes) }
}

/// AVX2 implementation - processes 32 bytes at a time
#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
#[inline(always)]
unsafe fn find_first_digit_avx2_inline(bytes: &[u8]) -> Option<usize> {
    use std::arch::x86_64::*;

    let len = bytes.len();
    if len == 0 {
        return None;
    }

    let ptr = bytes.as_ptr();
    let mut offset = 0;

    // Process 32 bytes at a time with AVX2
    if len >= 32 {
        let zero_char = _mm256_set1_epi8(b'0' as i8);
        let nine = _mm256_set1_epi8(9);

        while offset + 32 <= len {
            let chunk = _mm256_loadu_si256(ptr.add(offset) as *const __m256i);
            let sub = _mm256_sub_epi8(chunk, zero_char);
            let maxed = _mm256_max_epu8(sub, nine);
            let is_digit = _mm256_cmpeq_epi8(maxed, nine);
            let mask = _mm256_movemask_epi8(is_digit) as u32;

            if mask != 0 {
                return Some(offset + mask.trailing_zeros() as usize);
            }
            offset += 32;
        }
    }

    // Handle tail with SSE2 (16 bytes)
    if offset + 16 <= len {
        let zero_char = _mm_set1_epi8(b'0' as i8);
        let nine = _mm_set1_epi8(9);

        while offset + 16 <= len {
            let chunk = _mm_loadu_si128(ptr.add(offset) as *const __m128i);
            let sub = _mm_sub_epi8(chunk, zero_char);
            let maxed = _mm_max_epu8(sub, nine);
            let is_digit = _mm_cmpeq_epi8(maxed, nine);
            let mask = _mm_movemask_epi8(is_digit) as u32;

            if mask != 0 {
                return Some(offset + mask.trailing_zeros() as usize);
            }
            offset += 16;
        }
    }

    // Scalar tail
    while offset < len {
        if (*ptr.add(offset)).wrapping_sub(b'0') <= 9 {
            return Some(offset);
        }
        offset += 1;
    }

    None
}

/// SSE2 implementation - processes 16 bytes at a time
/// SSE2 is always available on x86_64
#[cfg(all(target_arch = "x86_64", not(target_feature = "avx2")))]
#[inline(always)]
unsafe fn find_first_digit_sse2_inline(bytes: &[u8]) -> Option<usize> {
    use std::arch::x86_64::*;

    let len = bytes.len();
    if len == 0 {
        return None;
    }

    let ptr = bytes.as_ptr();
    let mut offset = 0;

    // Process 16 bytes at a time
    if len >= 16 {
        // Constants: check (b - '0') <= 9
        let zero_char = _mm_set1_epi8(b'0' as i8);
        let nine = _mm_set1_epi8(9);

        while offset + 16 <= len {
            let chunk = _mm_loadu_si128(ptr.add(offset) as *const __m128i);
            // Subtract '0' - digits become 0-9, others wrap to large values
            let sub = _mm_sub_epi8(chunk, zero_char);
            // Unsigned max(sub, 9) - for digits result is 9, for others > 9
            let maxed = _mm_max_epu8(sub, nine);
            // Compare equal to 9 - digits match
            let is_digit = _mm_cmpeq_epi8(maxed, nine);
            // Extract bitmask
            let mask = _mm_movemask_epi8(is_digit) as u32;

            if mask != 0 {
                return Some(offset + mask.trailing_zeros() as usize);
            }
            offset += 16;
        }
    }

    // Scalar tail
    while offset < len {
        if (*ptr.add(offset)).wrapping_sub(b'0') <= 9 {
            return Some(offset);
        }
        offset += 1;
    }

    None
}

#[cfg(target_arch = "aarch64")]
#[inline]
fn find_first_digit_simd(bytes: &[u8]) -> Option<usize> {
    // SAFETY: NEON is always available on aarch64
    unsafe { simd_aarch64::find_first_digit_neon(bytes) }
}

// Fallback for other architectures
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
#[inline]
fn find_first_digit_simd(bytes: &[u8]) -> Option<usize> {
    // Use the portable implementation
    bytes.iter().position(|&b| b.wrapping_sub(b'0') <= 9)
}

// Threshold for using optimizations (bytes)
const OPTIMIZATION_THRESHOLD: usize = 32;

// ============================================================================
// OwnedFinder - Precomputed memmem::Finder with owned needle
// ============================================================================

/// A memmem::Finder that owns its needle, avoiding per-search preprocessing.
/// For short patterns, uses a rare-byte search strategy for better performance.
struct OwnedFinder {
    // IMPORTANT: finder must be declared before needle for correct drop order.
    finder: memmem::Finder<'static>,
    needle: Box<[u8]>,
    // Precomputed rare byte info for short patterns
    rare_byte: u8,
    rare_byte_offset: usize,
}

impl Clone for OwnedFinder {
    fn clone(&self) -> Self {
        Self::new(self.needle.to_vec())
    }
}

/// Byte frequency heuristic - lower score = rarer byte
/// Based on English letter frequency
#[inline]
fn byte_rarity_score(b: u8) -> u8 {
    match b {
        // Most common English letters (higher score = more common)
        b'e' | b'E' => 255,
        b't' | b'T' => 240,
        b'a' | b'A' => 230,
        b'o' | b'O' => 220,
        b'i' | b'I' => 210,
        b'n' | b'N' => 200,
        b's' | b'S' => 190,
        b'h' | b'H' => 180,
        b'r' | b'R' => 170,
        b' ' => 250, // Space is very common
        // Less common letters
        b'l' | b'L' => 100,
        b'd' | b'D' => 90,
        b'c' | b'C' => 80,
        b'u' | b'U' => 70,
        b'm' | b'M' => 60,
        b'w' | b'W' => 50,
        b'f' | b'F' => 45,
        b'g' | b'G' => 40,
        b'y' | b'Y' => 35,
        b'p' | b'P' => 30,
        b'b' | b'B' => 25,
        b'v' | b'V' => 20,
        b'k' | b'K' => 15,
        // Rare letters and other chars
        b'j' | b'J' | b'x' | b'X' | b'q' | b'Q' | b'z' | b'Z' => 5,
        // Digits are moderately rare
        b'0'..=b'9' => 30,
        // Punctuation and other ASCII
        _ if b.is_ascii() => 20,
        // Non-ASCII is rare
        _ => 10,
    }
}

impl OwnedFinder {
    /// Create a new OwnedFinder from a needle.
    fn new(needle: Vec<u8>) -> Self {
        let needle = needle.into_boxed_slice();

        // Find the rarest byte in the needle
        let (rare_byte_offset, rare_byte) = needle.iter()
            .enumerate()
            .min_by_key(|(_, &b)| byte_rarity_score(b))
            .map(|(i, &b)| (i, b))
            .unwrap_or((0, needle.get(0).copied().unwrap_or(0)));

        // SAFETY: We extend the lifetime to 'static, but the finder is only
        // accessed through &self methods, and self owns the needle data.
        let finder = unsafe {
            let needle_ref: &'static [u8] = &*(needle.as_ref() as *const [u8]);
            memmem::Finder::new(needle_ref)
        };
        Self { finder, needle, rare_byte, rare_byte_offset }
    }

    /// Find the needle in the haystack.
    #[inline(always)]
    fn find(&self, haystack: &[u8]) -> Option<usize> {
        // Use precomputed memmem::Finder - best general-purpose algorithm
        self.finder.find(haystack)
    }

    /// Search by finding the rare byte first, then verifying the pattern.
    #[inline]
    fn find_rare_byte_first(&self, haystack: &[u8]) -> Option<usize> {
        let needle = &self.needle;
        let needle_len = needle.len();

        if haystack.len() < needle_len {
            return None;
        }

        let max_start = haystack.len() - needle_len;
        let rare_offset = self.rare_byte_offset;
        let mut search_start = 0;

        while search_start <= max_start {
            // Find the rare byte at its expected position
            let search_from = search_start + rare_offset;
            if search_from > haystack.len() {
                return None;
            }

            if let Some(pos) = memchr(self.rare_byte, &haystack[search_from..]) {
                let candidate = search_from + pos - rare_offset;

                // Check bounds
                if candidate > max_start {
                    return None;
                }

                // Verify the entire pattern
                if &haystack[candidate..candidate + needle_len] == needle.as_ref() {
                    return Some(candidate);
                }

                // Move past this position
                search_start = candidate + 1;
            } else {
                return None;
            }
        }
        None
    }

    /// Get the needle bytes.
    #[inline(always)]
    fn needle(&self) -> &[u8] {
        &self.needle
    }

    /// Get the needle length.
    #[inline(always)]
    fn len(&self) -> usize {
        self.needle.len()
    }
}

// ============================================================================
// ByteBitmap - 256-bit bitmap for fast byte set membership testing
// ============================================================================

/// A 256-bit bitmap for testing byte membership in O(1)
/// Inspired by regress's ByteBitmap for fast character class scanning
#[derive(Clone)]
struct ByteBitmap {
    /// 4 x u64 = 256 bits, one per possible byte value
    bits: [u64; 4],
}

impl std::fmt::Debug for ByteBitmap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ByteBitmap({} bytes set)", self.count())
    }
}

impl ByteBitmap {
    /// Create an empty bitmap
    #[inline]
    const fn new() -> Self {
        Self { bits: [0; 4] }
    }

    /// Create a bitmap from a slice of bytes
    fn from_bytes(bytes: &[u8]) -> Self {
        let mut bm = Self::new();
        for &b in bytes {
            bm.set(b);
        }
        bm
    }

    /// Set a byte in the bitmap
    #[inline]
    fn set(&mut self, byte: u8) {
        let idx = (byte >> 6) as usize;  // Which u64 (0-3)
        let bit = byte & 0x3F;           // Which bit (0-63)
        self.bits[idx] |= 1u64 << bit;
    }

    /// Test if a byte is in the bitmap (branchless!)
    #[inline(always)]
    fn contains(&self, byte: u8) -> bool {
        let idx = (byte >> 6) as usize;
        let bit = byte & 0x3F;
        (self.bits[idx] >> bit) & 1 != 0
    }

    /// Count set bits
    fn count(&self) -> u32 {
        self.bits.iter().map(|x| x.count_ones()).sum()
    }

    /// Find first matching byte in slice using chunk processing
    /// Processes 4 bytes at a time for better throughput
    #[inline]
    fn find_in_slice(&self, bytes: &[u8]) -> Option<usize> {
        let len = bytes.len();
        let mut i = 0;

        // Process 4 bytes at a time (branchless inner loop)
        while i + 4 <= len {
            // Check all 4 bytes, compute matches branchlessly
            let m0 = self.contains(bytes[i]) as usize;
            let m1 = self.contains(bytes[i + 1]) as usize;
            let m2 = self.contains(bytes[i + 2]) as usize;
            let m3 = self.contains(bytes[i + 3]) as usize;

            // If any matched, find which one (first match wins)
            if (m0 | m1 | m2 | m3) != 0 {
                if m0 != 0 { return Some(i); }
                if m1 != 0 { return Some(i + 1); }
                if m2 != 0 { return Some(i + 2); }
                return Some(i + 3);
            }
            i += 4;
        }

        // Handle remaining bytes
        while i < len {
            if self.contains(bytes[i]) {
                return Some(i);
            }
            i += 1;
        }

        None
    }
}

// ============================================================================
// Search Strategy - determines how to find potential match positions
// ============================================================================

/// Optimized search strategy extracted from pattern analysis
#[derive(Clone)]
enum SearchStrategy {
    /// Pattern is anchored to start - only try position 0
    Anchored,
    /// Pattern is ^literal - anchored pure literal
    AnchoredLiteral(Vec<u8>),
    /// Pattern is a pure literal - no engine needed! Uses precomputed Finder.
    PureLiteral(OwnedFinder),
    /// Single literal byte to search for
    SingleByte(u8),
    /// Two possible first bytes (e.g., alternation or case-insensitive)
    TwoBytes(u8, u8),
    /// Three possible first bytes
    ThreeBytes(u8, u8, u8),
    /// Multi-byte literal prefix with precomputed Finder
    LiteralPrefix(OwnedFinder),
    /// Alternation of pure literals - use Aho-Corasick!
    AlternationLiterals {
        literals: Vec<Vec<u8>>,
        ac: AhoCorasick,
    },
    /// Case-insensitive ASCII literal - search using case-folded comparison
    CaseInsensitiveLiteral {
        /// The pattern in lowercase for comparison
        lowercase: Vec<u8>,
        /// Original pattern length
        len: usize,
    },
    /// Pattern ends with a literal suffix - search backwards
    SuffixLiteral(Vec<u8>),
    /// Bitmap-based search for character classes with 4+ bytes
    Bitmap(ByteBitmap),
    /// Search for digit (0-9)
    Digit,
    /// Search for word char start (a-z, A-Z, 0-9, _)
    WordChar,
    /// Search for whitespace
    Whitespace,
    // === PURE FAST PATHS - No interpreter needed! ===
    /// Pure [0-9]+ or \d+ - scan consecutive digits directly
    PureDigitPlus,
    /// Pure [a-z]+ - scan consecutive lowercase letters
    PureLowerPlus,
    /// Pure [A-Z]+ - scan consecutive uppercase letters
    PureUpperPlus,
    /// Pure [a-zA-Z]+ - scan consecutive letters
    PureAlphaPlus,
    /// Pure [a-zA-Z0-9]+ - scan consecutive alphanumerics
    PureAlnumPlus,
    /// Pure \w+ or [a-zA-Z0-9_]+ - scan consecutive word chars
    PureWordPlus,
    /// Pure "[^"]*" - scan quoted strings directly
    QuotedString(u8), // the quote character
    /// Pure [A-Z][a-z]+ - capital letter followed by lowercase letters
    PureCapitalWord,
    /// Pure [a-z]+suffix - lowercase letters ending with a literal suffix
    PureLowerSuffix(Vec<u8>), // the suffix (e.g., "ing")
    /// No optimization available - scan every position
    None,
}

impl std::fmt::Debug for SearchStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchStrategy::Anchored => write!(f, "Anchored"),
            SearchStrategy::AnchoredLiteral(lit) => write!(f, "AnchoredLiteral({:?})", String::from_utf8_lossy(lit)),
            SearchStrategy::PureLiteral(finder) => write!(f, "PureLiteral({:?})", String::from_utf8_lossy(finder.needle())),
            SearchStrategy::SingleByte(b) => write!(f, "SingleByte({:?})", *b as char),
            SearchStrategy::TwoBytes(b1, b2) => write!(f, "TwoBytes({:?}, {:?})", *b1 as char, *b2 as char),
            SearchStrategy::ThreeBytes(b1, b2, b3) => write!(f, "ThreeBytes({:?}, {:?}, {:?})", *b1 as char, *b2 as char, *b3 as char),
            SearchStrategy::LiteralPrefix(finder) => write!(f, "LiteralPrefix({:?})", String::from_utf8_lossy(finder.needle())),
            SearchStrategy::AlternationLiterals { literals, .. } => {
                let strs: Vec<_> = literals.iter().map(|l| String::from_utf8_lossy(l).to_string()).collect();
                write!(f, "AlternationLiterals({:?})", strs)
            }
            SearchStrategy::CaseInsensitiveLiteral { lowercase, len } => {
                write!(f, "CaseInsensitiveLiteral({:?}, len={})", String::from_utf8_lossy(lowercase), len)
            }
            SearchStrategy::SuffixLiteral(lit) => write!(f, "SuffixLiteral({:?})", String::from_utf8_lossy(lit)),
            SearchStrategy::Bitmap(bm) => write!(f, "{:?}", bm),
            SearchStrategy::Digit => write!(f, "Digit"),
            SearchStrategy::WordChar => write!(f, "WordChar"),
            SearchStrategy::Whitespace => write!(f, "Whitespace"),
            SearchStrategy::PureDigitPlus => write!(f, "PureDigitPlus"),
            SearchStrategy::PureLowerPlus => write!(f, "PureLowerPlus"),
            SearchStrategy::PureUpperPlus => write!(f, "PureUpperPlus"),
            SearchStrategy::PureAlphaPlus => write!(f, "PureAlphaPlus"),
            SearchStrategy::PureAlnumPlus => write!(f, "PureAlnumPlus"),
            SearchStrategy::PureWordPlus => write!(f, "PureWordPlus"),
            SearchStrategy::QuotedString(q) => write!(f, "QuotedString({:?})", *q as char),
            SearchStrategy::PureCapitalWord => write!(f, "PureCapitalWord"),
            SearchStrategy::PureLowerSuffix(s) => write!(f, "PureLowerSuffix({:?})", String::from_utf8_lossy(s)),
            SearchStrategy::None => write!(f, "None"),
        }
    }
}

// ============================================================================
// Public Regex API
// ============================================================================

/// A compiled regular expression
pub struct Regex {
    /// Bit-parallel program (compiled at construction time, if pattern fits)
    bit_program: Option<bitvm::BitVmProgram>,
    /// The compiled bytecode (heap-allocated by C or Rust)
    bytecode: *mut u8,
    /// The original pattern (for Display)
    pattern: String,
    /// The flags
    flags: Flags,
    /// Optimized search strategy (heuristic-based)
    strategy: SearchStrategy,
    /// Rust-owned bytecode (when compiled by pure Rust compiler).
    /// When Some, bytecode ptr points into this Vec.
    owned_bytecode: Option<Vec<u8>>,
    /// Selective prefilter derived from AST analysis
    selective_prefilter: selective::Prefilter,
    /// Whether this pattern is safe for Pike VM (no backrefs)
    use_pike_vm: bool,
    /// Aho-Corasick automaton for multi-literal prefiltering
    ac_prefilter: Option<AhoCorasick>,
    /// Memmem finder for single-literal prefiltering
    memmem_prefilter: Option<memmem::Finder<'static>>,
}

// Regex is Send + Sync since the bytecode is immutable after compilation
unsafe impl Send for Regex {}
unsafe impl Sync for Regex {}

impl Regex {
    /// Compile a new regular expression
    pub fn new(pattern: &str) -> Result<Self> {
        Self::with_flags(pattern, Flags::empty())
    }

    /// Get the bytecode for debugging purposes
    #[doc(hidden)]
    pub fn debug_bytecode(&self) -> &[u8] {
        unsafe {
            let bc_len = self.bytecode_len();
            std::slice::from_raw_parts(self.bytecode, bc_len)
        }
    }

    /// Compile a new regular expression with flags (pure Rust)
    pub fn with_flags(pattern: &str, flags: Flags) -> Result<Self> {
        let (processed_pattern, extracted_flags) = extract_inline_flags(pattern);
        let mut final_flags = flags;
        final_flags.insert(extracted_flags.bits());

        // Parse and compile to bytecode
        let ast = compiler::parser::parse(&processed_pattern, final_flags)
            .map_err(|e| Error::Syntax(e.to_string()))?;

        let mut bytecode_vec = compiler::compile_regex(&processed_pattern, final_flags)
            .map_err(|e| Error::Syntax(e.to_string()))?;

        let bytecode_ptr = bytecode_vec.as_mut_ptr();
        let strategy = analyze_pattern(&processed_pattern, final_flags);

        // Selective analysis: derive prefilter from AST
        let ir = selective::from_ast(&ast);
        let info = selective::analyze(&ir);
        let sel_prefilter = selective::derive_prefilter(&info);

        // Build Aho-Corasick or memmem prefilter objects
        let (ac_prefilter, memmem_prefilter) = match &sel_prefilter {
            selective::Prefilter::AhoCorasickStart(patterns)
            | selective::Prefilter::AhoCorasickInner { patterns, .. }
                if patterns.len() >= 2 =>
            {
                let ac = AhoCorasickBuilder::new()
                    .match_kind(MatchKind::LeftmostFirst)
                    .build(patterns)
                    .ok();
                (ac, None)
            }
            selective::Prefilter::MemmemStart(needle)
            | selective::Prefilter::MemmemInner { needle, .. }
                if needle.len() >= 2 =>
            {
                let boxed: Box<[u8]> = needle.clone().into_boxed_slice();
                let leaked: &'static [u8] = Box::leak(boxed);
                let finder = memmem::Finder::new(leaked);
                (None, Some(finder))
            }
            _ => (None, None),
        };

        // Use Pike VM for patterns without backreferences or lookaround (guaranteed linear time).
        // Pike VM already handles Unicode word chars via is_alphanumeric().
        let use_pike = !info.has_backrefs && !info.has_lookahead;

        // Compile Wide NFA (dynamic-width bit-parallel program).
        // Used for fast first-pass match_end detection: O(states/64) per byte.
        // Works with any number of states (no 1024-state limit).
        // Registers are ignored (treated as simple splits) — correct for match_end only.
        // Skip for patterns with lookahead — bit VM can't traverse those opcodes.
        let bit_program = if use_pike && info.min_length > 0 && !info.has_lookahead {
            bitvm::BitVmProgram::compile(&bytecode_vec)
        } else {
            None
        };

        Ok(Regex {
            bit_program,
            bytecode: bytecode_ptr,
            pattern: pattern.to_string(),
            flags: final_flags,
            strategy,
            owned_bytecode: Some(bytecode_vec),
            selective_prefilter: sel_prefilter,
            use_pike_vm: use_pike,
            ac_prefilter,
            memmem_prefilter,
        })
    }

    /// Alias for with_flags — kept for API compatibility
    pub fn with_flags_pure_rust(pattern: &str, flags: Flags) -> Result<Self> {
        Self::with_flags(pattern, flags)
    }

    /// Test if the pattern matches anywhere in the text
    pub fn is_match(&self, text: &str) -> bool {
        // Bit VM fast REJECTION: if bit VM says no match, definitely no match.
        // If it says yes, verify (bit VM ignores assertions so has false positives).
        if let Some(ref prog) = self.bit_program {
            if !prog.has_match(text.as_bytes()) {
                return false;
            }
        }
        self.find(text).is_some()
    }

    /// Find the first match in the text
    ///
    /// Uses optimized search strategies based on pattern analysis.
    pub fn find(&self, text: &str) -> Option<Match> {
        let len = text.len();

        // For short inputs, just use the engine directly
        if len < OPTIMIZATION_THRESHOLD {
            return self.try_match_at(text, 0).or_else(|| {
                // Advance by first char's UTF-8 length (not 1 byte!)
                let first_char_len = text.chars().next().map(|c| c.len_utf8()).unwrap_or(1);
                self.find_at_linear(text, first_char_len)
            });
        }

        // Use the pre-computed search strategy
        match &self.strategy {
            SearchStrategy::Anchored => {
                // Only try at position 0
                self.try_match_at(text, 0)
            }

            SearchStrategy::AnchoredLiteral(literal) => {
                // Fast path: ^literal - just check prefix
                self.find_anchored_literal(text, literal)
            }

            SearchStrategy::PureLiteral(literal) => {
                // Fast path: pure literal patterns skip the engine entirely!
                self.find_pure_literal(text, literal)
            }

            SearchStrategy::SingleByte(b) => {
                self.find_with_single_byte(text, *b)
            }

            SearchStrategy::TwoBytes(b1, b2) => {
                self.find_with_two_bytes(text, *b1, *b2)
            }

            SearchStrategy::ThreeBytes(b1, b2, b3) => {
                self.find_with_three_bytes(text, *b1, *b2, *b3)
            }

            SearchStrategy::LiteralPrefix(prefix) => {
                self.find_with_literal_prefix(text, prefix)
            }

            SearchStrategy::Bitmap(bitmap) => {
                self.find_with_bitmap(text, bitmap)
            }

            SearchStrategy::AlternationLiterals { literals, ac } => {
                self.find_with_alternation_literals(text, literals, ac)
            }

            SearchStrategy::CaseInsensitiveLiteral { lowercase, len } => {
                self.find_case_insensitive_literal(text, lowercase, *len)
            }

            SearchStrategy::SuffixLiteral(suffix) => {
                self.find_with_suffix_literal(text, suffix)
            }

            SearchStrategy::Digit => {
                self.find_with_digit_scan(text)
            }

            SearchStrategy::WordChar => {
                self.find_with_word_char_scan(text)
            }

            SearchStrategy::Whitespace => {
                self.find_with_whitespace_scan(text)
            }

            // === PURE FAST PATHS - No interpreter needed! ===
            SearchStrategy::PureDigitPlus => {
                find_digit_run(text.as_bytes(), 0)
            }

            SearchStrategy::PureLowerPlus => {
                find_lower_run(text.as_bytes(), 0)
            }

            SearchStrategy::PureUpperPlus => {
                find_upper_run(text.as_bytes(), 0)
            }

            SearchStrategy::PureAlphaPlus => {
                find_alpha_run(text.as_bytes(), 0)
            }

            SearchStrategy::PureAlnumPlus => {
                find_alnum_run(text.as_bytes(), 0)
            }

            SearchStrategy::PureWordPlus => {
                find_word_run(text.as_bytes(), 0)
            }

            SearchStrategy::QuotedString(quote) => {
                find_quoted_string(text.as_bytes(), 0, *quote)
            }

            SearchStrategy::PureCapitalWord => {
                find_capital_word(text.as_bytes(), 0)
            }

            SearchStrategy::PureLowerSuffix(suffix) => {
                find_lower_suffix(text.as_bytes(), 0, suffix)
            }

            SearchStrategy::None => {
                self.find_at_linear(text, 0)
            }
        }
    }

    /// Find an anchored literal pattern (^literal) - just check prefix
    #[inline]
    fn find_anchored_literal(&self, text: &str, literal: &[u8]) -> Option<Match> {
        let bytes = text.as_bytes();
        if bytes.len() >= literal.len() && bytes.starts_with(literal) {
            Some(Match {
                start: 0,
                end: literal.len(),
            })
        } else {
            None
        }
    }

    /// Find a pure literal pattern - no engine needed!
    #[inline]
    fn find_pure_literal(&self, text: &str, finder: &OwnedFinder) -> Option<Match> {
        let bytes = text.as_bytes();
        // Use precomputed finder - no per-call preprocessing!
        finder.find(bytes).map(|pos| Match {
            start: pos,
            end: pos + finder.len(),
        })
    }

    /// Find using single-byte search (fastest) - used by find()
    #[inline]
    fn find_with_single_byte(&self, text: &str, byte: u8) -> Option<Match> {
        let bytes = text.as_bytes();
        let mut start = 0;

        while let Some(pos) = memchr(byte, &bytes[start..]) {
            let abs_pos = start + pos;
            if let Some(m) = self.try_match_at(text, abs_pos) {
                return Some(m);
            }
            start = abs_pos + 1;
            if start >= bytes.len() {
                break;
            }
        }
        None
    }

    /// Find using two-byte search - used by find()
    #[inline]
    fn find_with_two_bytes(&self, text: &str, b1: u8, b2: u8) -> Option<Match> {
        let bytes = text.as_bytes();
        let mut start = 0;

        while let Some(pos) = memchr2(b1, b2, &bytes[start..]) {
            let abs_pos = start + pos;
            if let Some(m) = self.try_match_at(text, abs_pos) {
                return Some(m);
            }
            start = abs_pos + 1;
            if start >= bytes.len() {
                break;
            }
        }
        None
    }

    /// Find using three-byte search - used by find()
    #[inline]
    fn find_with_three_bytes(&self, text: &str, b1: u8, b2: u8, b3: u8) -> Option<Match> {
        let bytes = text.as_bytes();
        let mut start = 0;

        while let Some(pos) = memchr3(b1, b2, b3, &bytes[start..]) {
            let abs_pos = start + pos;
            if let Some(m) = self.try_match_at(text, abs_pos) {
                return Some(m);
            }
            start = abs_pos + 1;
            if start >= bytes.len() {
                break;
            }
        }
        None
    }

    /// Find using literal prefix search (memmem) - used by find()
    #[inline]
    fn find_with_literal_prefix(&self, text: &str, finder: &OwnedFinder) -> Option<Match> {
        let bytes = text.as_bytes();
        // Use precomputed finder for iteration
        for pos in finder.finder.find_iter(bytes) {
            if let Some(m) = self.try_match_at(text, pos) {
                return Some(m);
            }
        }
        None
    }

    /// Find using bitmap-based byte set search (for character classes) - used by find()
    #[inline]
    fn find_with_bitmap(&self, text: &str, bitmap: &ByteBitmap) -> Option<Match> {
        let bytes = text.as_bytes();
        let mut start = 0;

        while let Some(pos) = bitmap.find_in_slice(&bytes[start..]) {
            let abs_pos = start + pos;
            if let Some(m) = self.try_match_at(text, abs_pos) {
                return Some(m);
            }
            start = abs_pos + 1;
            if start >= bytes.len() {
                break;
            }
        }
        None
    }

    /// Find by scanning for digits (0-9) - used by find()
    #[inline]
    fn find_with_digit_scan(&self, text: &str) -> Option<Match> {
        let bytes = text.as_bytes();
        let mut start = 0;

        while start < bytes.len() {
            if let Some(pos) = find_digit(&bytes[start..]) {
                let abs_pos = start + pos;
                if let Some(m) = self.try_match_at(text, abs_pos) {
                    return Some(m);
                }
                start = abs_pos + 1;
            } else {
                break;
            }
        }
        None
    }

    /// Find by scanning for word characters - used by find()
    #[inline]
    fn find_with_word_char_scan(&self, text: &str) -> Option<Match> {
        let bytes = text.as_bytes();
        let unicode_mode = self.flags.contains(Flags::UNICODE);
        let mut start = 0;

        while start < bytes.len() {
            // In Unicode mode, also scan for high bytes (could be Unicode word chars)
            let pos = if unicode_mode {
                find_unicode_word_char(&bytes[start..])
            } else {
                find_word_char(&bytes[start..])
            };

            match pos {
                Some(p) => {
                    let abs_pos = start + p;
                    if let Some(m) = self.try_match_at(text, abs_pos) {
                        return Some(m);
                    }
                    start = abs_pos + 1;
                }
                None => break,
            }
        }
        None
    }

    /// Find by scanning for whitespace (branchless using bitmap) - used by find()
    #[inline]
    fn find_with_whitespace_scan(&self, text: &str) -> Option<Match> {
        let bytes = text.as_bytes();
        let mut start = 0;

        while let Some(pos) = find_whitespace(&bytes[start..]) {
            let abs_pos = start + pos;
            if let Some(m) = self.try_match_at(text, abs_pos) {
                return Some(m);
            }
            start = abs_pos + 1;
            if start >= bytes.len() {
                break;
            }
        }
        None
    }

    /// Find alternation of pure literals using Aho-Corasick - BLAZING FAST!
    #[inline]
    fn find_with_alternation_literals(&self, text: &str, literals: &[Vec<u8>], ac: &AhoCorasick) -> Option<Match> {
        let bytes = text.as_bytes();
        // Aho-Corasick finds the earliest match across all patterns
        ac.find(bytes).map(|mat| {
            Match {
                start: mat.start(),
                end: mat.end(),
            }
        })
    }

    /// Find pattern with suffix literal by scanning backwards
    #[inline]
    fn find_with_suffix_literal(&self, text: &str, suffix: &[u8]) -> Option<Match> {
        let bytes = text.as_bytes();
        let finder = memmem::Finder::new(suffix);

        // Find all occurrences of the suffix and try to match from before each
        for pos in finder.find_iter(bytes) {
            // The pattern must end at pos + suffix.len()
            // Try matching from various positions before the suffix
            // For patterns like [a-z]+ing, we need to find where the match starts

            // Try matching at position 0 first if suffix is at start
            if pos == 0 {
                if let Some(m) = self.try_match_at(text, 0) {
                    return Some(m);
                }
            } else {
                // Try positions going back from the suffix
                // Limit how far back we look to avoid quadratic behavior
                let look_back = pos.min(64);
                for back in 0..=look_back {
                    let try_pos = pos - back;
                    if let Some(m) = self.try_match_at(text, try_pos) {
                        // Verify the match actually covers this suffix
                        if m.end >= pos + suffix.len() {
                            return Some(m);
                        }
                    }
                }
            }
        }
        None
    }

    /// Find a match starting at or after the given byte offset.
    /// Uses memchr scanning based on SearchStrategy for fast candidate position finding.
    /// This is the key optimization that makes pure-rust competitive with hybrid.
    pub fn find_at(&self, text: &str, start: usize) -> Option<Match> {
        let text_bytes = text.as_bytes();
        let len = text_bytes.len();

        // For short remaining text, just use linear scan
        if len.saturating_sub(start) < OPTIMIZATION_THRESHOLD {
            return self.find_at_linear(text, start);
        }

        // Bit VM fast rejection: if no match exists in remaining input, return immediately.
        // O(N/64) scan — much faster than Pike VM's O(N*states).
        if let Some(ref prog) = self.bit_program {
            if !prog.has_match(&text_bytes[start..]) {
                return None;
            }
        }

        // For large haystacks, use selective prefilter if it can skip-scan
        if len.saturating_sub(start) > 10_000 {
            if matches!(self.selective_prefilter,
                selective::Prefilter::MemmemStart(_) |
                selective::Prefilter::MemmemInner { .. } |
                selective::Prefilter::AhoCorasickStart(_) |
                selective::Prefilter::AhoCorasickInner { .. }
            ) {
                return self.find_at_with_selective_prefilter(text, start);
            }
        }

        match &self.strategy {
            SearchStrategy::Anchored => {
                // Only try at position 0
                if start == 0 {
                    self.try_match_at(text, 0)
                } else {
                    None
                }
            }

            SearchStrategy::AnchoredLiteral(literal) => {
                // Fast path: ^literal - just check prefix at start
                if start == 0 {
                    self.find_anchored_literal(text, literal)
                } else {
                    None
                }
            }

            SearchStrategy::PureLiteral(finder) => {
                // Use precomputed finder - no per-call preprocessing overhead!
                finder.find(&text_bytes[start..])
                    .map(|pos| Match { start: start + pos, end: start + pos + finder.len() })
            }

            SearchStrategy::SingleByte(b) => {
                self.find_at_single_byte(text, start, *b)
            }

            SearchStrategy::TwoBytes(b1, b2) => {
                self.find_at_two_bytes(text, start, *b1, *b2)
            }

            SearchStrategy::ThreeBytes(b1, b2, b3) => {
                self.find_at_three_bytes(text, start, *b1, *b2, *b3)
            }

            SearchStrategy::LiteralPrefix(prefix) => {
                self.find_at_literal_prefix(text, start, prefix)
            }

            SearchStrategy::Bitmap(bitmap) => {
                self.find_at_bitmap(text, start, bitmap)
            }

            SearchStrategy::AlternationLiterals { literals, ac } => {
                self.find_at_alternation_literals(text, start, literals, ac)
            }

            SearchStrategy::CaseInsensitiveLiteral { lowercase, len } => {
                self.find_at_case_insensitive_literal(text, start, lowercase, *len)
            }

            SearchStrategy::SuffixLiteral(suffix) => {
                self.find_at_suffix_literal(text, start, suffix)
            }

            SearchStrategy::Digit => {
                self.find_at_digit(text, start)
            }

            SearchStrategy::WordChar => {
                self.find_at_word_char(text, start)
            }

            SearchStrategy::Whitespace => {
                self.find_at_whitespace(text, start)
            }

            // === PURE FAST PATHS - No interpreter needed! ===
            SearchStrategy::PureDigitPlus => {
                find_digit_run(text_bytes, start)
            }

            SearchStrategy::PureLowerPlus => {
                find_lower_run(text_bytes, start)
            }

            SearchStrategy::PureUpperPlus => {
                find_upper_run(text_bytes, start)
            }

            SearchStrategy::PureAlphaPlus => {
                find_alpha_run(text_bytes, start)
            }

            SearchStrategy::PureAlnumPlus => {
                find_alnum_run(text_bytes, start)
            }

            SearchStrategy::PureWordPlus => {
                find_word_run(text_bytes, start)
            }

            SearchStrategy::QuotedString(quote) => {
                find_quoted_string(text_bytes, start, *quote)
            }

            SearchStrategy::PureCapitalWord => {
                find_capital_word(text_bytes, start)
            }

            SearchStrategy::PureLowerSuffix(suffix) => {
                find_lower_suffix(text_bytes, start, suffix)
            }

            SearchStrategy::None => {
                // Use selective prefilter if it has a literal scanner, otherwise linear scan
                if matches!(self.selective_prefilter,
                    selective::Prefilter::MemmemStart(_) |
                    selective::Prefilter::MemmemInner { .. } |
                    selective::Prefilter::AhoCorasickStart(_) |
                    selective::Prefilter::AhoCorasickInner { .. }
                ) {
                    self.find_at_with_selective_prefilter(text, start)
                } else {
                    self.find_at_linear(text, start)
                }
            }
        }
    }

    /// Try to match at an exact position (no scanning).
    /// Uses Pike VM for safe patterns (linear time), backtracker for others.
    /// For Pike VM patterns, uses the cached capture-free path first (fast scan),
    /// then falls back to full capture extraction only when needed.
    #[inline]
    fn try_match_at(&self, text: &str, pos: usize) -> Option<Match> {
        let text_bytes = text.as_bytes();
        let bytecode = self.bytecode_slice();

        if self.use_pike_vm {
            let vm = pikevm::PikeVm::new(bytecode, text_bytes);
            match vm.exec(pos) {
                pikevm::PikeResult::Match(caps) => {
                    let start = caps.get(0).copied().flatten()?;
                    let end = caps.get(1).copied().flatten()?;
                    Some(Match { start, end })
                }
                pikevm::PikeResult::NoMatch => None,
            }
        } else {
            let mut ctx = interpreter::ExecContext::new(bytecode, text_bytes);
            match ctx.exec(pos) {
                interpreter::ExecResult::Match => {
                    if let (Some(match_start), Some(match_end)) = (
                        ctx.captures.get(0).copied().flatten(),
                        ctx.captures.get(1).copied().flatten()
                    ) {
                        Some(Match { start: match_start, end: match_end })
                    } else {
                        None
                    }
                }
                interpreter::ExecResult::NoMatch => None,
            }
        }
    }

    /// Find match using single-byte memchr scanning
    #[inline]
    fn find_at_single_byte(&self, text: &str, start: usize, byte: u8) -> Option<Match> {
        let bytes = &text.as_bytes()[start..];
        let mut offset = 0;

        while let Some(pos) = memchr(byte, &bytes[offset..]) {
            let abs_pos = start + offset + pos;
            if let Some(m) = self.try_match_at(text, abs_pos) {
                return Some(m);
            }
            offset += pos + 1;
        }
        None
    }

    /// Find match using two-byte memchr scanning
    #[inline]
    fn find_at_two_bytes(&self, text: &str, start: usize, b1: u8, b2: u8) -> Option<Match> {
        let bytes = &text.as_bytes()[start..];
        let mut offset = 0;

        while let Some(pos) = memchr2(b1, b2, &bytes[offset..]) {
            let abs_pos = start + offset + pos;
            if let Some(m) = self.try_match_at(text, abs_pos) {
                return Some(m);
            }
            offset += pos + 1;
        }
        None
    }

    /// Find match using three-byte memchr scanning
    #[inline]
    fn find_at_three_bytes(&self, text: &str, start: usize, b1: u8, b2: u8, b3: u8) -> Option<Match> {
        let bytes = &text.as_bytes()[start..];
        let mut offset = 0;

        while let Some(pos) = memchr3(b1, b2, b3, &bytes[offset..]) {
            let abs_pos = start + offset + pos;
            if let Some(m) = self.try_match_at(text, abs_pos) {
                return Some(m);
            }
            offset += pos + 1;
        }
        None
    }

    /// Find match using literal prefix memmem scanning
    #[inline]
    fn find_at_literal_prefix(&self, text: &str, start: usize, finder: &OwnedFinder) -> Option<Match> {
        let bytes = &text.as_bytes()[start..];
        // Use precomputed finder for iteration
        for pos in finder.finder.find_iter(bytes) {
            let abs_pos = start + pos;
            if let Some(m) = self.try_match_at(text, abs_pos) {
                return Some(m);
            }
        }
        None
    }

    /// Find match using bitmap scanning for character classes
    #[inline]
    fn find_at_bitmap(&self, text: &str, start: usize, bitmap: &ByteBitmap) -> Option<Match> {
        let bytes = &text.as_bytes()[start..];
        let mut offset = 0;

        while let Some(pos) = bitmap.find_in_slice(&bytes[offset..]) {
            let abs_pos = start + offset + pos;
            if let Some(m) = self.try_match_at(text, abs_pos) {
                return Some(m);
            }
            offset += pos + 1;
        }
        None
    }

    /// Find match by scanning for digits
    #[inline]
    fn find_at_digit(&self, text: &str, start: usize) -> Option<Match> {
        let bytes = &text.as_bytes()[start..];
        let mut offset = 0;

        while let Some(pos) = find_digit(&bytes[offset..]) {
            let abs_pos = start + offset + pos;
            if let Some(m) = self.try_match_at(text, abs_pos) {
                return Some(m);
            }
            offset += pos + 1;
        }
        None
    }

    /// Find match by scanning for word characters
    #[inline]
    fn find_at_word_char(&self, text: &str, start: usize) -> Option<Match> {
        let bytes = &text.as_bytes()[start..];
        let unicode_mode = self.flags.contains(Flags::UNICODE);
        let mut offset = 0;

        while offset < bytes.len() {
            // In Unicode mode, also scan for high bytes (could be Unicode word chars)
            let pos = if unicode_mode {
                find_unicode_word_char(&bytes[offset..])
            } else {
                find_word_char(&bytes[offset..])
            };

            match pos {
                Some(p) => {
                    let abs_pos = start + offset + p;
                    if let Some(m) = self.try_match_at(text, abs_pos) {
                        return Some(m);
                    }
                    offset += p + 1;
                }
                None => break,
            }
        }
        None
    }

    /// Find match by scanning for whitespace
    #[inline]
    fn find_at_whitespace(&self, text: &str, start: usize) -> Option<Match> {
        let bytes = &text.as_bytes()[start..];
        let mut offset = 0;

        while let Some(pos) = find_whitespace(&bytes[offset..]) {
            let abs_pos = start + offset + pos;
            if let Some(m) = self.try_match_at(text, abs_pos) {
                return Some(m);
            }
            offset += pos + 1;
        }
        None
    }

    /// Find alternation of literals using Aho-Corasick from a starting position
    #[inline]
    fn find_at_alternation_literals(&self, text: &str, start: usize, literals: &[Vec<u8>], ac: &AhoCorasick) -> Option<Match> {
        let bytes = &text.as_bytes()[start..];
        ac.find(bytes).map(|mat| {
            Match {
                start: start + mat.start(),
                end: start + mat.end(),
            }
        })
    }

    /// Find case-insensitive ASCII literal using fast case-folded comparison
    #[inline]
    fn find_case_insensitive_literal(&self, text: &str, lowercase: &[u8], len: usize) -> Option<Match> {
        self.find_at_case_insensitive_literal(text, 0, lowercase, len)
    }

    /// Find case-insensitive ASCII literal from a starting position
    /// Uses optimized byte-by-byte comparison with ASCII case folding
    #[inline]
    fn find_at_case_insensitive_literal(&self, text: &str, start: usize, lowercase: &[u8], len: usize) -> Option<Match> {
        let bytes = text.as_bytes();
        if bytes.len() < start + len {
            return None;
        }

        // Use the first byte (already lowercase) for memchr scanning
        let first_lower = lowercase[0];
        let first_upper = first_lower.to_ascii_uppercase();

        let mut pos = start;
        while pos + len <= bytes.len() {
            // Find next occurrence of first char (either case)
            let search_bytes = &bytes[pos..];
            let next_pos = if first_lower == first_upper {
                // Non-alphabetic first char
                memchr(first_lower, search_bytes)
            } else {
                memchr2(first_lower, first_upper, search_bytes)
            };

            match next_pos {
                Some(offset) => {
                    let candidate_pos = pos + offset;
                    if candidate_pos + len > bytes.len() {
                        return None;
                    }

                    // Check if the full pattern matches case-insensitively
                    if self.matches_case_insensitive(&bytes[candidate_pos..candidate_pos + len], lowercase) {
                        return Some(Match {
                            start: candidate_pos,
                            end: candidate_pos + len,
                        });
                    }
                    pos = candidate_pos + 1;
                }
                None => return None,
            }
        }
        None
    }

    /// Check if a byte slice matches a lowercase pattern case-insensitively
    #[inline(always)]
    fn matches_case_insensitive(&self, haystack: &[u8], lowercase_pattern: &[u8]) -> bool {
        if haystack.len() != lowercase_pattern.len() {
            return false;
        }
        for (h, p) in haystack.iter().zip(lowercase_pattern.iter()) {
            if h.to_ascii_lowercase() != *p {
                return false;
            }
        }
        true
    }

    /// Find suffix literal from a starting position
    #[inline]
    fn find_at_suffix_literal(&self, text: &str, start: usize, suffix: &[u8]) -> Option<Match> {
        let bytes = &text.as_bytes()[start..];
        let finder = memmem::Finder::new(suffix);

        for pos in finder.find_iter(bytes) {
            let abs_suffix_pos = start + pos;

            // Try matching from positions before the suffix
            let look_back = pos.min(64);
            for back in 0..=look_back {
                let try_pos = start + pos - back;
                if let Some(m) = self.try_match_at(text, try_pos) {
                    if m.end >= abs_suffix_pos + suffix.len() {
                        return Some(m);
                    }
                }
            }
        }
        None
    }

    /// Fallback: Linear scan trying every position from start
    #[inline]
    /// Find using selective prefilter to skip to candidate positions.
    fn find_at_with_selective_prefilter(&self, text: &str, start: usize) -> Option<Match> {
        // Pike VM has its own prefix loop — single call is enough
        if self.use_pike_vm {
            return self.try_match_at(text, start);
        }
        let bytes = text.as_bytes();
        let mut search_from = start;
        while search_from <= text.len() {
            // Find next literal occurrence
            let remaining = &bytes[search_from..];
            let lit_pos = match &self.selective_prefilter {
                selective::Prefilter::MemmemStart(_) | selective::Prefilter::MemmemInner { .. } => {
                    self.memmem_prefilter.as_ref()
                        .and_then(|f| f.find(remaining))
                        .map(|i| search_from + i)
                }
                selective::Prefilter::AhoCorasickStart(_) | selective::Prefilter::AhoCorasickInner { .. } => {
                    self.ac_prefilter.as_ref()
                        .and_then(|ac| ac.find(remaining))
                        .map(|m| search_from + m.start())
                }
                _ => None,
            };

            let lit_pos = match lit_pos {
                Some(p) => p,
                None => break, // No more literal occurrences
            };

            // Calculate where to start trying the match
            let try_pos = match &self.selective_prefilter {
                selective::Prefilter::MemmemInner { min_prefix, .. }
                | selective::Prefilter::AhoCorasickInner { min_prefix, .. } => {
                    lit_pos.saturating_sub(*min_prefix).max(start)
                }
                _ => lit_pos,
            };

            // Try matching at the candidate position
            if let Some(m) = self.try_match_at(text, try_pos) {
                return Some(m);
            }

            // Failed — skip past this literal occurrence to find the next one
            search_from = lit_pos + 1;
        }
        None
    }

    fn find_at_linear(&self, text: &str, start: usize) -> Option<Match> {
        if self.use_pike_vm {
            return self.try_match_at(text, start);
        }
        let mut pos = start;
        while pos <= text.len() {
            if let Some(m) = self.try_match_at(text, pos) {
                return Some(m);
            }
            if pos < text.len() {
                pos += text[pos..].chars().next().map(|c| c.len_utf8()).unwrap_or(1);
            } else {
                break;
            }
        }
        None
    }

    /// Use the selective prefilter to skip to the next candidate position.
    /// Returns `pos` unchanged if no prefilter, or the next position worth trying.
    /// IMPORTANT: the bytecode already has a prefix loop that tries every position.
    /// This prefilter only helps when called from loops that try positions manually
    /// (like captures_at). It must never skip backwards or return a position less
    /// than pos.
    #[inline]
    fn next_candidate(&self, haystack: &[u8], pos: usize) -> usize {
        if pos >= haystack.len() { return pos; }
        let remaining = &haystack[pos..];

        match &self.selective_prefilter {
            selective::Prefilter::None => pos,
            selective::Prefilter::AnchoredStart => {
                if pos == 0 { 0 } else { haystack.len() + 1 }
            }
            selective::Prefilter::SingleByte(b) => {
                memchr(*b, remaining).map(|i| pos + i).unwrap_or(haystack.len() + 1)
            }
            selective::Prefilter::ByteSet(bytes) => {
                match bytes.len() {
                    1 => memchr(bytes[0], remaining)
                        .map(|i| pos + i).unwrap_or(haystack.len() + 1),
                    2 => memchr2(bytes[0], bytes[1], remaining)
                        .map(|i| pos + i).unwrap_or(haystack.len() + 1),
                    3 => memchr3(bytes[0], bytes[1], bytes[2], remaining)
                        .map(|i| pos + i).unwrap_or(haystack.len() + 1),
                    _ => pos,
                }
            }
            selective::Prefilter::MemmemStart(_) => {
                if let Some(ref finder) = self.memmem_prefilter {
                    finder.find(remaining).map(|i| pos + i).unwrap_or(haystack.len() + 1)
                } else {
                    pos
                }
            }
            selective::Prefilter::MemmemInner { min_prefix, .. } => {
                // Inner literal: find it, then back up to where match could start
                if let Some(ref finder) = self.memmem_prefilter {
                    finder.find(remaining).map(|i| {
                        let lit_pos = pos + i;
                        // Back up but never before current pos
                        lit_pos.saturating_sub(*min_prefix).max(pos)
                    }).unwrap_or(haystack.len() + 1)
                } else {
                    pos
                }
            }
            selective::Prefilter::AhoCorasickStart(_) => {
                if let Some(ref ac) = self.ac_prefilter {
                    ac.find(remaining).map(|m| pos + m.start()).unwrap_or(haystack.len() + 1)
                } else {
                    pos
                }
            }
            selective::Prefilter::AhoCorasickInner { min_prefix, .. } => {
                if let Some(ref ac) = self.ac_prefilter {
                    ac.find(remaining).map(|m| {
                        let lit_pos = pos + m.start();
                        lit_pos.saturating_sub(*min_prefix)
                    }).unwrap_or(haystack.len() + 1)
                } else {
                    pos
                }
            }
        }
    }

    /// Find a match using the original C engine (for benchmarking comparison)
    /// Legacy C engine match — only for benchmarking comparisons.
    /// Uses the c2rust-transpiled lre_exec function.
    #[doc(hidden)]
    pub fn find_at_c_engine(&self, text: &str, start: usize) -> Option<Match> {
        let text_bytes = text.as_bytes();
        let capture_count = self.capture_count();
        let mut captures: Vec<*mut u8> = vec![std::ptr::null_mut(); capture_count * 2];
        let ret = engine::lre_exec(
            captures.as_mut_ptr(),
            self.bytecode,
            text_bytes.as_ptr(),
            start as i32,
            text_bytes.len() as i32,
            0,
            std::ptr::null_mut(),
        );
        if ret == 1 {
            let text_start = text_bytes.as_ptr();
            let match_start = if captures[0].is_null() {
                return None;
            } else {
                unsafe { captures[0].offset_from(text_start) as usize }
            };
            let match_end = if captures[1].is_null() {
                return None;
            } else {
                unsafe { captures[1].offset_from(text_start) as usize }
            };
            Some(Match { start: match_start, end: match_end })
        } else {
            None
        }
    }

    /// Bytecode header layout (8 bytes):
    ///   bytes 0-1: flags (u16 LE)
    ///   byte  2:   capture_count (u8)
    ///   byte  3:   register_count (u8)
    ///   bytes 4-7: bytecode body length (u32 LE)
    const HEADER_LEN: usize = 8;

    /// Get total bytecode length (header + body). Pure Rust.
    fn bytecode_len(&self) -> usize {
        let header = unsafe {
            std::slice::from_raw_parts(self.bytecode, Self::HEADER_LEN)
        };
        let body_len = u32::from_le_bytes([header[4], header[5], header[6], header[7]]) as usize;
        Self::HEADER_LEN + body_len
    }

    /// Get bytecode as a slice.
    #[inline]
    fn bytecode_slice(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.bytecode, self.bytecode_len())
        }
    }

    /// Get the number of capture groups (including group 0). Pure Rust.
    pub fn capture_count(&self) -> usize {
        let header = unsafe {
            std::slice::from_raw_parts(self.bytecode, Self::HEADER_LEN)
        };
        header[2] as usize
    }

    /// Get the flags
    pub fn flags(&self) -> Flags {
        self.flags
    }

    /// Get the original pattern
    pub fn pattern(&self) -> &str {
        &self.pattern
    }

    /// Get the search strategy (for debugging/analysis)
    pub fn strategy_name(&self) -> String {
        format!("{:?}", self.strategy)
    }

    /// Create a reusable scratch space for this regex. Allocate once, pass to
    /// `find_at` for zero-allocation matching. Each thread should own its own Scratch.
    pub fn create_scratch(&self) -> pikevm::Scratch {
        let bytecode = self.bytecode_slice();
        let capture_count = bytecode[2] as usize;
        let register_count = bytecode[3] as usize;
        let body_len = u32::from_le_bytes([
            bytecode[4], bytecode[5], bytecode[6], bytecode[7]
        ]) as usize;
        let num_pcs = 8 + body_len + 1;
        pikevm::Scratch::new(num_pcs, capture_count, register_count)
    }

    /// Find the first match starting at or after `start`, using pre-allocated
    /// scratch space. This is the fast path: zero allocation per call.
    /// Create scratch via `regex.create_scratch()`. Each thread needs its own.
    pub fn find_at_scratch(&self, text: &str, start: usize, scratch: &mut pikevm::Scratch) -> Option<Match> {
        // Skip has_match pre-check: the Wide NFA in find_at_linear_scratch
        // already handles rejection. The pre-check scans the entire remaining
        // text and is counterproductive when matches are dense.
        self.find_at_linear_scratch(text, start, scratch)
    }

    /// Core scratch-based linear scan for Pike VM.
    /// Two-pass: DFA O(1)/byte finds match_end, bounded exec finds match_start.
    /// All buffers in Scratch — zero allocation per call. DFA cache warms across calls.
    fn find_at_linear_scratch(&self, text: &str, start: usize, scratch: &mut pikevm::Scratch) -> Option<Match> {
        if self.use_pike_vm {
            if let Some(ref wide_nfa) = self.bit_program {
                // Two-pass: Wide NFA (O(states/64)/byte) + bounded exec
                let bytecode = self.bytecode_slice();
                let text_bytes = text.as_bytes();
                let vm = pikevm::PikeVm::new(bytecode, text_bytes);
                return scratch.find_at(&vm, wide_nfa, start)
                    .map(|(s, e)| Match { start: s, end: e });
            }
            // Fallback: direct exec (for patterns without Wide NFA, e.g. lookahead)
            return self.try_match_at(text, start);
        }
        // Backtracker path
        let mut pos = start;
        while pos <= text.len() {
            if let Some(m) = self.try_match_at(text, pos) {
                return Some(m);
            }
            if pos < text.len() {
                pos += text[pos..].chars().next().map(|c| c.len_utf8()).unwrap_or(1);
            } else {
                break;
            }
        }
        None
    }

    /// Count all non-overlapping matches efficiently.
    ///
    /// This is optimized for counting and uses native Aho-Corasick iteration
    /// for alternation patterns, which is faster than repeated find_at calls.
    pub fn count_matches(&self, text: &str) -> usize {
        match &self.strategy {
            SearchStrategy::AlternationLiterals { ac, .. } => {
                ac.find_iter(text.as_bytes()).count()
            }
            SearchStrategy::PureLiteral(finder) => {
                finder.finder.find_iter(text.as_bytes()).count()
            }
            _ => {
                // Scanner/Verifier: Bit VM fast-forwards, Pike VM verifies
                if let Some(ref prog) = self.bit_program {
                    return self.count_matches_bit_scanner(text, prog);
                }
                // Pike VM with prefilter acceleration
                if self.use_pike_vm {
                    return self.count_matches_pike(text);
                }
                self.find_iter(text).count()
            }
        }
    }

    /// Count matches: Wide NFA find_match_end (fast scan) + bounded exec (correct semantics).
    /// Scratch allocated once, reused across all matches.
    fn count_matches_bit_scanner(&self, text: &str, prog: &bitvm::BitVmProgram) -> usize {
        let text_bytes = text.as_bytes();
        let bytecode = self.bytecode_slice();
        let mut scratch = self.create_scratch();
        let mut count = 0;
        let mut pos = 0;

        while pos <= text_bytes.len() {
            let match_end = match prog.find_match_end(text_bytes, pos) {
                Some(end) => end,
                None => break,
            };
            let bounded_vm = pikevm::PikeVm::new(bytecode, &text_bytes[..match_end]);
            match bounded_vm.exec_with_scratch(&mut scratch, pos) {
                pikevm::PikeResult::Match(caps) => {
                    count += 1;
                    let end = caps.get(1).copied().flatten().unwrap_or(pos + 1);
                    let start = caps.get(0).copied().flatten().unwrap_or(pos);
                    pos = if end > start { end } else { start + 1 };
                }
                pikevm::PikeResult::NoMatch => {
                    pos = match_end; // Wide NFA false positive — skip past
                }
            }
        }

        count
    }

    /// Count matches using Pike VM with prefilter acceleration.
    /// Uses memmem/AC to jump to candidate positions, then Pike VM to verify.
    fn count_matches_pike(&self, text: &str) -> usize {
        let text_bytes = text.as_bytes();
        let bytecode = self.bytecode_slice();

        // If we have a memmem or AC prefilter, use prefilter-accelerated counting
        let has_literal_prefilter = matches!(self.selective_prefilter,
            selective::Prefilter::MemmemStart(_) |
            selective::Prefilter::MemmemInner { .. } |
            selective::Prefilter::AhoCorasickStart(_) |
            selective::Prefilter::AhoCorasickInner { .. }
        );

        if has_literal_prefilter && text_bytes.len() > 1024 {
            return self.count_matches_pike_prefiltered(text);
        }

        // No useful prefilter — use DFA-cached scanner
        let mut scanner = pikevm::PikeScanner::new(bytecode, text_bytes);
        scanner.count_all()
    }

    /// Count matches by jumping to prefilter candidates and running Pike VM nearby.
    fn count_matches_pike_prefiltered(&self, text: &str) -> usize {
        let text_bytes = text.as_bytes();
        let bytecode = unsafe {
            std::slice::from_raw_parts(self.bytecode, self.bytecode_len())
        };
        let mut count = 0;
        let mut search_from = 0;

        while search_from < text_bytes.len() {
            let remaining = &text_bytes[search_from..];

            // Find next literal candidate
            let lit_pos = match &self.selective_prefilter {
                selective::Prefilter::MemmemStart(_) | selective::Prefilter::MemmemInner { .. } => {
                    self.memmem_prefilter.as_ref()
                        .and_then(|f| f.find(remaining))
                        .map(|i| search_from + i)
                }
                selective::Prefilter::AhoCorasickStart(_) | selective::Prefilter::AhoCorasickInner { .. } => {
                    self.ac_prefilter.as_ref()
                        .and_then(|ac| ac.find(remaining))
                        .map(|m| search_from + m.start())
                }
                _ => None,
            };

            let lit_pos = match lit_pos {
                Some(p) => p,
                None => break, // No more candidates
            };

            // Back up by max possible prefix before the literal.
            // min_prefix is the pattern's min_length. The actual prefix before the
            // literal can be larger (due to variable-length parts). Use 200 bytes
            // as a practical maximum — covers most real-world patterns.
            let backup = 200;
            let try_pos = lit_pos.saturating_sub(backup);

            // Window: backup before literal + 300 bytes after (for suffix after literal)
            let window_start = try_pos;
            let window_end = (lit_pos + 300).min(text_bytes.len());
            let window = &text_bytes[window_start..window_end];
            let vm = pikevm::PikeVm::new(bytecode, window);
            match vm.exec(0) {
                pikevm::PikeResult::Match(caps) => {
                    count += 1;
                    // Translate window-relative end back to absolute
                    let rel_end = caps.get(1).copied().flatten().unwrap_or(1);
                    let abs_end = window_start + rel_end;
                    search_from = if abs_end > try_pos { abs_end } else { lit_pos + 1 };
                }
                pikevm::PikeResult::NoMatch => {
                    search_from = lit_pos + 1;
                }
            }
        }

        count
    }

    /// Find all non-overlapping matches.
    ///
    /// # Example
    ///
    /// ```
    /// use quickjs_regex::Regex;
    ///
    /// let re = Regex::new(r"\d+").unwrap();
    /// let text = "a1b22c333";
    /// let matches: Vec<_> = re.find_iter(text).collect();
    /// assert_eq!(matches.len(), 3);
    /// ```
    pub fn find_iter<'r, 't>(&'r self, text: &'t str) -> MatchIterator<'r, 't> {
        // Use specialized iterator for patterns we can handle super fast
        match &self.strategy {
            SearchStrategy::PureLiteral(finder) => {
                MatchIterator::Literal(LiteralMatches::new(finder.needle(), text))
            }
            SearchStrategy::AlternationLiterals { literals, ac } => {
                MatchIterator::Alternation(AlternationMatches {
                    ac,
                    literals,
                    text,
                    pos: 0,
                })
            }
            // Pure fast-path iterators
            SearchStrategy::PureDigitPlus => {
                MatchIterator::PureDigit(PureDigitMatches { text, pos: 0 })
            }
            SearchStrategy::PureWordPlus => {
                MatchIterator::PureWord(PureWordMatches { text, pos: 0 })
            }
            SearchStrategy::QuotedString(quote) => {
                MatchIterator::QuotedStr(QuotedStringMatches { text, pos: 0, quote: *quote })
            }
            SearchStrategy::PureCapitalWord => {
                MatchIterator::CapitalWord(CapitalWordMatches { text, pos: 0 })
            }
            SearchStrategy::PureLowerSuffix(suffix) => {
                MatchIterator::LowerSuffix(LowerSuffixMatches { text, pos: 0, suffix: suffix.clone() })
            }
            _ => {
                // Use Pike VM iterator for patterns routed to Pike VM
                if self.use_pike_vm {
                    let bytecode = unsafe {
                        std::slice::from_raw_parts(self.bytecode, self.bytecode_len())
                    };
                    let scanner = pikevm::PikeScanner::new(bytecode, text.as_bytes());
                    MatchIterator::PikeVm(PikeVmMatches {
                        scanner,
                        regex: self,
                        text,
                        pos: 0,
                        last_was_empty: false,
                    })
                } else {
                    MatchIterator::General(Matches {
                        regex: self,
                        text,
                        last_end: 0,
                        last_was_empty: false,
                    })
                }
            }
        }
    }

    /// Get capture groups from the first match.
    ///
    /// Returns `None` if there is no match. Returns `Some(Captures)` with
    /// all capture groups on success. Group 0 is the entire match.
    ///
    /// # Example
    ///
    /// ```
    /// use quickjs_regex::Regex;
    ///
    /// let re = Regex::new(r"(\w+)@(\w+)\.(\w+)").unwrap();
    /// let caps = re.captures("user@example.com").unwrap();
    /// assert_eq!(caps.get_str(0), Some("user@example.com"));
    /// assert_eq!(caps.get_str(1), Some("user"));
    /// assert_eq!(caps.get_str(2), Some("example"));
    /// assert_eq!(caps.get_str(3), Some("com"));
    /// ```
    pub fn captures(&self, text: &str) -> Option<Captures> {
        self.captures_at(text, 0)
    }

    /// Get capture groups from a match starting at the given byte offset.
    /// Get capture groups at a byte offset (pure Rust, with selective prefiltering)
    pub fn captures_at(&self, text: &str, start: usize) -> Option<Captures> {
        let text_bytes = text.as_bytes();
        let capture_count = self.capture_count();

        // Bit VM fast rejection
        if let Some(ref prog) = self.bit_program {
            if !prog.has_match(&text_bytes[start..]) {
                return None;
            }
        }

        let bytecode = unsafe {
            std::slice::from_raw_parts(self.bytecode, self.bytecode_len())
        };

        // Use Pike VM for captures when available (linear time, correct semantics)
        if self.use_pike_vm {
            let vm = pikevm::PikeVm::new(bytecode, text_bytes);
            return match vm.exec(start) {
                pikevm::PikeResult::Match(caps) => {
                    let mut groups = Vec::with_capacity(capture_count);
                    for i in 0..capture_count {
                        let s = caps.get(i * 2).copied().flatten();
                        let e = caps.get(i * 2 + 1).copied().flatten();
                        match (s, e) {
                            (Some(s), Some(e)) => groups.push(Some((s, e))),
                            _ => groups.push(None),
                        }
                    }
                    Some(Captures {
                        text: text.to_string(),
                        groups,
                    })
                }
                pikevm::PikeResult::NoMatch => None,
            };
        }

        // Fallback: backtracking interpreter (for patterns with backreferences)
        let mut ctx = interpreter::ExecContext::new(bytecode, text_bytes);

        let mut pos = start;
        while pos <= text.len() {
            // Skip to next candidate position using prefilter
            pos = self.next_candidate(text_bytes, pos);
            if pos > text.len() { break; }

            match ctx.exec(pos) {
                interpreter::ExecResult::Match => {
                    let mut groups = Vec::with_capacity(capture_count);
                    for i in 0..capture_count {
                        let cap_start = ctx.captures.get(i * 2).copied().flatten();
                        let cap_end = ctx.captures.get(i * 2 + 1).copied().flatten();
                        match (cap_start, cap_end) {
                            (Some(s), Some(e)) => groups.push(Some((s, e))),
                            _ => groups.push(None),
                        }
                    }
                    return Some(Captures {
                        text: text.to_string(),
                        groups,
                    });
                }
                interpreter::ExecResult::NoMatch => {
                    if pos < text.len() {
                        pos += text[pos..].chars().next().map(|c| c.len_utf8()).unwrap_or(1);
                    } else {
                        break;
                    }
                    ctx.reset();
                }
            }
        }
        None
    }

    /// Alias for captures_at — kept for API compatibility
    #[doc(hidden)]
    pub fn captures_at_pure_rust(&self, text: &str, start: usize) -> Option<Captures> {
        self.captures_at(text, start)
    }

    /// Get captures using pre-allocated scratch. Uses Wide NFA to find match
    /// bounds first, then bounded exec for capture extraction.
    pub fn captures_at_scratch(&self, text: &str, start: usize, scratch: &mut pikevm::Scratch) -> Option<Captures> {
        let text_bytes = text.as_bytes();
        let capture_count = self.capture_count();
        let bytecode = self.bytecode_slice();

        if self.use_pike_vm {
            // Use Wide NFA to find match_end, then bounded exec for captures
            if let Some(ref wide_nfa) = self.bit_program {
                let match_end = wide_nfa.find_match_end(text_bytes, start)?;
                let bounded_vm = pikevm::PikeVm::new(bytecode, &text_bytes[..match_end]);
                return match bounded_vm.exec_with_scratch(scratch, start) {
                    pikevm::PikeResult::Match(caps) => {
                        let mut groups = Vec::with_capacity(capture_count);
                        for i in 0..capture_count {
                            let s = caps.get(i * 2).copied().flatten();
                            let e = caps.get(i * 2 + 1).copied().flatten();
                            match (s, e) {
                                (Some(s), Some(e)) => groups.push(Some((s, e))),
                                _ => groups.push(None),
                            }
                        }
                        Some(Captures { text: text.to_string(), groups })
                    }
                    pikevm::PikeResult::NoMatch => {
                        // Bounded exec disagrees — fall back to full exec
                        let vm = pikevm::PikeVm::new(bytecode, text_bytes);
                        match vm.exec_with_scratch(scratch, start) {
                            pikevm::PikeResult::Match(caps) => {
                                let mut groups = Vec::with_capacity(capture_count);
                                for i in 0..capture_count {
                                    let s = caps.get(i * 2).copied().flatten();
                                    let e = caps.get(i * 2 + 1).copied().flatten();
                                    match (s, e) {
                                        (Some(s), Some(e)) => groups.push(Some((s, e))),
                                        _ => groups.push(None),
                                    }
                                }
                                Some(Captures { text: text.to_string(), groups })
                            }
                            pikevm::PikeResult::NoMatch => None,
                        }
                    }
                };
            }
            // No Wide NFA — full exec with scratch
            let vm = pikevm::PikeVm::new(bytecode, text_bytes);
            return match vm.exec_with_scratch(scratch, start) {
                pikevm::PikeResult::Match(caps) => {
                    let mut groups = Vec::with_capacity(capture_count);
                    for i in 0..capture_count {
                        let s = caps.get(i * 2).copied().flatten();
                        let e = caps.get(i * 2 + 1).copied().flatten();
                        match (s, e) {
                            (Some(s), Some(e)) => groups.push(Some((s, e))),
                            _ => groups.push(None),
                        }
                    }
                    Some(Captures { text: text.to_string(), groups })
                }
                pikevm::PikeResult::NoMatch => None,
            };
        }
        self.captures_at(text, start)
    }
}

impl Drop for Regex {
    fn drop(&mut self) {
        if self.owned_bytecode.is_some() {
            // Bytecode is owned by the Vec; Rust will free it when the Vec drops.
            return;
        }
        if !self.bytecode.is_null() {
            // SAFETY: bytecode was allocated by lre_compile via lre_realloc (which uses libc::malloc).
            unsafe {
                libc::free(self.bytecode as *mut std::ffi::c_void);
            }
        }
    }
}

impl std::fmt::Debug for Regex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Regex")
            .field("pattern", &self.pattern)
            .field("flags", &self.flags)
            .finish()
    }
}

impl std::fmt::Display for Regex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "/{}/{}", self.pattern, self.flags)
    }
}

/// A match result with start and end byte positions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Match {
    /// Start byte offset (inclusive)
    pub start: usize,
    /// End byte offset (exclusive)
    pub end: usize,
}

impl Match {
    /// Get the matched substring from the original text
    pub fn as_str<'a>(&self, text: &'a str) -> &'a str {
        &text[self.start..self.end]
    }

    /// Get the length of the match in bytes
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Check if the match is empty
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

/// Captured groups from a regex match.
///
/// Group 0 is always the entire match. Groups 1+ are the explicit
/// capture groups in the pattern.
#[derive(Debug, Clone)]
pub struct Captures {
    /// The original text
    text: String,
    /// Pairs of (start, end) byte offsets for each group
    /// None means the group didn't participate in the match
    groups: Vec<Option<(usize, usize)>>,
}

impl Captures {
    /// Get the number of capture groups (including group 0).
    pub fn len(&self) -> usize {
        self.groups.len()
    }

    /// Check if there are no captures (should never be true for a valid match).
    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }

    /// Get a specific capture group by index.
    ///
    /// Group 0 is the entire match. Returns `None` if the group
    /// index is out of bounds or if the group didn't participate
    /// in the match.
    pub fn get(&self, i: usize) -> Option<Match> {
        self.groups.get(i).and_then(|opt| {
            opt.map(|(start, end)| Match { start, end })
        })
    }

    /// Get the text of a specific capture group.
    pub fn get_str(&self, i: usize) -> Option<&str> {
        self.get(i).map(|m| &self.text[m.start..m.end])
    }

    /// Get the entire match (group 0).
    pub fn entire_match(&self) -> Option<Match> {
        self.get(0)
    }

    /// Iterate over all capture groups.
    pub fn iter(&self) -> impl Iterator<Item = Option<Match>> + '_ {
        self.groups.iter().map(|opt| {
            opt.map(|(start, end)| Match { start, end })
        })
    }

    /// Count the number of capture groups that actually matched (non-None).
    /// This is different from `len()` which returns the total number of groups.
    pub fn count_matched(&self) -> usize {
        self.groups.iter().filter(|g| g.is_some()).count()
    }
}

/// Match iterator enum - dispatches between literal and general matching
pub enum MatchIterator<'r, 't> {
    Literal(LiteralMatches<'r, 't>),
    Alternation(AlternationMatches<'r, 't>),
    PureDigit(PureDigitMatches<'t>),
    PureWord(PureWordMatches<'t>),
    QuotedStr(QuotedStringMatches<'t>),
    CapitalWord(CapitalWordMatches<'t>),
    LowerSuffix(LowerSuffixMatches<'t>),
    /// Pike VM iterator with persistent DFA cache
    PikeVm(PikeVmMatches<'r, 't>),
    General(Matches<'r, 't>),
}

/// Match iterator backed by a Pike VM scanner with persistent DFA cache.
pub struct PikeVmMatches<'r, 't> {
    scanner: pikevm::PikeScanner<'t>,
    regex: &'r Regex,
    text: &'t str,
    pos: usize,
    last_was_empty: bool,
}

impl<'r, 't> Iterator for PikeVmMatches<'r, 't> {
    type Item = Match;

    fn next(&mut self) -> Option<Match> {
        if self.pos > self.text.len() { return None; }

        let search_start = if self.last_was_empty {
            let mut next = self.pos;
            if next < self.text.len() {
                next += self.text[next..].chars().next().map(|c| c.len_utf8()).unwrap_or(1);
            } else {
                return None;
            }
            next
        } else {
            self.pos
        };

        match self.scanner.find_next(search_start) {
            Some((start, end)) => {
                self.last_was_empty = start == end;
                self.pos = end;
                Some(Match { start, end })
            }
            None => None,
        }
    }
}

impl<'r, 't> Iterator for MatchIterator<'r, 't> {
    type Item = Match;

    fn next(&mut self) -> Option<Match> {
        match self {
            MatchIterator::Literal(lit) => lit.next(),
            MatchIterator::Alternation(alt) => alt.next(),
            MatchIterator::PureDigit(d) => d.next(),
            MatchIterator::PureWord(w) => w.next(),
            MatchIterator::QuotedStr(q) => q.next(),
            MatchIterator::CapitalWord(c) => c.next(),
            MatchIterator::LowerSuffix(s) => s.next(),
            MatchIterator::PikeVm(pike) => pike.next(),
            MatchIterator::General(gen) => gen.next(),
        }
    }
}

/// Fast iterator for pure literal patterns
/// Uses memchr for single bytes, memmem for longer patterns
pub enum LiteralMatches<'r, 't> {
    /// Single byte - use memchr (fastest for single chars)
    Single {
        bytes: &'t [u8],
        pos: usize,
        byte: u8,
    },
    /// Multi-byte patterns - use memmem FindIter (SIMD-accelerated)
    Multi {
        inner: memmem::FindIter<'t, 'r>,
        literal_len: usize,
    },
}

impl<'r, 't> LiteralMatches<'r, 't> {
    /// Create a new literal match iterator
    fn new(literal: &'r [u8], text: &'t str) -> Self {
        let bytes = text.as_bytes();
        if literal.len() == 1 {
            LiteralMatches::Single {
                bytes,
                pos: 0,
                byte: literal[0],
            }
        } else {
            // memmem is highly optimized for multi-byte patterns
            LiteralMatches::Multi {
                inner: memmem::find_iter(bytes, literal),
                literal_len: literal.len(),
            }
        }
    }
}

impl<'r, 't> Iterator for LiteralMatches<'r, 't> {
    type Item = Match;

    #[inline]
    fn next(&mut self) -> Option<Match> {
        match self {
            LiteralMatches::Single { bytes, pos, byte } => {
                if *pos >= bytes.len() {
                    return None;
                }
                let found = memchr(*byte, &bytes[*pos..])?;
                let start = *pos + found;
                *pos = start + 1;
                Some(Match { start, end: start + 1 })
            }
            LiteralMatches::Multi { inner, literal_len } => {
                inner.next().map(|start| Match {
                    start,
                    end: start + *literal_len,
                })
            }
        }
    }
}

/// Fast iterator for alternation of literals using Aho-Corasick
pub struct AlternationMatches<'r, 't> {
    ac: &'r AhoCorasick,
    literals: &'r [Vec<u8>],
    text: &'t str,
    pos: usize,
}

impl<'r, 't> Iterator for AlternationMatches<'r, 't> {
    type Item = Match;

    fn next(&mut self) -> Option<Match> {
        if self.pos >= self.text.len() {
            return None;
        }

        let bytes = &self.text.as_bytes()[self.pos..];
        if let Some(mat) = self.ac.find(bytes) {
            let start = self.pos + mat.start();
            let end = self.pos + mat.end();
            self.pos = end; // Non-overlapping
            Some(Match { start, end })
        } else {
            self.pos = self.text.len();
            None
        }
    }
}

/// Fast iterator for pure digit patterns [0-9]+ - NO INTERPRETER!
pub struct PureDigitMatches<'t> {
    text: &'t str,
    pos: usize,
}

impl<'t> Iterator for PureDigitMatches<'t> {
    type Item = Match;

    fn next(&mut self) -> Option<Match> {
        let bytes = self.text.as_bytes();
        let m = find_digit_run(bytes, self.pos)?;
        self.pos = m.end;
        Some(m)
    }
}

/// Fast iterator for pure word char patterns \w+ - NO INTERPRETER!
pub struct PureWordMatches<'t> {
    text: &'t str,
    pos: usize,
}

impl<'t> Iterator for PureWordMatches<'t> {
    type Item = Match;

    fn next(&mut self) -> Option<Match> {
        let bytes = self.text.as_bytes();
        let m = find_word_run(bytes, self.pos)?;
        self.pos = m.end;
        Some(m)
    }
}

/// Fast iterator for quoted string patterns "[^"]*" - NO INTERPRETER!
pub struct QuotedStringMatches<'t> {
    text: &'t str,
    pos: usize,
    quote: u8,
}

impl<'t> Iterator for QuotedStringMatches<'t> {
    type Item = Match;

    fn next(&mut self) -> Option<Match> {
        let bytes = self.text.as_bytes();
        let m = find_quoted_string(bytes, self.pos, self.quote)?;
        self.pos = m.end;
        Some(m)
    }
}

/// Fast iterator for capital word patterns [A-Z][a-z]+ - NO INTERPRETER!
pub struct CapitalWordMatches<'t> {
    text: &'t str,
    pos: usize,
}

impl<'t> Iterator for CapitalWordMatches<'t> {
    type Item = Match;

    fn next(&mut self) -> Option<Match> {
        let bytes = self.text.as_bytes();
        let m = find_capital_word(bytes, self.pos)?;
        self.pos = m.end;
        Some(m)
    }
}

/// Fast iterator for lowercase+suffix patterns [a-z]+ing - NO INTERPRETER!
pub struct LowerSuffixMatches<'t> {
    text: &'t str,
    pos: usize,
    suffix: Vec<u8>,
}

impl<'t> Iterator for LowerSuffixMatches<'t> {
    type Item = Match;

    fn next(&mut self) -> Option<Match> {
        let bytes = self.text.as_bytes();
        let m = find_lower_suffix(bytes, self.pos, &self.suffix)?;
        self.pos = m.end;
        Some(m)
    }
}

/// An iterator over all non-overlapping matches in a string.
pub struct Matches<'r, 't> {
    regex: &'r Regex,
    text: &'t str,
    last_end: usize,
    /// Track if last match was empty to avoid infinite loops
    last_was_empty: bool,
}

impl<'r, 't> Iterator for Matches<'r, 't> {
    type Item = Match;

    fn next(&mut self) -> Option<Match> {
        if self.last_end > self.text.len() {
            return None;
        }

        let search_start = if self.last_was_empty {
            // After an empty match, advance by one character to avoid infinite loop
            let mut next = self.last_end;
            if next < self.text.len() {
                // Advance by one UTF-8 character
                next += self.text[next..].chars().next().map(|c| c.len_utf8()).unwrap_or(1);
            } else {
                return None;
            }
            next
        } else {
            self.last_end
        };

        match self.regex.find_at(self.text, search_start) {
            Some(m) => {
                self.last_was_empty = m.is_empty();
                self.last_end = m.end;
                Some(m)
            }
            None => None,
        }
    }
}

// ============================================================================
// Pattern Analysis - determines optimal search strategy
// ============================================================================

/// Detect patterns that can be handled with pure fast paths (no interpreter!)
/// These patterns are common and can be matched much faster with specialized code.
fn detect_pure_pattern(pattern: &str) -> Option<SearchStrategy> {
    // Check for \d+ or [0-9]+
    if pattern == r"\d+" || pattern == "[0-9]+" {
        return Some(SearchStrategy::PureDigitPlus);
    }

    // Check for \w+ or [a-zA-Z0-9_]+
    if pattern == r"\w+" || pattern == "[a-zA-Z0-9_]+" || pattern == "[_a-zA-Z0-9]+" {
        return Some(SearchStrategy::PureWordPlus);
    }

    // Check for [a-z]+
    if pattern == "[a-z]+" {
        return Some(SearchStrategy::PureLowerPlus);
    }

    // Check for [A-Z]+
    if pattern == "[A-Z]+" {
        return Some(SearchStrategy::PureUpperPlus);
    }

    // Check for [a-zA-Z]+ or [A-Za-z]+
    if pattern == "[a-zA-Z]+" || pattern == "[A-Za-z]+" {
        return Some(SearchStrategy::PureAlphaPlus);
    }

    // Check for [a-zA-Z0-9]+ or similar
    if pattern == "[a-zA-Z0-9]+" || pattern == "[0-9a-zA-Z]+" ||
       pattern == "[A-Za-z0-9]+" || pattern == "[0-9A-Za-z]+" {
        return Some(SearchStrategy::PureAlnumPlus);
    }

    // Check for quoted string patterns: "[^"]*" or '[^']*'
    if pattern == r#""[^"]*""# {
        return Some(SearchStrategy::QuotedString(b'"'));
    }
    if pattern == r"'[^']*'" {
        return Some(SearchStrategy::QuotedString(b'\''));
    }

    // Check for [A-Z][a-z]+ (capital word)
    if pattern == "[A-Z][a-z]+" {
        return Some(SearchStrategy::PureCapitalWord);
    }

    // Check for [a-z]+suffix patterns (e.g., [a-z]+ing)
    // Must find lowercase runs that END with the suffix
    if pattern.starts_with("[a-z]+") && pattern.len() > 6 {
        let suffix = &pattern[6..];
        // Verify suffix is pure ASCII lowercase literal
        if suffix.bytes().all(|b| b.is_ascii_lowercase()) {
            return Some(SearchStrategy::PureLowerSuffix(suffix.as_bytes().to_vec()));
        }
    }

    None
}

/// Extract inline flags from a pattern like (?i)pattern or (?ims)pattern
///
/// Supports Perl-style inline flags at the start of the pattern:
/// - (?i) - case insensitive
/// - (?m) - multiline
/// - (?s) - dot matches newline
/// - (?u) - unicode
/// - (?imsu) - multiple flags
///
/// Returns the pattern without the inline flag prefix and the extracted flags.
fn extract_inline_flags(pattern: &str) -> (String, Flags) {
    let mut flags = Flags::empty();
    let mut pos = 0;
    let bytes = pattern.as_bytes();

    // Keep extracting inline flags from the start of the pattern
    while pos + 2 < bytes.len() && bytes[pos] == b'(' && bytes[pos + 1] == b'?' {
        let mut i = pos + 2; // Skip "(?"
        let mut local_flags = Flags::empty();
        let mut has_flags = false;
        let mut found_end = false;

        // Parse flag characters
        while i < bytes.len() {
            match bytes[i] {
                b'i' => {
                    local_flags.insert(Flags::IGNORE_CASE);
                    has_flags = true;
                    i += 1;
                }
                b'm' => {
                    local_flags.insert(Flags::MULTILINE);
                    has_flags = true;
                    i += 1;
                }
                b's' => {
                    local_flags.insert(Flags::DOT_ALL);
                    has_flags = true;
                    i += 1;
                }
                b'u' => {
                    local_flags.insert(Flags::UNICODE);
                    has_flags = true;
                    i += 1;
                }
                b'x' => {
                    // Extended mode - just consume it (not supported but don't error)
                    has_flags = true;
                    i += 1;
                }
                b')' => {
                    // End of flag-only group like (?i)
                    if has_flags {
                        found_end = true;
                        i += 1; // Move past ')'
                    }
                    break;
                }
                _ => {
                    // Not a flag character - this is a different kind of group
                    // e.g., (?:...), (?=...), (?!...), etc.
                    break;
                }
            }
        }

        if found_end {
            // Successfully parsed a flag group, merge flags and advance position
            flags.insert(local_flags.bits());
            pos = i;
        } else {
            // Not a pure flag group, stop parsing
            break;
        }
    }

    // Return the remaining pattern after all flag groups
    (pattern[pos..].to_string(), flags)
}

/// Analyze a pattern to determine the best search strategy.
fn analyze_pattern(pattern: &str, flags: Flags) -> SearchStrategy {
    // First, check for pure fast-path patterns (no interpreter needed!)
    if !flags.is_ignore_case() {
        if let Some(strategy) = detect_pure_pattern(pattern) {
            return strategy;
        }
    }

    let case_insensitive = flags.is_ignore_case();

    // Check if entire pattern is wrapped in a capturing group: (alt1|alt2|...)
    // This is common for patterns like "(january|february|...)"
    if pattern.starts_with('(') && pattern.ends_with(')') && !pattern.starts_with("(?") {
        // Check if the parentheses are balanced (not nested groups)
        let inner = &pattern[1..pattern.len()-1];
        let mut depth = 0;
        let mut has_alternation = false;
        for c in inner.chars() {
            match c {
                '(' => depth += 1,
                ')' => depth -= 1,
                '|' if depth == 0 => has_alternation = true,
                _ => {}
            }
            if depth < 0 { break; } // Unbalanced
        }
        // If it's a simple group with top-level alternation, analyze the inner pattern
        if depth == 0 && has_alternation {
            let inner_strategy = analyze_alternation(inner, case_insensitive);
            // Always return - even if None, don't fall through to extract a partial prefix
            // from just one branch of the alternation
            return inner_strategy;
        }
    }

    let mut chars = pattern.chars().peekable();

    // Check for start anchor
    if chars.peek() == Some(&'^') {
        chars.next(); // consume '^'
        // Check if the rest is a pure literal
        let anchored_literal = analyze_anchored_remainder(&mut chars, flags);
        return anchored_literal;
    }

    // Try to extract leading literal(s) or character class
    let mut literals = Vec::new();
    let mut is_pure_literal = true; // Track if we consumed the entire pattern
    let mut depth = 0; // Track group nesting depth

    while let Some(c) = chars.next() {
        match c {
            // Metacharacters - use accumulated literals if any
            '.' => {
                // Dot matches anything - break and use what we have
                is_pure_literal = false;
                if literals.is_empty() {
                    return SearchStrategy::None;
                }
                break;
            }

            '*' | '+' | '?' => {
                // Quantifier - use what we have (minus the quantified char)
                // If the previous char is quantified, it's not a reliable prefix
                is_pure_literal = false;
                if !literals.is_empty() {
                    literals.pop(); // Remove the quantified character
                }
                break;
            }

            '[' => {
                // Character class - try to extract specific bytes
                is_pure_literal = false;
                if literals.is_empty() {
                    return analyze_char_class(&mut chars, case_insensitive);
                }
                break;
            }

            '(' => {
                is_pure_literal = false;
                depth += 1;
                // Group - check for non-capturing or special
                if chars.peek() == Some(&'?') {
                    chars.next();
                    match chars.next() {
                        Some(':') => continue, // Non-capturing, continue
                        Some('=') | Some('!') => {
                            // Lookahead - use what we have
                            break;
                        }
                        _ => break,
                    }
                }
                // Capturing group - continue parsing inside
                continue;
            }

            ')' => {
                is_pure_literal = false;
                if depth > 0 {
                    depth -= 1;
                    continue; // Continue parsing after closing group
                }
                break;
            }

            '{' | '}' | '$' => {
                is_pure_literal = false;
                break;
            }

            '|' => {
                is_pure_literal = false;
                if depth == 0 {
                    // Alternation at top level - analyze all branches
                    return analyze_alternation(pattern, case_insensitive);
                }
                // Alternation inside a group - we can't safely extract a prefix
                // Fall through and let the end check handle it
                continue;
            }

            '\\' => {
                // Escape sequence
                match chars.next() {
                    // Character classes
                    Some('d') | Some('D') => {
                        is_pure_literal = false;
                        if literals.is_empty() {
                            return SearchStrategy::Digit;
                        }
                        break;
                    }
                    Some('w') | Some('W') => {
                        is_pure_literal = false;
                        if literals.is_empty() {
                            return SearchStrategy::WordChar;
                        }
                        break;
                    }
                    Some('s') | Some('S') => {
                        is_pure_literal = false;
                        if literals.is_empty() {
                            return SearchStrategy::Whitespace;
                        }
                        break;
                    }
                    Some('b') | Some('B') => {
                        // Word boundary - not a pure literal anymore
                        is_pure_literal = false;
                        continue;
                    }
                    // Literal escapes
                    Some('\\') => literals.push(b'\\'),
                    Some('/') => literals.push(b'/'),
                    Some('n') => literals.push(b'\n'),
                    Some('r') => literals.push(b'\r'),
                    Some('t') => literals.push(b'\t'),
                    Some('.') => literals.push(b'.'),
                    Some('*') => literals.push(b'*'),
                    Some('+') => literals.push(b'+'),
                    Some('?') => literals.push(b'?'),
                    Some('[') => literals.push(b'['),
                    Some(']') => literals.push(b']'),
                    Some('(') => literals.push(b'('),
                    Some(')') => literals.push(b')'),
                    Some('{') => literals.push(b'{'),
                    Some('}') => literals.push(b'}'),
                    Some('|') => literals.push(b'|'),
                    Some('^') => literals.push(b'^'),
                    Some('$') => literals.push(b'$'),
                    _ => {
                        is_pure_literal = false;
                        break;
                    }
                }
            }

            // Regular ASCII character
            _ if c.is_ascii() => {
                if case_insensitive && c.is_ascii_alphabetic() {
                    // For case-insensitive, accumulate lowercase version
                    is_pure_literal = false;
                    literals.push(c.to_ascii_lowercase() as u8);
                } else {
                    literals.push(c as u8);
                }
            }

            // Non-ASCII character - encode as UTF-8 bytes
            _ => {
                if case_insensitive {
                    // Unicode case-insensitive is complex, bail out
                    is_pure_literal = false;
                    break;
                }
                // Push UTF-8 bytes for this character
                let mut buf = [0u8; 4];
                let encoded = c.encode_utf8(&mut buf);
                literals.extend_from_slice(encoded.as_bytes());
            }
        }
    }

    // For case-insensitive full literal patterns, use optimized search
    if case_insensitive && !is_pure_literal && literals.len() == pattern.len() && literals.len() >= 2 {
        // The entire pattern is a literal (all chars were pushed)
        return SearchStrategy::CaseInsensitiveLiteral {
            lowercase: literals,
            len: pattern.len(),
        };
    }

    // IMPORTANT: If we only parsed a partial prefix AND the pattern contains alternation,
    // the prefix may only represent one branch. Using it for scanning would miss other branches.
    // Fall back to None (linear scan) in this case for correctness.
    if !is_pure_literal && pattern_has_alternation(pattern) {
        return SearchStrategy::None;
    }

    // Convert literals to appropriate strategy
    match literals.len() {
        0 => {
            // No prefix - try to find a suffix literal
            if let Some(suffix) = extract_suffix_literal(pattern) {
                if suffix.len() >= 2 {
                    return SearchStrategy::SuffixLiteral(suffix);
                }
            }
            SearchStrategy::None
        }
        1 if is_pure_literal => SearchStrategy::PureLiteral(OwnedFinder::new(literals)),
        1 if case_insensitive => {
            // Single byte prefix for case-insensitive - need both cases
            let lower = literals[0].to_ascii_lowercase();
            let upper = literals[0].to_ascii_uppercase();
            if lower != upper {
                SearchStrategy::TwoBytes(lower, upper)
            } else {
                SearchStrategy::SingleByte(literals[0])
            }
        }
        1 => SearchStrategy::SingleByte(literals[0]),
        _ if is_pure_literal => SearchStrategy::PureLiteral(OwnedFinder::new(literals)),
        _ if case_insensitive => {
            // For case-insensitive with multi-byte prefix, we can't use memmem directly
            // because the prefix is lowercased but the text may have mixed case.
            // Use TwoBytes strategy with both cases of the first letter instead.
            let first = literals[0];
            let lower = first.to_ascii_lowercase();
            let upper = first.to_ascii_uppercase();
            if lower != upper {
                SearchStrategy::TwoBytes(lower, upper)
            } else {
                SearchStrategy::SingleByte(first)
            }
        }
        _ => SearchStrategy::LiteralPrefix(OwnedFinder::new(literals)),
    }
}

/// Extract a literal suffix from the end of a pattern
/// For example, "[a-z]+ing" -> Some("ing")
fn extract_suffix_literal(pattern: &str) -> Option<Vec<u8>> {
    let mut suffix = Vec::new();
    let mut chars: Vec<char> = pattern.chars().collect();

    // Work backwards from the end
    while let Some(c) = chars.pop() {
        match c {
            // End anchor is OK, we can still have a suffix
            '$' => continue,

            // Metacharacters - stop here
            '.' | '*' | '+' | '?' | ']' | ')' | '}' | '|' | '^' => {
                break;
            }

            // Escaped character - check what it is
            _ if !chars.is_empty() && chars.last() == Some(&'\\') => {
                chars.pop(); // consume the backslash
                match c {
                    // These escapes are literal characters
                    '\\' | '/' | 'n' | 'r' | 't' | '.' | '*' | '+' | '?' |
                    '[' | ']' | '(' | ')' | '{' | '}' | '|' | '^' | '$' => {
                        let byte = match c {
                            'n' => b'\n',
                            'r' => b'\r',
                            't' => b'\t',
                            _ => c as u8,
                        };
                        suffix.push(byte);
                    }
                    // Character classes - stop
                    'd' | 'D' | 'w' | 'W' | 's' | 'S' | 'b' | 'B' => {
                        break;
                    }
                    _ => break,
                }
            }

            // Regular ASCII character
            _ if c.is_ascii() => {
                suffix.push(c as u8);
            }

            _ => break,
        }
    }

    // Reverse since we collected backwards
    suffix.reverse();

    if suffix.is_empty() {
        None
    } else {
        Some(suffix)
    }
}

/// Analyze the remainder of a pattern after ^ to see if it's a pure literal
fn analyze_anchored_remainder(chars: &mut std::iter::Peekable<std::str::Chars>, flags: Flags) -> SearchStrategy {
    if flags.is_ignore_case() {
        // Case-insensitive anchored patterns need full engine
        return SearchStrategy::Anchored;
    }

    let mut literals = Vec::new();

    while let Some(c) = chars.next() {
        match c {
            // Any metacharacter means it's not a pure literal
            '.' | '*' | '+' | '?' | '[' | '(' | ')' | '{' | '}' | '$' | '|' => {
                return SearchStrategy::Anchored;
            }
            '\\' => {
                // Check for literal escapes
                match chars.next() {
                    Some('\\') => literals.push(b'\\'),
                    Some('/') => literals.push(b'/'),
                    Some('n') => literals.push(b'\n'),
                    Some('r') => literals.push(b'\r'),
                    Some('t') => literals.push(b'\t'),
                    Some('.') => literals.push(b'.'),
                    Some('*') => literals.push(b'*'),
                    Some('+') => literals.push(b'+'),
                    Some('?') => literals.push(b'?'),
                    Some('[') => literals.push(b'['),
                    Some(']') => literals.push(b']'),
                    Some('(') => literals.push(b'('),
                    Some(')') => literals.push(b')'),
                    Some('{') => literals.push(b'{'),
                    Some('}') => literals.push(b'}'),
                    Some('|') => literals.push(b'|'),
                    Some('^') => literals.push(b'^'),
                    Some('$') => literals.push(b'$'),
                    _ => return SearchStrategy::Anchored,
                }
            }
            _ if c.is_ascii() => {
                literals.push(c as u8);
            }
            _ => return SearchStrategy::Anchored,
        }
    }

    if literals.is_empty() {
        SearchStrategy::Anchored
    } else {
        SearchStrategy::AnchoredLiteral(literals)
    }
}

/// Analyze a character class like [abc] or [a-z]
fn analyze_char_class(chars: &mut std::iter::Peekable<std::str::Chars>, _case_insensitive: bool) -> SearchStrategy {
    let mut bytes = Vec::new();
    let negated = chars.peek() == Some(&'^');
    if negated {
        chars.next();
        // Negated classes are hard to optimize
        // Just consume until ] and return None
        while let Some(c) = chars.next() {
            if c == ']' {
                break;
            }
        }
        return SearchStrategy::None;
    }

    while let Some(c) = chars.next() {
        match c {
            ']' => break,
            '\\' => {
                match chars.next() {
                    Some('d') => {
                        // \d in character class
                        for b in b'0'..=b'9' {
                            bytes.push(b);
                        }
                    }
                    Some('w') => {
                        // Too many characters for memchr
                        return SearchStrategy::WordChar;
                    }
                    Some('s') => {
                        return SearchStrategy::Whitespace;
                    }
                    Some(escaped) if escaped.is_ascii() => {
                        bytes.push(escaped as u8);
                    }
                    _ => return SearchStrategy::None,
                }
            }
            '-' => {
                // Range - check if we have a previous byte
                if let Some(&prev) = bytes.last() {
                    if let Some(end) = chars.next() {
                        if end != ']' && end.is_ascii() {
                            let end_byte = end as u8;
                            // Add all bytes in range
                            for b in (prev + 1)..=end_byte {
                                bytes.push(b);
                            }
                        }
                    }
                }
            }
            _ if c.is_ascii() => {
                bytes.push(c as u8);
            }
            _ => return SearchStrategy::None,
        }

        // Limit to 128 bytes (ASCII range) to avoid huge allocations
        if bytes.len() > 128 {
            return SearchStrategy::None;
        }
    }

    // Deduplicate and sort
    bytes.sort_unstable();
    bytes.dedup();

    match bytes.len() {
        0 => SearchStrategy::None,
        1 => SearchStrategy::SingleByte(bytes[0]),
        2 => SearchStrategy::TwoBytes(bytes[0], bytes[1]),
        3 => SearchStrategy::ThreeBytes(bytes[0], bytes[1], bytes[2]),
        // Use bitmap for 4+ bytes - much faster than falling back to None!
        _ => SearchStrategy::Bitmap(ByteBitmap::from_bytes(&bytes)),
    }
}

/// Check if a pattern contains alternation at any depth (outside character classes)
/// This is used to detect patterns where a prefix from one branch shouldn't be
/// used to scan since it would miss matches from other branches.
fn pattern_has_alternation(pattern: &str) -> bool {
    let mut in_char_class = false;
    let mut chars = pattern.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\\' => { chars.next(); } // Skip escaped char
            '[' if !in_char_class => in_char_class = true,
            ']' if in_char_class => in_char_class = false,
            '|' if !in_char_class => return true, // Alternation at any depth
            _ => {}
        }
    }
    false
}

/// Analyze alternation like foo|bar|baz
/// Returns AlternationLiterals if all alternatives are pure literals (uses Aho-Corasick!)
/// Otherwise extracts first bytes for memchr optimization
fn analyze_alternation(pattern: &str, case_insensitive: bool) -> SearchStrategy {
    // First, try to extract all alternatives as pure literals
    let alternatives: Vec<&str> = split_top_level_alternation(pattern);

    if alternatives.len() >= 2 {
        // Check if all alternatives are pure literals (ASCII only for case-insensitive)
        let mut all_pure = true;
        let mut literals: Vec<Vec<u8>> = Vec::new();

        for alt in &alternatives {
            if let Some(lit) = extract_pure_literal(alt) {
                // For case-insensitive, only use AC if all literals are ASCII
                if case_insensitive && !lit.iter().all(|&b| b.is_ascii()) {
                    all_pure = false;
                    break;
                }
                literals.push(lit);
            } else {
                all_pure = false;
                break;
            }
        }

        if all_pure && !literals.is_empty() {
            // Use Aho-Corasick for multi-pattern matching - BLAZING FAST!
            // CRITICAL: MatchKind::LeftmostFirst enables optimal DFA construction
            // which is 10-12x faster than the default Standard mode.
            let ac = AhoCorasickBuilder::new()
                .match_kind(MatchKind::LeftmostFirst)
                .ascii_case_insensitive(case_insensitive)
                .build(&literals)
                .unwrap();
            return SearchStrategy::AlternationLiterals { literals, ac };
        }
    }

    // Fall back to extracting first bytes
    let mut first_bytes = Vec::new();
    let mut depth = 0;
    let mut chars = pattern.chars().peekable();
    let mut current_first: Option<u8> = None;

    while let Some(c) = chars.next() {
        match c {
            '(' => {
                depth += 1;
            }
            ')' => {
                if depth > 0 {
                    depth -= 1;
                }
            }
            '|' if depth == 0 => {
                // End of this alternative
                if let Some(b) = current_first {
                    first_bytes.push(b);
                }
                current_first = None;
            }
            '\\' if current_first.is_none() => {
                match chars.next() {
                    Some(escaped) if escaped.is_ascii() && !matches!(escaped, 'd' | 'w' | 's' | 'D' | 'W' | 'S') => {
                        current_first = Some(escaped as u8);
                    }
                    _ => return SearchStrategy::None,
                }
            }
            _ if current_first.is_none() && c.is_ascii() && !matches!(c, '.' | '*' | '+' | '?' | '[' | '^' | '$') => {
                if case_insensitive && c.is_ascii_alphabetic() {
                    // Add both cases
                    first_bytes.push(c.to_ascii_lowercase() as u8);
                    first_bytes.push(c.to_ascii_uppercase() as u8);
                    current_first = Some(c as u8); // Mark as found
                } else {
                    current_first = Some(c as u8);
                }
            }
            _ => {
                // Complex pattern in this alternative
                if current_first.is_none() {
                    return SearchStrategy::None;
                }
            }
        }
    }

    // Don't forget the last alternative
    if let Some(b) = current_first {
        first_bytes.push(b);
    }

    // Deduplicate
    first_bytes.sort_unstable();
    first_bytes.dedup();

    match first_bytes.len() {
        0 => SearchStrategy::None,
        1 => SearchStrategy::SingleByte(first_bytes[0]),
        2 => SearchStrategy::TwoBytes(first_bytes[0], first_bytes[1]),
        3 => SearchStrategy::ThreeBytes(first_bytes[0], first_bytes[1], first_bytes[2]),
        _ => SearchStrategy::Bitmap(ByteBitmap::from_bytes(&first_bytes)),
    }
}

/// Split a pattern on top-level '|' (not inside groups)
fn split_top_level_alternation(pattern: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut depth = 0;
    let mut start = 0;

    for (i, c) in pattern.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => { if depth > 0 { depth -= 1; } }
            '|' if depth == 0 => {
                result.push(&pattern[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    result.push(&pattern[start..]);
    result
}

/// Extract a pure literal from a simple pattern (no metacharacters)
fn extract_pure_literal(pattern: &str) -> Option<Vec<u8>> {
    let mut literal = Vec::new();
    let mut chars = pattern.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            // Metacharacters - not a pure literal
            '.' | '*' | '+' | '?' | '[' | ']' | '(' | ')' | '{' | '}' | '$' | '^' | '|' => {
                return None;
            }
            '\\' => {
                // Handle escape sequences
                match chars.next() {
                    Some('\\') => literal.push(b'\\'),
                    Some('n') => literal.push(b'\n'),
                    Some('r') => literal.push(b'\r'),
                    Some('t') => literal.push(b'\t'),
                    Some('.') => literal.push(b'.'),
                    Some('*') => literal.push(b'*'),
                    Some('+') => literal.push(b'+'),
                    Some('?') => literal.push(b'?'),
                    Some('[') => literal.push(b'['),
                    Some(']') => literal.push(b']'),
                    Some('(') => literal.push(b'('),
                    Some(')') => literal.push(b')'),
                    Some('{') => literal.push(b'{'),
                    Some('}') => literal.push(b'}'),
                    Some('|') => literal.push(b'|'),
                    Some('^') => literal.push(b'^'),
                    Some('$') => literal.push(b'$'),
                    Some('/') => literal.push(b'/'),
                    // Character classes are not literals
                    Some('d') | Some('D') | Some('w') | Some('W') | Some('s') | Some('S') | Some('b') | Some('B') => {
                        return None;
                    }
                    _ => return None,
                }
            }
            _ if c.is_ascii() => {
                literal.push(c as u8);
            }
            _ => {
                // Non-ASCII - encode as UTF-8
                let mut buf = [0u8; 4];
                let encoded = c.encode_utf8(&mut buf);
                literal.extend_from_slice(encoded.as_bytes());
            }
        }
    }

    if literal.is_empty() {
        None
    } else {
        Some(literal)
    }
}

// ============================================================================
// Fast byte scanning helpers
// ============================================================================

// Pre-computed bitmaps for common character classes (used for testing and Bitmap strategy)
static DIGIT_BITMAP: ByteBitmap = {
    let mut bits = [0u64; 4];
    bits[0] = 0x03FF_0000_0000_0000; // bits 48-57 set for '0'-'9'
    ByteBitmap { bits }
};

static WORD_CHAR_BITMAP: ByteBitmap = {
    let mut bits = [0u64; 4];
    bits[0] = 0x03FF_0000_0000_0000;  // '0'-'9' (48-57)
    bits[1] = 0x07FF_FFFE_87FF_FFFE;  // A-Z (1-26), _ (31), a-z (33-58)
    ByteBitmap { bits }
};

/// Word chars + all high bytes (0x80-0xFF) for Unicode mode
/// In Unicode mode, \w matches ID_Continue which includes Cyrillic, Greek, etc.
/// High bytes are UTF-8 lead/continuation bytes that could be Unicode word chars.
static UNICODE_WORD_CHAR_BITMAP: ByteBitmap = {
    let mut bits = [0u64; 4];
    bits[0] = 0x03FF_0000_0000_0000;  // '0'-'9' (48-57)
    bits[1] = 0x07FF_FFFE_87FF_FFFE;  // A-Z (1-26), _ (31), a-z (33-58)
    bits[2] = 0xFFFF_FFFF_FFFF_FFFF;  // 128-191 (all high bytes)
    bits[3] = 0xFFFF_FFFF_FFFF_FFFF;  // 192-255 (all high bytes)
    ByteBitmap { bits }
};

static WHITESPACE_BITMAP: ByteBitmap = {
    let mut bits = [0u64; 4];
    bits[0] = (1u64 << 32) | (1u64 << 9) | (1u64 << 10) | (1u64 << 11) | (1u64 << 12) | (1u64 << 13);
    ByteBitmap { bits }
};

/// Find the first digit (0-9) in a byte slice using SIMD
#[inline]
fn find_digit(bytes: &[u8]) -> Option<usize> {
    find_first_digit(bytes, 0)
}

/// Find the first word character (a-z, A-Z, 0-9, _) in a byte slice
#[inline]
fn find_word_char(bytes: &[u8]) -> Option<usize> {
    // For word chars, bitmap is actually good since we have 63 possible bytes
    WORD_CHAR_BITMAP.find_in_slice(bytes)
}

/// Find the first word character or high byte (for Unicode mode)
/// In Unicode mode, any byte >= 0x80 could start a Unicode word character
#[inline]
fn find_unicode_word_char(bytes: &[u8]) -> Option<usize> {
    UNICODE_WORD_CHAR_BITMAP.find_in_slice(bytes)
}

/// Find the first whitespace character
#[inline]
fn find_whitespace(bytes: &[u8]) -> Option<usize> {
    // Common whitespace: space, tab, newline - use memchr3 for SIMD
    memchr3(b' ', b'\t', b'\n', bytes)
}

// ============================================================================
// PURE FAST PATHS - Complete pattern matching without interpreter
// ============================================================================

/// Find the first digit in bytes[start..] using LLVM auto-vectorization
/// The simple range check `b.wrapping_sub(b'0') <= 9` is easily vectorized
#[inline]
fn find_first_digit(bytes: &[u8], start: usize) -> Option<usize> {
    // Use explicit SIMD for digit finding - much faster than LLVM auto-vectorization
    find_first_digit_simd(&bytes[start..]).map(|pos| start + pos)
}

/// Find a run of consecutive digits [0-9]+ starting at or after `start`
/// Returns the match directly - NO INTERPRETER NEEDED!
#[inline]
fn find_digit_run(bytes: &[u8], start: usize) -> Option<Match> {
    // Use SIMD-accelerated search to find first digit
    let match_start = find_first_digit(bytes, start)?;

    // Find end of digit run
    let mut pos = match_start + 1;
    while pos < bytes.len() && bytes[pos].wrapping_sub(b'0') <= 9 {
        pos += 1;
    }

    Some(Match { start: match_start, end: pos })
}

/// Find a run of consecutive lowercase letters [a-z]+ starting at or after `start`
/// Uses LLVM-vectorizable range check pattern
#[inline]
fn find_lower_run(bytes: &[u8], start: usize) -> Option<Match> {
    // Find first lowercase: b.wrapping_sub(b'a') <= 25 is vectorizable
    let match_start = bytes[start..]
        .iter()
        .position(|&b| b.wrapping_sub(b'a') <= 25)
        .map(|p| start + p)?;

    // Find end of run
    let match_end = bytes[match_start..]
        .iter()
        .position(|&b| b.wrapping_sub(b'a') > 25)
        .map(|p| match_start + p)
        .unwrap_or(bytes.len());

    Some(Match { start: match_start, end: match_end })
}

/// Find a run of consecutive uppercase letters [A-Z]+ starting at or after `start`
/// Uses LLVM-vectorizable range check pattern
#[inline]
fn find_upper_run(bytes: &[u8], start: usize) -> Option<Match> {
    // Find first uppercase: b.wrapping_sub(b'A') <= 25 is vectorizable
    let match_start = bytes[start..]
        .iter()
        .position(|&b| b.wrapping_sub(b'A') <= 25)
        .map(|p| start + p)?;

    // Find end of run
    let match_end = bytes[match_start..]
        .iter()
        .position(|&b| b.wrapping_sub(b'A') > 25)
        .map(|p| match_start + p)
        .unwrap_or(bytes.len());

    Some(Match { start: match_start, end: match_end })
}

/// Check if byte is ASCII alphabetic using bit trick
/// (b | 0x20).wrapping_sub(b'a') <= 25 maps A-Z and a-z to 0-25
#[inline(always)]
fn is_alpha(b: u8) -> bool {
    (b | 0x20).wrapping_sub(b'a') <= 25
}

/// Check if byte is ASCII alphanumeric
#[inline(always)]
fn is_alnum(b: u8) -> bool {
    is_alpha(b) || b.wrapping_sub(b'0') <= 9
}

/// Find a run of consecutive letters [a-zA-Z]+ starting at or after `start`
/// Uses LLVM-vectorizable bit trick for alphabet check
#[inline]
fn find_alpha_run(bytes: &[u8], start: usize) -> Option<Match> {
    // Find first alphabetic using vectorizable pattern
    let match_start = bytes[start..]
        .iter()
        .position(|&b| is_alpha(b))
        .map(|p| start + p)?;

    // Find end of run
    let match_end = bytes[match_start..]
        .iter()
        .position(|&b| !is_alpha(b))
        .map(|p| match_start + p)
        .unwrap_or(bytes.len());

    Some(Match { start: match_start, end: match_end })
}

/// Find a run of consecutive alphanumeric chars [a-zA-Z0-9]+ starting at or after `start`
/// Uses LLVM-vectorizable patterns
#[inline]
fn find_alnum_run(bytes: &[u8], start: usize) -> Option<Match> {
    // Find first alphanumeric
    let match_start = bytes[start..]
        .iter()
        .position(|&b| is_alnum(b))
        .map(|p| start + p)?;

    // Find end of run
    let match_end = bytes[match_start..]
        .iter()
        .position(|&b| !is_alnum(b))
        .map(|p| match_start + p)
        .unwrap_or(bytes.len());

    Some(Match { start: match_start, end: match_end })
}

/// Check if byte is a word character [a-zA-Z0-9_]
#[inline(always)]
fn is_word_char(b: u8) -> bool {
    is_alnum(b) || b == b'_'
}

/// Find a run of consecutive word chars [a-zA-Z0-9_]+ or \w+ starting at or after `start`
/// Uses LLVM-vectorizable patterns
#[inline]
fn find_word_run(bytes: &[u8], start: usize) -> Option<Match> {
    // Find first word char
    let match_start = bytes[start..]
        .iter()
        .position(|&b| is_word_char(b))
        .map(|p| start + p)?;

    // Find end of run
    let match_end = bytes[match_start..]
        .iter()
        .position(|&b| !is_word_char(b))
        .map(|p| match_start + p)
        .unwrap_or(bytes.len());

    Some(Match { start: match_start, end: match_end })
}

/// Fast literal search using memchr + inline verification
/// Optimized for repeated find_at calls - avoids memmem preprocessing overhead
#[inline]
fn find_literal_memchr(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }

    let needle_len = needle.len();
    if needle_len > haystack.len() {
        return None;
    }

    // Single byte: just use memchr
    if needle_len == 1 {
        return memchr(needle[0], haystack);
    }

    // Use memchr to find first byte, then verify rest
    let first_byte = needle[0];
    let needle_rest = &needle[1..];
    let mut offset = 0;

    while offset + needle_len <= haystack.len() {
        if let Some(pos) = memchr(first_byte, &haystack[offset..]) {
            let abs_pos = offset + pos;
            if abs_pos + needle_len > haystack.len() {
                return None;
            }
            // Verify the rest of the needle
            if &haystack[abs_pos + 1..abs_pos + needle_len] == needle_rest {
                return Some(abs_pos);
            }
            offset = abs_pos + 1;
        } else {
            return None;
        }
    }
    None
}

/// Fast literal search using memchr + verification with rare byte heuristic
/// For patterns with rare bytes (uppercase, digits, punctuation), this is faster
/// than memmem::find. For patterns with only common letters, use memmem.
#[inline]
fn find_literal_fast(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }

    let needle_len = needle.len();
    let haystack_len = haystack.len();

    if needle_len > haystack_len {
        return None;
    }

    // Single byte: just use memchr
    if needle_len == 1 {
        return memchr(needle[0], haystack);
    }

    // For short patterns (<=4 bytes), memmem is well-optimized
    if needle_len <= 4 {
        return memmem::find(haystack, needle);
    }

    // For longer patterns, check if there's a rare byte to search for
    let (rare_byte_idx, rare_score) = find_rare_byte_index_with_score(needle);

    // If no rare byte (score < 50), fall back to memmem
    if rare_score < 50 {
        return memmem::find(haystack, needle);
    }

    // Use rare byte heuristic: search for the rarest byte in the pattern
    let rare_byte = needle[rare_byte_idx];

    let mut offset = 0;
    while offset + needle_len <= haystack_len {
        // Search for the rare byte at its expected position
        let search_start = offset + rare_byte_idx;
        if search_start >= haystack_len {
            break;
        }

        if let Some(rare_pos) = memchr(rare_byte, &haystack[search_start..]) {
            let candidate_start = search_start + rare_pos - rare_byte_idx;

            // Check if candidate is valid and within bounds
            if candidate_start >= offset && candidate_start + needle_len <= haystack_len {
                // Verify the entire pattern
                if &haystack[candidate_start..candidate_start + needle_len] == needle {
                    return Some(candidate_start);
                }
            }

            // Move past this position
            offset = search_start + rare_pos + 1;
            if rare_byte_idx > 0 {
                offset = offset.saturating_sub(rare_byte_idx);
            }
        } else {
            break;
        }
    }

    None
}

/// Find the index of the "rarest" byte in a pattern, along with its rarity score
/// Rarer bytes (less common in typical text) lead to fewer false positives
/// Returns (index, score) where score >= 50 indicates a rare byte worth using
#[inline]
fn find_rare_byte_index_with_score(needle: &[u8]) -> (usize, u8) {
    // Byte frequency heuristic: uppercase, digits, and punctuation are rarer
    // Common bytes: space, e, t, a, o, i, n, s, r, h (most common in English)
    const COMMON_BYTES: [u8; 12] = [b' ', b'e', b't', b'a', b'o', b'i', b'n', b's', b'r', b'h', b'l', b'd'];

    let mut best_idx = 0;
    let mut best_score = 0u8;

    for (i, &b) in needle.iter().enumerate() {
        let score = if b.is_ascii_uppercase() {
            200 // Uppercase letters are rare
        } else if b.is_ascii_digit() {
            150 // Digits are somewhat rare
        } else if !b.is_ascii_alphanumeric() && b != b' ' {
            180 // Punctuation is rare
        } else if COMMON_BYTES.contains(&b.to_ascii_lowercase()) {
            10 // Common letters
        } else {
            50 // Other lowercase letters
        };

        if score > best_score {
            best_score = score;
            best_idx = i;
        }
    }

    (best_idx, best_score)
}

/// Find a quoted string "[^"]*" or '[^']*' starting at or after `start`
/// This directly finds the opening quote, scans for closing quote, returns match.
#[inline]
fn find_quoted_string(bytes: &[u8], start: usize, quote: u8) -> Option<Match> {
    let slice = &bytes[start..];

    // Find the opening quote
    let open_pos = memchr(quote, slice)?;
    let abs_open = start + open_pos;

    // Find the closing quote (starting after the opening quote)
    let rest = &bytes[abs_open + 1..];
    let close_offset = memchr(quote, rest)?;
    let abs_close = abs_open + 1 + close_offset;

    // Match includes both quotes
    Some(Match { start: abs_open, end: abs_close + 1 })
}

/// Find a capital word [A-Z][a-z]+ starting at or after `start`
/// Returns the match directly - NO INTERPRETER NEEDED!
#[inline]
fn find_capital_word(bytes: &[u8], start: usize) -> Option<Match> {
    let mut pos = start;

    while pos < bytes.len() {
        let b = bytes[pos];

        // Found uppercase letter?
        if b.is_ascii_uppercase() {
            let match_start = pos;
            pos += 1;

            // Must have at least one lowercase letter following
            if pos < bytes.len() && bytes[pos].is_ascii_lowercase() {
                pos += 1;
                // Consume all following lowercase letters
                while pos < bytes.len() && bytes[pos].is_ascii_lowercase() {
                    pos += 1;
                }
                return Some(Match { start: match_start, end: pos });
            }
            // No lowercase letter - continue searching
        } else {
            pos += 1;
        }
    }
    None
}

/// Find a lowercase word ending with a literal suffix [a-z]+suffix starting at or after `start`
/// Returns the match directly - NO INTERPRETER NEEDED!
///
/// Algorithm: Find lowercase runs and locate the RIGHTMOST occurrence of the suffix.
/// This matches regex semantics of [a-z]+suffix (greedy).
///
/// For "singing": rightmost "ing" is at position 4, match = "singing"
/// For "winged": only "ing" is at position 1, match = "wing"
#[inline]
fn find_lower_suffix(bytes: &[u8], start: usize, suffix: &[u8]) -> Option<Match> {
    let suffix_len = suffix.len();
    if suffix_len == 0 || start >= bytes.len() {
        return None;
    }

    // Use SIMD-accelerated memmem to find suffix occurrences directly
    // This is much faster than scanning byte-by-byte for lowercase runs
    let finder = memmem::Finder::new(suffix);

    let mut pos = start;
    while pos < bytes.len() {
        // Find next suffix occurrence using memmem (SIMD-accelerated)
        let Some(rel_pos) = finder.find(&bytes[pos..]) else {
            break;
        };
        let suffix_start = pos + rel_pos;

        // Need at least 1 lowercase char before suffix
        if suffix_start == 0 || !bytes[suffix_start - 1].is_ascii_lowercase() {
            pos = suffix_start + 1;
            continue;
        }

        // Find start of the lowercase run (scan backwards)
        let mut run_start = suffix_start - 1;
        while run_start > 0 && bytes[run_start - 1].is_ascii_lowercase() {
            run_start -= 1;
        }

        // Find end of the lowercase run
        let mut run_end = suffix_start + suffix_len;
        while run_end < bytes.len() && bytes[run_end].is_ascii_lowercase() {
            run_end += 1;
        }

        // Search for RIGHTMOST suffix within this run for greedy matching
        // Start from the rightmost possible position and scan backwards
        let mut check_pos = run_end - suffix_len;
        while check_pos > run_start {
            if &bytes[check_pos..check_pos + suffix_len] == suffix {
                return Some(Match { start: run_start, end: check_pos + suffix_len });
            }
            check_pos -= 1;
        }

        // Move past this run
        pos = run_end;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_match() {
        let re = Regex::new("hello").unwrap();
        assert!(re.is_match("hello world"));
        assert!(!re.is_match("goodbye world"));
    }

    #[test]
    fn test_find_position() {
        let re = Regex::new("world").unwrap();
        let m = re.find("hello world").unwrap();
        assert_eq!(m.start, 6);
        assert_eq!(m.end, 11);
        assert_eq!(m.as_str("hello world"), "world");
    }

    #[test]
    fn test_character_class() {
        let re = Regex::new("[0-9]+").unwrap();
        assert!(re.is_match("abc123def"));
        let m = re.find("abc123def").unwrap();
        assert_eq!(m.as_str("abc123def"), "123");
    }

    #[test]
    fn test_alternation() {
        let re = Regex::new("cat|dog").unwrap();
        assert!(re.is_match("I have a cat"));
        assert!(re.is_match("I have a dog"));
        assert!(!re.is_match("I have a bird"));
    }

    #[test]
    fn test_quantifiers() {
        let re = Regex::new("a+").unwrap();
        let m = re.find("baaab").unwrap();
        assert_eq!(m.as_str("baaab"), "aaa");
    }

    #[test]
    fn test_quantifier_star() {
        let re = Regex::new("a*").unwrap();
        let m = re.find("aaabbb").unwrap();
        assert_eq!((m.start, m.end), (0, 3));
    }

    #[test]
    fn test_quantifier_question() {
        let re = Regex::new("a?").unwrap();
        let m = re.find("aaabbb").unwrap();
        assert_eq!((m.start, m.end), (0, 1));
    }

    #[test]
    fn test_quantifier_range() {
        let re = Regex::new("a{2,4}").unwrap();
        let m = re.find("aaaaa").unwrap();
        assert_eq!((m.start, m.end), (0, 4));
    }

    #[test]
    fn test_quantifier_range_lazy() {
        let re = Regex::new("a{2,4}?").unwrap();
        let m = re.find("aaaaa").unwrap();
        assert_eq!((m.start, m.end), (0, 2));
    }

    #[test]
    fn test_backreference() {
        let re = Regex::new(r"(a)\1").unwrap();
        assert!(re.is_match("aa"));
        assert!(!re.is_match("ab"));

        let re2 = Regex::new(r"(\w+)\s+\1").unwrap();
        assert!(re2.is_match("hello hello"));
        assert!(!re2.is_match("hello world"));
    }

    #[test]
    fn test_case_insensitive() {
        let re = Regex::with_flags("hello", Flags::from_bits(Flags::IGNORE_CASE)).unwrap();
        assert!(re.is_match("HELLO"));
        assert!(re.is_match("Hello"));
        assert!(re.is_match("hello"));
    }

    #[test]
    fn test_capture_count() {
        let re = Regex::new("(a)(b)(c)").unwrap();
        assert_eq!(re.capture_count(), 4); // Group 0 + 3 explicit groups
    }

    #[test]
    fn test_display() {
        let re = Regex::with_flags("test", Flags::from_bits(Flags::GLOBAL | Flags::IGNORE_CASE)).unwrap();
        assert_eq!(re.to_string(), "/test/gi");
    }

    #[test]
    fn test_invalid_pattern() {
        let result = Regex::new("(unclosed");
        assert!(result.is_err());
    }

    #[test]
    fn test_word_boundary() {
        let re = Regex::new(r"\bword\b").unwrap();
        assert!(re.is_match("a word here"));
        assert!(!re.is_match("awordhere"));
    }

    #[test]
    fn test_digit_shorthand() {
        let re = Regex::new(r"\d+").unwrap();
        assert!(re.is_match("abc123"));
        let m = re.find("abc123").unwrap();
        assert_eq!(m.as_str("abc123"), "123");
    }

    #[test]
    fn test_find_iter() {
        let re = Regex::new(r"\d+").unwrap();
        let text = "a1b22c333d4444";
        let matches: Vec<_> = re.find_iter(text).collect();
        assert_eq!(matches.len(), 4);
        assert_eq!(matches[0].as_str(text), "1");
        assert_eq!(matches[1].as_str(text), "22");
        assert_eq!(matches[2].as_str(text), "333");
        assert_eq!(matches[3].as_str(text), "4444");
    }

    #[test]
    fn test_find_iter_no_matches() {
        let re = Regex::new(r"\d+").unwrap();
        let matches: Vec<_> = re.find_iter("no digits here").collect();
        assert!(matches.is_empty());
    }

    #[test]
    fn test_captures_basic() {
        let re = Regex::new(r"(\w+)@(\w+)").unwrap();
        let caps = re.captures("user@host").unwrap();
        assert_eq!(caps.len(), 3); // Group 0 + 2 captures
        assert_eq!(caps.get_str(0), Some("user@host"));
        assert_eq!(caps.get_str(1), Some("user"));
        assert_eq!(caps.get_str(2), Some("host"));
    }

    #[test]
    fn test_captures_no_match() {
        let re = Regex::new(r"(\d+)").unwrap();
        let caps = re.captures("no digits");
        assert!(caps.is_none());
    }

    #[test]
    fn test_captures_optional_group() {
        let re = Regex::new(r"(\d+)(x)?").unwrap();
        let caps = re.captures("123").unwrap();
        assert_eq!(caps.get_str(0), Some("123"));
        assert_eq!(caps.get_str(1), Some("123"));
        assert!(caps.get(2).is_none()); // Optional group didn't match
    }

    #[test]
    fn test_captures_iter() {
        let re = Regex::new(r"(a)(b)(c)").unwrap();
        let caps = re.captures("abc").unwrap();
        let groups: Vec<_> = caps.iter().collect();
        assert_eq!(groups.len(), 4);
        assert!(groups[0].is_some()); // Full match
        assert!(groups[1].is_some()); // (a)
        assert!(groups[2].is_some()); // (b)
        assert!(groups[3].is_some()); // (c)
    }

    #[test]
    fn test_match_methods() {
        let re = Regex::new("test").unwrap();
        let m = re.find("a test here").unwrap();
        assert_eq!(m.start, 2);
        assert_eq!(m.end, 6);
        assert_eq!(m.len(), 4);
        assert!(!m.is_empty());
    }

    // ========================================================================
    // Search strategy tests
    // ========================================================================

    #[test]
    fn test_strategy_anchored() {
        // Pure anchored literal
        let strategy = analyze_pattern("^hello", Flags::empty());
        match strategy {
            SearchStrategy::AnchoredLiteral(lit) => assert_eq!(lit, b"hello"),
            _ => panic!("Expected AnchoredLiteral, got {:?}", strategy),
        }

        // Complex anchored pattern falls back to Anchored
        let strategy = analyze_pattern("^hello.*world", Flags::empty());
        assert!(matches!(strategy, SearchStrategy::Anchored));
    }

    #[test]
    fn test_strategy_single_byte() {
        let strategy = analyze_pattern("x.*", Flags::empty());
        assert!(matches!(strategy, SearchStrategy::SingleByte(b'x')));
    }

    #[test]
    fn test_strategy_pure_literal() {
        // Pure literal patterns should use PureLiteral (no engine!)
        let strategy = analyze_pattern("needle", Flags::empty());
        match strategy {
            SearchStrategy::PureLiteral(finder) => {
                assert_eq!(finder.needle(), b"needle");
            }
            _ => panic!("Expected PureLiteral, got {:?}", strategy),
        }

        // Single char pure literal
        let strategy = analyze_pattern("x", Flags::empty());
        match strategy {
            SearchStrategy::PureLiteral(finder) => {
                assert_eq!(finder.needle(), b"x");
            }
            _ => panic!("Expected PureLiteral for single char, got {:?}", strategy),
        }
    }

    #[test]
    fn test_strategy_utf8_literal() {
        // UTF-8 pure literal (Russian "Sherlock Holmes")
        let strategy = analyze_pattern("Шерлок Холмс", Flags::empty());
        match strategy {
            SearchStrategy::PureLiteral(finder) => {
                // UTF-8 encoding of "Шерлок Холмс"
                assert_eq!(finder.needle(), "Шерлок Холмс".as_bytes());
            }
            _ => panic!("Russian text should be PureLiteral, got {:?}", strategy),
        }

        // Chinese text
        let strategy = analyze_pattern("福尔摩斯", Flags::empty());
        match strategy {
            SearchStrategy::PureLiteral(finder) => {
                assert_eq!(finder.needle(), "福尔摩斯".as_bytes());
            }
            _ => panic!("Chinese text should be PureLiteral, got {:?}", strategy),
        }
    }

    #[test]
    fn test_strategy_benchmark_patterns() {
        // Verify benchmark patterns use PureLiteral
        let strategy = analyze_pattern("Holmes", Flags::empty());
        match strategy {
            SearchStrategy::PureLiteral(finder) => {
                assert_eq!(finder.needle(), b"Holmes");
            }
            _ => panic!("Holmes should be PureLiteral, got {:?}", strategy),
        }

        let strategy = analyze_pattern("the", Flags::empty());
        match strategy {
            SearchStrategy::PureLiteral(finder) => {
                assert_eq!(finder.needle(), b"the");
            }
            _ => panic!("the should be PureLiteral, got {:?}", strategy),
        }
    }

    #[test]
    fn test_strategy_literal_prefix() {
        let strategy = analyze_pattern("hello.*world", Flags::empty());
        match strategy {
            SearchStrategy::LiteralPrefix(finder) => {
                assert_eq!(finder.needle(), b"hello");
            }
            _ => panic!("Expected LiteralPrefix, got {:?}", strategy),
        }
    }

    #[test]
    fn test_strategy_digit() {
        // \d+ now uses PureDigitPlus (fast path, no interpreter!)
        let strategy = analyze_pattern(r"\d+", Flags::empty());
        assert!(matches!(strategy, SearchStrategy::PureDigitPlus));

        // Single \d without + still uses Digit
        let strategy = analyze_pattern(r"\d", Flags::empty());
        assert!(matches!(strategy, SearchStrategy::Digit));
    }

    #[test]
    fn test_strategy_word_char() {
        // \w+ now uses PureWordPlus (fast path, no interpreter!)
        let strategy = analyze_pattern(r"\w+", Flags::empty());
        assert!(matches!(strategy, SearchStrategy::PureWordPlus));

        // Single \w without + still uses WordChar
        let strategy = analyze_pattern(r"\w", Flags::empty());
        assert!(matches!(strategy, SearchStrategy::WordChar));
    }

    #[test]
    fn test_strategy_whitespace() {
        let strategy = analyze_pattern(r"\s+", Flags::empty());
        assert!(matches!(strategy, SearchStrategy::Whitespace));
    }

    #[test]
    fn test_strategy_bitmap() {
        // Character class with 4+ chars should use Bitmap
        let strategy = analyze_pattern("[aeiou].*", Flags::empty());
        match strategy {
            SearchStrategy::Bitmap(bm) => {
                assert!(bm.contains(b'a'));
                assert!(bm.contains(b'e'));
                assert!(bm.contains(b'i'));
                assert!(bm.contains(b'o'));
                assert!(bm.contains(b'u'));
                assert!(!bm.contains(b'b'));
                assert_eq!(bm.count(), 5);
            }
            _ => panic!("Expected Bitmap, got {:?}", strategy),
        }

        // Larger range should also use Bitmap
        let strategy = analyze_pattern("[a-z].*", Flags::empty());
        match strategy {
            SearchStrategy::Bitmap(bm) => {
                assert!(bm.contains(b'a'));
                assert!(bm.contains(b'm'));
                assert!(bm.contains(b'z'));
                assert!(!bm.contains(b'A'));
                assert_eq!(bm.count(), 26);
            }
            _ => panic!("Expected Bitmap for [a-z], got {:?}", strategy),
        }
    }

    #[test]
    fn test_bitmap_find() {
        // Test that Bitmap-based search actually works
        let re = Regex::new("[aeiou]test").unwrap();
        let text = "x".repeat(1000) + "atest";
        let m = re.find(&text).unwrap();
        assert_eq!(m.as_str(&text), "atest");
    }

    #[test]
    fn test_static_bitmaps() {
        // Verify DIGIT_BITMAP contains 0-9
        for b in b'0'..=b'9' {
            assert!(DIGIT_BITMAP.contains(b), "DIGIT_BITMAP should contain '{}'", b as char);
        }
        assert!(!DIGIT_BITMAP.contains(b'a'));
        assert!(!DIGIT_BITMAP.contains(b'/'));

        // Verify WORD_CHAR_BITMAP contains a-z, A-Z, 0-9, _
        for b in b'a'..=b'z' {
            assert!(WORD_CHAR_BITMAP.contains(b), "WORD_CHAR should contain '{}'", b as char);
        }
        for b in b'A'..=b'Z' {
            assert!(WORD_CHAR_BITMAP.contains(b), "WORD_CHAR should contain '{}'", b as char);
        }
        for b in b'0'..=b'9' {
            assert!(WORD_CHAR_BITMAP.contains(b), "WORD_CHAR should contain '{}'", b as char);
        }
        assert!(WORD_CHAR_BITMAP.contains(b'_'));
        assert!(!WORD_CHAR_BITMAP.contains(b'-'));
        assert!(!WORD_CHAR_BITMAP.contains(b' '));

        // Verify WHITESPACE_BITMAP
        assert!(WHITESPACE_BITMAP.contains(b' '));
        assert!(WHITESPACE_BITMAP.contains(b'\t'));
        assert!(WHITESPACE_BITMAP.contains(b'\n'));
        assert!(WHITESPACE_BITMAP.contains(b'\r'));
        assert!(!WHITESPACE_BITMAP.contains(b'a'));
    }

    #[test]
    fn test_find_digit_branchless() {
        // Test find_digit with various inputs
        assert_eq!(find_digit(b"abc123"), Some(3));
        assert_eq!(find_digit(b"123"), Some(0));
        assert_eq!(find_digit(b"abc"), None);
        assert_eq!(find_digit(b""), None);

        // Long string with digit at end
        let mut long = vec![b'x'; 1000];
        long.push(b'5');
        assert_eq!(find_digit(&long), Some(1000));
    }

    #[test]
    fn test_strategy_case_insensitive() {
        // Case insensitive single letter should use TwoBytes
        let strategy = analyze_pattern("a.*", Flags::from_bits(Flags::IGNORE_CASE));
        assert!(matches!(strategy, SearchStrategy::TwoBytes(b'a', b'A')));
    }

    #[test]
    fn test_strategy_alternation() {
        // Pure literal alternation now uses Aho-Corasick
        let strategy = analyze_pattern("cat|dog", Flags::empty());
        match strategy {
            SearchStrategy::AlternationLiterals { literals, .. } => {
                assert_eq!(literals.len(), 2);
                assert_eq!(literals[0], b"cat");
                assert_eq!(literals[1], b"dog");
            }
            _ => panic!("Expected AlternationLiterals, got {:?}", strategy),
        }

        // Case-insensitive alternation now uses Aho-Corasick with ascii_case_insensitive
        let strategy = analyze_pattern("cat|dog", Flags::from_bits(Flags::IGNORE_CASE));
        match strategy {
            SearchStrategy::AlternationLiterals { literals, .. } => {
                assert_eq!(literals.len(), 2);
                assert_eq!(literals[0], b"cat");
                assert_eq!(literals[1], b"dog");
            }
            _ => panic!("Expected AlternationLiterals for case-insensitive, got {:?}", strategy),
        }
    }

    // ========================================================================
    // Optimization behavior tests
    // ========================================================================

    #[test]
    fn test_optimization_long_input() {
        let re = Regex::new("needle.*thread").unwrap();
        let text = "hay".repeat(10000) + "needle and thread here";

        // Should find the match using literal prefix optimization
        let m = re.find(&text).unwrap();
        assert_eq!(m.as_str(&text), "needle and thread");
    }

    #[test]
    fn test_optimization_no_match() {
        let re = Regex::new("needle").unwrap();
        let text = "hay".repeat(1000); // No needle

        // Should return None efficiently
        assert!(re.find(&text).is_none());
    }

    #[test]
    fn test_optimization_multiple_candidates() {
        // Pattern where prefix matches multiple times but full pattern only matches once
        let re = Regex::new("hello world").unwrap();
        let text = "hello there, hello friend, hello world!";

        let m = re.find(text).unwrap();
        assert_eq!(m.as_str(text), "hello world");
        assert_eq!(m.start, 27);
    }

    #[test]
    fn test_optimization_digit_search() {
        let re = Regex::new(r"\d+").unwrap();
        let text = "a".repeat(1000) + "12345";

        let m = re.find(&text).unwrap();
        assert_eq!(m.as_str(&text), "12345");
    }

    #[test]
    fn test_optimization_anchored() {
        let re = Regex::new("^hello").unwrap();

        // Should only match at start
        assert!(re.find("hello world").is_some());
        assert!(re.find("say hello").is_none());
    }

    #[test]
    fn test_optimization_alternation() {
        let re = Regex::new("cat|dog|bird").unwrap();
        let text = "x".repeat(1000) + "the dog ran";

        let m = re.find(&text).unwrap();
        assert_eq!(m.as_str(&text), "dog");
    }

    #[test]
    fn test_ruff_noqa_captures() {
        let re = Regex::new(r"(\s*)((?:# [Nn][Oo][Qq][Aa])(?::\s?(([A-Z]+[0-9]+(?:[,\s]+)?)+))?)").unwrap();

        // Bare # noqa — groups 3,4 should be None
        let caps = re.captures("  # noqa").unwrap();
        assert_eq!(caps.count_matched(), 3, "bare noqa: groups 0,1,2 should match, not 3,4");
        assert!(caps.get(3).is_none(), "group 3 should be None for bare noqa");
        assert!(caps.get(4).is_none(), "group 4 should be None for bare noqa");

        // # noqa: E501 — all groups should match
        let caps = re.captures("  # noqa: E501").unwrap();
        assert_eq!(caps.count_matched(), 5, "noqa with code: all 5 groups should match");

        // grep-captures model: count matched groups per line
        let mut count = 0;
        for line in ["  # noqa", "  # noqa: E501", "nothing here", "  # noqa: E501,F401"].iter() {
            let mut pos = 0;
            while pos < line.len() {
                match re.captures_at(line, pos) {
                    Some(caps) => {
                        count += caps.count_matched();
                        if let Some(m) = caps.get(0) {
                            let end = m.end;
                            let start = m.start;
                            pos = if end > start { end } else { start + 1 };
                        } else {
                            break;
                        }
                    }
                    None => break,
                }
            }
        }
        assert_eq!(count, 3 + 5 + 5, "grep-captures count should be 13");
    }
}
