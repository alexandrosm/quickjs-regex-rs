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

// C2Rust generated modules (formerly in src/generated/)
mod util;
mod unicode;
pub(crate) mod engine;

// Clean Rust interpreter (experimental)
mod interpreter;

pub use opcodes::OpCode;
pub use flags::{Flags, InvalidFlag};
pub use error::{Error, Result, ExecResult};

use std::ffi::CStr;
use std::ptr;

use aho_corasick::AhoCorasick;
use memchr::{memchr, memchr2, memchr3, memmem};

// Threshold for using optimizations (bytes)
const OPTIMIZATION_THRESHOLD: usize = 32;

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
    /// Pattern is a pure literal - no engine needed!
    PureLiteral(Vec<u8>),
    /// Single literal byte to search for
    SingleByte(u8),
    /// Two possible first bytes (e.g., alternation or case-insensitive)
    TwoBytes(u8, u8),
    /// Three possible first bytes
    ThreeBytes(u8, u8, u8),
    /// Multi-byte literal prefix
    LiteralPrefix(Vec<u8>),
    /// Alternation of pure literals - use Aho-Corasick!
    AlternationLiterals {
        literals: Vec<Vec<u8>>,
        ac: AhoCorasick,
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
            SearchStrategy::PureLiteral(lit) => write!(f, "PureLiteral({:?})", String::from_utf8_lossy(lit)),
            SearchStrategy::SingleByte(b) => write!(f, "SingleByte({:?})", *b as char),
            SearchStrategy::TwoBytes(b1, b2) => write!(f, "TwoBytes({:?}, {:?})", *b1 as char, *b2 as char),
            SearchStrategy::ThreeBytes(b1, b2, b3) => write!(f, "ThreeBytes({:?}, {:?}, {:?})", *b1 as char, *b2 as char, *b3 as char),
            SearchStrategy::LiteralPrefix(lit) => write!(f, "LiteralPrefix({:?})", String::from_utf8_lossy(lit)),
            SearchStrategy::AlternationLiterals { literals, .. } => {
                let strs: Vec<_> = literals.iter().map(|l| String::from_utf8_lossy(l).to_string()).collect();
                write!(f, "AlternationLiterals({:?})", strs)
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
    /// The compiled bytecode (heap-allocated)
    bytecode: *mut u8,
    /// The original pattern (for Display)
    pattern: String,
    /// The flags
    flags: Flags,
    /// Optimized search strategy
    strategy: SearchStrategy,
}

// Regex is Send + Sync since the bytecode is immutable after compilation
unsafe impl Send for Regex {}
unsafe impl Sync for Regex {}

impl Regex {
    /// Compile a new regular expression
    pub fn new(pattern: &str) -> Result<Self> {
        Self::with_flags(pattern, Flags::empty())
    }

    /// Compile a new regular expression with flags
    pub fn with_flags(pattern: &str, flags: Flags) -> Result<Self> {
        let mut error_msg = [0i8; 128];
        let mut bytecode_len: i32 = 0;

        // Create a null-terminated copy of the pattern.
        // The lre_compile function expects the buffer to be null-terminated
        // for proper end-of-pattern detection.
        let mut pattern_buf: Vec<u8> = pattern.as_bytes().to_vec();
        pattern_buf.push(0);

        let bytecode = engine::lre_compile(
            &mut bytecode_len,
            error_msg.as_mut_ptr(),
            error_msg.len() as i32,
            pattern_buf.as_ptr() as *const i8,
            pattern.len(),
            flags.bits() as i32,
            ptr::null_mut(),
        );

        if bytecode.is_null() {
            // SAFETY: error_msg was populated by lre_compile with a null-terminated
            // C string on failure. The buffer is valid for reads up to its length.
            let msg = unsafe {
                CStr::from_ptr(error_msg.as_ptr())
                    .to_string_lossy()
                    .into_owned()
            };
            return Err(Error::Syntax(msg));
        }

        // Analyze pattern to determine optimal search strategy
        let strategy = analyze_pattern(pattern, flags);

        Ok(Regex {
            bytecode,
            pattern: pattern.to_string(),
            flags,
            strategy,
        })
    }

    /// Test if the pattern matches anywhere in the text
    pub fn is_match(&self, text: &str) -> bool {
        self.find(text).is_some()
    }

    /// Find the first match in the text
    ///
    /// Uses optimized search strategies based on pattern analysis.
    pub fn find(&self, text: &str) -> Option<Match> {
        let len = text.len();

        // For short inputs, just use the engine directly
        if len < OPTIMIZATION_THRESHOLD {
            return self.try_match_at(text, 0).or_else(|| self.find_at_linear(text, 1));
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
    fn find_pure_literal(&self, text: &str, literal: &[u8]) -> Option<Match> {
        let bytes = text.as_bytes();
        if literal.len() == 1 {
            // Single byte - use memchr directly
            memchr(literal[0], bytes).map(|pos| Match {
                start: pos,
                end: pos + 1,
            })
        } else {
            // Multi-byte - use memmem
            memmem::find(bytes, literal).map(|pos| Match {
                start: pos,
                end: pos + literal.len(),
            })
        }
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
    fn find_with_literal_prefix(&self, text: &str, prefix: &[u8]) -> Option<Match> {
        let finder = memmem::Finder::new(prefix);
        let bytes = text.as_bytes();

        for pos in finder.find_iter(bytes) {
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
        let mut start = 0;

        while start < bytes.len() {
            if let Some(pos) = find_word_char(&bytes[start..]) {
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

            SearchStrategy::PureLiteral(literal) => {
                // Fast literal search using memchr + verify pattern
                // This avoids memmem::Finder creation overhead for repeated calls
                find_literal_fast(&text_bytes[start..], literal)
                    .map(|pos| Match { start: start + pos, end: start + pos + literal.len() })
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
                self.find_at_linear(text, start)
            }
        }
    }

    /// Try to match at an exact position (no scanning).
    /// This is the internal method that runs the interpreter at a specific offset.
    #[inline]
    fn try_match_at(&self, text: &str, pos: usize) -> Option<Match> {
        let text_bytes = text.as_bytes();

        // SAFETY: bytecode is valid from constructor
        let bytecode = unsafe {
            std::slice::from_raw_parts(self.bytecode, self.bytecode_len())
        };

        let mut ctx = interpreter::ExecContext::new(bytecode, text_bytes);

        match ctx.exec(pos) {
            interpreter::ExecResult::Match => {
                // Extract match from captures
                if let (Some(match_start), Some(match_end)) = (
                    ctx.captures.get(0).copied().flatten(),
                    ctx.captures.get(1).copied().flatten()
                ) {
                    Some(Match {
                        start: match_start,
                        end: match_end,
                    })
                } else {
                    None
                }
            }
            interpreter::ExecResult::NoMatch => None,
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
    fn find_at_literal_prefix(&self, text: &str, start: usize, prefix: &[u8]) -> Option<Match> {
        let bytes = &text.as_bytes()[start..];
        let finder = memmem::Finder::new(prefix);

        for pos in finder.find_iter(bytes) {
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
        let mut offset = 0;

        while let Some(pos) = find_word_char(&bytes[offset..]) {
            let abs_pos = start + offset + pos;
            if let Some(m) = self.try_match_at(text, abs_pos) {
                return Some(m);
            }
            offset += pos + 1;
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
    fn find_at_linear(&self, text: &str, start: usize) -> Option<Match> {
        let mut pos = start;
        while pos <= text.len() {
            if let Some(m) = self.try_match_at(text, pos) {
                return Some(m);
            }
            // Advance by one UTF-8 char
            if pos < text.len() {
                pos += text[pos..].chars().next().map(|c| c.len_utf8()).unwrap_or(1);
            } else {
                break;
            }
        }
        None
    }

    /// Find a match using the original C engine (for benchmarking comparison)
    /// This uses the c2rust-transpiled lre_exec function
    #[doc(hidden)]
    pub fn find_at_c_engine(&self, text: &str, start: usize) -> Option<Match> {
        let text_bytes = text.as_bytes();
        let capture_count = self.capture_count();

        // Allocate capture array
        let mut captures: Vec<*mut u8> = vec![std::ptr::null_mut(); capture_count * 2];

        // Call the C engine
        // Note: lre_exec is not marked unsafe despite taking raw pointers (c2rust artifact)
        let ret = engine::lre_exec(
            captures.as_mut_ptr(),
            self.bytecode,
            text_bytes.as_ptr(),
            start as i32,
            text_bytes.len() as i32,
            0, // cbuf_type = 8-bit
            std::ptr::null_mut(), // opaque
        );

        if ret == 1 {
            // Match found - extract positions
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

            Some(Match {
                start: match_start,
                end: match_end,
            })
        } else {
            None
        }
    }

    /// Get bytecode length
    fn bytecode_len(&self) -> usize {
        // SAFETY: bytecode is valid from constructor
        unsafe {
            let header = std::slice::from_raw_parts(self.bytecode, engine::RE_HEADER_LEN as usize);
            let bc_len = engine::lre_get_bytecode_len(header);
            engine::RE_HEADER_LEN as usize + bc_len as usize
        }
    }

    /// Get the number of capture groups (including group 0 for the whole match)
    pub fn capture_count(&self) -> usize {
        // SAFETY: self.bytecode is valid (checked non-null in constructor).
        // We only need RE_HEADER_LEN (8) bytes for the header.
        let header = unsafe {
            std::slice::from_raw_parts(self.bytecode, engine::RE_HEADER_LEN as usize)
        };
        engine::lre_get_capture_count(header) as usize
    }

    /// Get the flags
    pub fn flags(&self) -> Flags {
        self.flags
    }

    /// Get the original pattern
    pub fn pattern(&self) -> &str {
        &self.pattern
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
            SearchStrategy::PureLiteral(literal) => {
                MatchIterator::Literal(LiteralMatches::new(literal.as_slice(), text))
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
                MatchIterator::General(Matches {
                    regex: self,
                    text,
                    last_end: 0,
                    last_was_empty: false,
                })
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
    pub fn captures_at(&self, text: &str, start: usize) -> Option<Captures> {
        let capture_count = self.capture_count();
        let mut capture_ptrs: Vec<*mut u8> = vec![ptr::null_mut(); capture_count * 2];

        let text_bytes = text.as_bytes();
        let char_index = if start == 0 {
            0
        } else {
            text[..start].chars().count() as i32
        };

        let result = engine::lre_exec(
            capture_ptrs.as_mut_ptr(),
            self.bytecode,
            text_bytes.as_ptr(),
            char_index,
            text_bytes.len() as i32,
            0,
            ptr::null_mut(),
        );

        if result != 1 {
            return None;
        }

        let text_start = text_bytes.as_ptr() as usize;
        let mut groups = Vec::with_capacity(capture_count);

        for i in 0..capture_count {
            let start_ptr = capture_ptrs[i * 2];
            let end_ptr = capture_ptrs[i * 2 + 1];

            if start_ptr.is_null() || end_ptr.is_null() {
                groups.push(None);
            } else {
                let match_start = start_ptr as usize - text_start;
                let match_end = end_ptr as usize - text_start;
                groups.push(Some((match_start, match_end)));
            }
        }

        Some(Captures {
            text: text.to_string(),
            groups,
        })
    }
}

impl Drop for Regex {
    fn drop(&mut self) {
        if !self.bytecode.is_null() {
            // SAFETY: bytecode was allocated by lre_compile via lre_realloc (which uses libc::malloc).
            // It has not been freed (Drop only runs once). No other references exist
            // (Regex owns the bytecode exclusively).
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
    General(Matches<'r, 't>),
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
            MatchIterator::General(gen) => gen.next(),
        }
    }
}

/// Fast iterator for pure literal patterns using memmem::find_iter for efficiency
/// FindIter<'h, 'n> where 'h = haystack (text), 'n = needle (literal)
pub struct LiteralMatches<'r, 't> {
    inner: memmem::FindIter<'t, 'r>,
    literal_len: usize,
}

impl<'r, 't> LiteralMatches<'r, 't> {
    /// Create a new literal match iterator
    fn new(literal: &'r [u8], text: &'t str) -> Self {
        Self {
            inner: memmem::find_iter(text.as_bytes(), literal),
            literal_len: literal.len(),
        }
    }
}

impl<'r, 't> Iterator for LiteralMatches<'r, 't> {
    type Item = Match;

    #[inline]
    fn next(&mut self) -> Option<Match> {
        self.inner.next().map(|start| Match {
            start,
            end: start + self.literal_len,
        })
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

/// Analyze a pattern to determine the best search strategy.
fn analyze_pattern(pattern: &str, flags: Flags) -> SearchStrategy {
    // First, check for pure fast-path patterns (no interpreter needed!)
    if !flags.is_ignore_case() {
        if let Some(strategy) = detect_pure_pattern(pattern) {
            return strategy;
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

    // Case-insensitive patterns are tricky - handle alternation of cases
    let case_insensitive = flags.is_ignore_case();

    // Try to extract leading literal(s) or character class
    let mut literals = Vec::new();
    let mut is_pure_literal = true; // Track if we consumed the entire pattern

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

            ')' | '{' | '}' | '$' => {
                is_pure_literal = false;
                break;
            }

            '|' => {
                // Alternation at top level - need to analyze all branches
                // regardless of what literals we've accumulated so far
                return analyze_alternation(pattern, case_insensitive);
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

            // Regular character
            _ if c.is_ascii() => {
                if case_insensitive && c.is_ascii_alphabetic() {
                    // For case-insensitive, first char could be upper or lower
                    is_pure_literal = false;
                    if literals.is_empty() {
                        let lower = c.to_ascii_lowercase() as u8;
                        let upper = c.to_ascii_uppercase() as u8;
                        if lower != upper {
                            return SearchStrategy::TwoBytes(lower, upper);
                        }
                    }
                    break;
                }
                literals.push(c as u8);
            }

            _ => {
                is_pure_literal = false;
                break; // Non-ASCII
            }
        }
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
        1 if is_pure_literal => SearchStrategy::PureLiteral(literals),
        1 => SearchStrategy::SingleByte(literals[0]),
        _ if is_pure_literal => SearchStrategy::PureLiteral(literals),
        _ => SearchStrategy::LiteralPrefix(literals),
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

/// Analyze alternation like foo|bar|baz
/// Returns AlternationLiterals if all alternatives are pure literals (uses Aho-Corasick!)
/// Otherwise extracts first bytes for memchr optimization
fn analyze_alternation(pattern: &str, case_insensitive: bool) -> SearchStrategy {
    // First, try to extract all alternatives as pure literals
    let alternatives: Vec<&str> = split_top_level_alternation(pattern);

    if !case_insensitive && alternatives.len() >= 2 {
        // Check if all alternatives are pure literals
        let mut all_pure = true;
        let mut literals: Vec<Vec<u8>> = Vec::new();

        for alt in &alternatives {
            if let Some(lit) = extract_pure_literal(alt) {
                literals.push(lit);
            } else {
                all_pure = false;
                break;
            }
        }

        if all_pure && !literals.is_empty() {
            // Use Aho-Corasick for multi-pattern matching - BLAZING FAST!
            let ac = AhoCorasick::new(&literals).unwrap();
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
        _ => SearchStrategy::None,
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

static WHITESPACE_BITMAP: ByteBitmap = {
    let mut bits = [0u64; 4];
    bits[0] = (1u64 << 32) | (1u64 << 9) | (1u64 << 10) | (1u64 << 11) | (1u64 << 12) | (1u64 << 13);
    ByteBitmap { bits }
};

/// Find the first digit (0-9) in a byte slice
/// Uses memchr for SIMD acceleration on common digits
#[inline]
fn find_digit(bytes: &[u8]) -> Option<usize> {
    // memchr is SIMD-accelerated and much faster than bitmap checking
    // Search for common digits first
    let mut pos = 0;
    while pos < bytes.len() {
        // Find any of 0-4 (covers most cases)
        if let Some(found) = memchr3(b'0', b'1', b'2', &bytes[pos..]) {
            return Some(pos + found);
        }
        // Also check 3-9 with another memchr3 pass
        if let Some(found) = memchr3(b'3', b'4', b'5', &bytes[pos..]) {
            return Some(pos + found);
        }
        if let Some(found) = memchr3(b'6', b'7', b'8', &bytes[pos..]) {
            return Some(pos + found);
        }
        if let Some(found) = memchr(b'9', &bytes[pos..]) {
            return Some(pos + found);
        }
        break;
    }
    None
}

/// Find the first word character (a-z, A-Z, 0-9, _) in a byte slice
#[inline]
fn find_word_char(bytes: &[u8]) -> Option<usize> {
    // For word chars, bitmap is actually good since we have 63 possible bytes
    WORD_CHAR_BITMAP.find_in_slice(bytes)
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

/// Find a run of consecutive digits [0-9]+ starting at or after `start`
/// Returns the match directly - NO INTERPRETER NEEDED!
#[inline]
fn find_digit_run(bytes: &[u8], start: usize) -> Option<Match> {
    let len = bytes.len();
    let mut pos = start;

    // Find first digit using explicit range check (LLVM can vectorize this)
    // Process 8 bytes at a time for better instruction-level parallelism
    while pos + 8 <= len {
        // Check 8 bytes, looking for any digit
        let b0 = bytes[pos];
        let b1 = bytes[pos + 1];
        let b2 = bytes[pos + 2];
        let b3 = bytes[pos + 3];
        let b4 = bytes[pos + 4];
        let b5 = bytes[pos + 5];
        let b6 = bytes[pos + 6];
        let b7 = bytes[pos + 7];

        // Range check: digit if b'0' <= b && b <= b'9'
        let d0 = b0.wrapping_sub(b'0') <= 9;
        let d1 = b1.wrapping_sub(b'0') <= 9;
        let d2 = b2.wrapping_sub(b'0') <= 9;
        let d3 = b3.wrapping_sub(b'0') <= 9;
        let d4 = b4.wrapping_sub(b'0') <= 9;
        let d5 = b5.wrapping_sub(b'0') <= 9;
        let d6 = b6.wrapping_sub(b'0') <= 9;
        let d7 = b7.wrapping_sub(b'0') <= 9;

        if d0 | d1 | d2 | d3 | d4 | d5 | d6 | d7 {
            // Found at least one digit - find which one
            if d0 { pos += 0; break; }
            if d1 { pos += 1; break; }
            if d2 { pos += 2; break; }
            if d3 { pos += 3; break; }
            if d4 { pos += 4; break; }
            if d5 { pos += 5; break; }
            if d6 { pos += 6; break; }
            pos += 7;
            break;
        }
        pos += 8;
    }

    // Check remaining bytes
    while pos < len && bytes[pos].wrapping_sub(b'0') > 9 {
        pos += 1;
    }

    if pos >= len {
        return None;
    }

    // Found start of digit run at pos
    let match_start = pos;

    // Find end of digit run
    pos += 1;
    while pos < len && bytes[pos].wrapping_sub(b'0') <= 9 {
        pos += 1;
    }

    Some(Match { start: match_start, end: pos })
}

/// Find a run of consecutive lowercase letters [a-z]+ starting at or after `start`
#[inline]
fn find_lower_run(bytes: &[u8], start: usize) -> Option<Match> {
    // Find the first lowercase letter
    let mut pos = start;
    while pos < bytes.len() {
        if bytes[pos].is_ascii_lowercase() {
            // Found start, now find end
            let match_start = pos;
            pos += 1;
            while pos < bytes.len() && bytes[pos].is_ascii_lowercase() {
                pos += 1;
            }
            return Some(Match { start: match_start, end: pos });
        }
        pos += 1;
    }
    None
}

/// Find a run of consecutive uppercase letters [A-Z]+ starting at or after `start`
#[inline]
fn find_upper_run(bytes: &[u8], start: usize) -> Option<Match> {
    let mut pos = start;
    while pos < bytes.len() {
        if bytes[pos].is_ascii_uppercase() {
            let match_start = pos;
            pos += 1;
            while pos < bytes.len() && bytes[pos].is_ascii_uppercase() {
                pos += 1;
            }
            return Some(Match { start: match_start, end: pos });
        }
        pos += 1;
    }
    None
}

/// Find a run of consecutive letters [a-zA-Z]+ starting at or after `start`
#[inline]
fn find_alpha_run(bytes: &[u8], start: usize) -> Option<Match> {
    let mut pos = start;
    while pos < bytes.len() {
        if bytes[pos].is_ascii_alphabetic() {
            let match_start = pos;
            pos += 1;
            while pos < bytes.len() && bytes[pos].is_ascii_alphabetic() {
                pos += 1;
            }
            return Some(Match { start: match_start, end: pos });
        }
        pos += 1;
    }
    None
}

/// Find a run of consecutive alphanumeric chars [a-zA-Z0-9]+ starting at or after `start`
#[inline]
fn find_alnum_run(bytes: &[u8], start: usize) -> Option<Match> {
    let mut pos = start;
    while pos < bytes.len() {
        if bytes[pos].is_ascii_alphanumeric() {
            let match_start = pos;
            pos += 1;
            while pos < bytes.len() && bytes[pos].is_ascii_alphanumeric() {
                pos += 1;
            }
            return Some(Match { start: match_start, end: pos });
        }
        pos += 1;
    }
    None
}

/// Find a run of consecutive word chars [a-zA-Z0-9_]+ or \w+ starting at or after `start`
#[inline]
fn find_word_run(bytes: &[u8], start: usize) -> Option<Match> {
    let slice = &bytes[start..];

    // Find the first word char using bitmap
    let first = WORD_CHAR_BITMAP.find_in_slice(slice)?;
    let abs_start = start + first;

    // Scan forward to find the end of the word run
    let mut end = abs_start + 1;
    while end < bytes.len() {
        let b = bytes[end];
        if b.is_ascii_alphanumeric() || b == b'_' {
            end += 1;
        } else {
            break;
        }
    }

    Some(Match { start: abs_start, end })
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
/// Strategy: search for the suffix, then extend backwards to find lowercase letters
/// Returns the match directly - NO INTERPRETER NEEDED!
///
/// Algorithm: Find lowercase runs and locate the RIGHTMOST occurrence of the suffix.
/// This matches regex semantics of [a-z]+suffix (greedy).
///
/// Example: "singing" with suffix "ing"
/// - Lowercase run: "singing" (positions 0-7)
/// - Rightmost "ing": position 4-7
/// - At least 1 char before? Yes ("sing")
/// - Match: "singing" (0-7)
///
/// Example: "winged" with suffix "ing"
/// - Lowercase run: "winged" (positions 0-6)
/// - Rightmost "ing": position 1-4
/// - At least 1 char before? Yes ("w")
/// - Match: "wing" (0-4)
#[inline]
fn find_lower_suffix(bytes: &[u8], start: usize, suffix: &[u8]) -> Option<Match> {
    let suffix_len = suffix.len();
    let mut pos = start;

    while pos < bytes.len() {
        // Find start of lowercase run
        if !bytes[pos].is_ascii_lowercase() {
            pos += 1;
            continue;
        }

        let run_start = pos;

        // Find end of lowercase run
        while pos < bytes.len() && bytes[pos].is_ascii_lowercase() {
            pos += 1;
        }

        let run_end = pos;
        let run_len = run_end - run_start;

        // Need at least suffix_len + 1 characters for a match
        if run_len <= suffix_len {
            continue;
        }

        // Search for RIGHTMOST occurrence of suffix in this run
        // The rightmost position where suffix can start is (run_end - suffix_len)
        // We need at least 1 char before the suffix, so search from (run_end - suffix_len) down to (run_start + 1)

        // Start checking from the rightmost possible position
        let mut check_pos = run_end - suffix_len;
        while check_pos > run_start {
            if &bytes[check_pos..check_pos + suffix_len] == suffix {
                // Found rightmost suffix at position check_pos
                // Match is from run_start to check_pos + suffix_len
                return Some(Match { start: run_start, end: check_pos + suffix_len });
            }
            check_pos -= 1;
        }
        // Note: we don't check check_pos == run_start because that would mean
        // 0 chars before suffix, which doesn't satisfy [a-z]+ (needs 1+)
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
        // Simple backreference
        let re = Regex::new(r"(a)\1").unwrap();
        assert!(re.is_match("aa"));
        assert!(!re.is_match("ab"));

        // Word backreference
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
            SearchStrategy::PureLiteral(literal) => {
                assert_eq!(literal, b"needle");
            }
            _ => panic!("Expected PureLiteral, got {:?}", strategy),
        }

        // Single char pure literal
        let strategy = analyze_pattern("x", Flags::empty());
        match strategy {
            SearchStrategy::PureLiteral(literal) => {
                assert_eq!(literal, b"x");
            }
            _ => panic!("Expected PureLiteral for single char, got {:?}", strategy),
        }
    }

    #[test]
    fn test_strategy_literal_prefix() {
        let strategy = analyze_pattern("hello.*world", Flags::empty());
        match strategy {
            SearchStrategy::LiteralPrefix(prefix) => {
                assert_eq!(prefix, b"hello");
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

        // Case-insensitive falls back to TwoBytes
        let strategy = analyze_pattern("cat|dog", Flags::from_bits(Flags::IGNORE_CASE));
        assert!(matches!(strategy, SearchStrategy::TwoBytes(_, _)));
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
}
