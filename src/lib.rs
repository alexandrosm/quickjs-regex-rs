//! # quickjs-regex
//!
//! A JavaScript-compatible regular expression engine translated from QuickJS.
//!
//! This crate provides a regex engine that is fully compatible with
//! ECMAScript regex semantics, translated from Fabrice Bellard's QuickJS.
//!
//! ## Features
//!
//! - Full ECMAScript regex compatibility
//! - Unicode support (when `u` flag is enabled)
//! - Named capture groups
//! - Lookahead and lookbehind assertions
//! - All standard flags: `g`, `i`, `m`, `s`, `u`, `y`
//!
//! ## Quick Start
//!
//! ```rust
//! use quickjs_regex::Regex;
//!
//! // Simple matching
//! let re = Regex::new(r"\d+").unwrap();
//! assert!(re.is_match("hello123world"));
//!
//! // Find a match
//! let m = re.find("hello123world").unwrap();
//! assert_eq!(m.as_str("hello123world"), "123");
//!
//! // Find all matches
//! let re = Regex::new(r"\w+").unwrap();
//! let matches: Vec<_> = re.find_iter("one two three").collect();
//! assert_eq!(matches.len(), 3);
//!
//! // Capture groups
//! let re = Regex::new(r"(\w+)@(\w+)\.(\w+)").unwrap();
//! let caps = re.captures("user@example.com").unwrap();
//! assert_eq!(caps.get_str(1), Some("user"));
//! assert_eq!(caps.get_str(2), Some("example"));
//! assert_eq!(caps.get_str(3), Some("com"));
//! ```
//!
//! ## Flags
//!
//! ```rust
//! use quickjs_regex::{Regex, Flags};
//!
//! // Case-insensitive matching
//! let re = Regex::with_flags("hello", Flags::from_bits(Flags::IGNORE_CASE)).unwrap();
//! assert!(re.is_match("HELLO"));
//!
//! // Multiple flags
//! let flags = Flags::from_bits(Flags::GLOBAL | Flags::IGNORE_CASE | Flags::MULTILINE);
//! let re = Regex::with_flags("^test", flags).unwrap();
//! ```
//!
//! ## Error Handling
//!
//! ```rust
//! use quickjs_regex::Regex;
//!
//! // Invalid patterns return an error
//! let result = Regex::new("(unclosed");
//! assert!(result.is_err());
//! ```

mod regex;

// Re-export the public API
pub use regex::*;
