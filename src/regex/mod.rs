//! High-level regex API
//!
//! This module provides a safe, idiomatic Rust API built on top of
//! the c2rust-translated QuickJS regex engine.

mod opcodes;
mod flags;
mod error;

pub use opcodes::OpCode;
pub use flags::{Flags, InvalidFlag};
pub use error::{Error, Result, ExecResult};

use std::ffi::CStr;
use std::ptr;

use crate::generated::libregexp;

// ============================================================================
// External C callbacks required by the libregexp code
// ============================================================================

/// Memory allocator callback for the regex engine.
/// This is called by the C code through the extern "C" declaration.
/// Uses libc-style malloc/realloc/free semantics.
#[no_mangle]
pub unsafe extern "C" fn lre_realloc(
    _opaque: *mut core::ffi::c_void,
    ptr: *mut core::ffi::c_void,
    size: usize,
) -> *mut core::ffi::c_void {
    if size == 0 {
        // Free
        if !ptr.is_null() {
            libc::free(ptr);
        }
        return ptr::null_mut();
    }

    if ptr.is_null() {
        // Allocate new
        libc::malloc(size)
    } else {
        // Realloc
        libc::realloc(ptr, size)
    }
}

/// Timeout check callback - always returns 0 (no timeout)
/// Can be made configurable in the future for long-running matches
#[no_mangle]
pub unsafe extern "C" fn lre_check_timeout(
    _opaque: *mut core::ffi::c_void,
) -> core::ffi::c_int {
    0 // No timeout
}

/// Stack overflow check callback - always returns 0 (no overflow)
/// Can be made more sophisticated in the future
#[no_mangle]
pub unsafe extern "C" fn lre_check_stack_overflow(
    _opaque: *mut core::ffi::c_void,
    _alloca_size: usize,
) -> core::ffi::c_int {
    0 // No stack overflow
}

// ============================================================================
// Public Regex API
// ============================================================================

/// A compiled regular expression
pub struct Regex {
    /// The compiled bytecode (heap-allocated)
    bytecode: *mut u8,
    /// Length of the bytecode
    bytecode_len: usize,
    /// The original pattern (for Display)
    pattern: String,
    /// The flags
    flags: Flags,
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

        let bytecode = unsafe {
            libregexp::lre_compile(
                &mut bytecode_len,
                error_msg.as_mut_ptr(),
                error_msg.len() as i32,
                pattern_buf.as_ptr() as *const i8,
                pattern.len(),  // Original length without null terminator
                flags.bits() as i32,
                ptr::null_mut(),
            )
        };

        if bytecode.is_null() {
            // Extract error message
            let msg = unsafe {
                CStr::from_ptr(error_msg.as_ptr())
                    .to_string_lossy()
                    .into_owned()
            };
            return Err(Error::Syntax(msg));
        }

        Ok(Regex {
            bytecode,
            bytecode_len: bytecode_len as usize,
            pattern: pattern.to_string(),
            flags,
        })
    }

    /// Test if the pattern matches anywhere in the text
    pub fn is_match(&self, text: &str) -> bool {
        self.find_at(text, 0).is_some()
    }

    /// Find the first match in the text
    pub fn find(&self, text: &str) -> Option<Match> {
        self.find_at(text, 0)
    }

    /// Find a match starting at the given byte offset
    pub fn find_at(&self, text: &str, start: usize) -> Option<Match> {
        let capture_count = self.capture_count();
        // We need 2 pointers per capture (start/end)
        let mut captures: Vec<*mut u8> = vec![ptr::null_mut(); capture_count * 2];

        let text_bytes = text.as_bytes();
        let char_index = if start == 0 {
            0
        } else {
            // Convert byte offset to character index for UTF-8
            text[..start].chars().count() as i32
        };

        let result = unsafe {
            libregexp::lre_exec(
                captures.as_mut_ptr(),
                self.bytecode,
                text_bytes.as_ptr(),
                char_index,
                text_bytes.len() as i32,
                0, // cbuf_type: 0 = UTF-8
                ptr::null_mut(),
            )
        };

        if result == 1 && !captures[0].is_null() && !captures[1].is_null() {
            let start_ptr = captures[0] as usize;
            let end_ptr = captures[1] as usize;
            let text_start = text_bytes.as_ptr() as usize;

            let match_start = start_ptr - text_start;
            let match_end = end_ptr - text_start;

            Some(Match {
                start: match_start,
                end: match_end,
            })
        } else {
            None
        }
    }

    /// Get the number of capture groups (including group 0 for the whole match)
    pub fn capture_count(&self) -> usize {
        unsafe { libregexp::lre_get_capture_count(self.bytecode) as usize }
    }

    /// Get the flags
    pub fn flags(&self) -> Flags {
        self.flags
    }

    /// Get the original pattern
    pub fn pattern(&self) -> &str {
        &self.pattern
    }
}

impl Drop for Regex {
    fn drop(&mut self) {
        if !self.bytecode.is_null() {
            unsafe {
                libc::free(self.bytecode as *mut core::ffi::c_void);
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
}
