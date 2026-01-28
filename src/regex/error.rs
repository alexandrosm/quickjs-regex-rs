//! Error types for regex compilation and execution

use std::fmt;

/// Error during regex compilation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Invalid pattern syntax
    Syntax(String),
    /// Invalid flag
    InvalidFlag(char),
    /// Too many capture groups (max 255)
    TooManyCaptures,
    /// Too many registers (max 255)
    TooManyRegisters,
    /// Pattern too large
    PatternTooLarge,
    /// Invalid escape sequence
    InvalidEscape(String),
    /// Invalid character class
    InvalidCharClass(String),
    /// Invalid quantifier
    InvalidQuantifier(String),
    /// Invalid group
    InvalidGroup(String),
    /// Invalid back reference
    InvalidBackReference(u32),
    /// Invalid unicode property
    InvalidUnicodeProperty(String),
    /// Memory allocation failed
    OutOfMemory,
    /// Internal error (should not happen)
    Internal(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Syntax(msg) => write!(f, "syntax error: {}", msg),
            Error::InvalidFlag(c) => write!(f, "invalid flag: '{}'", c),
            Error::TooManyCaptures => write!(f, "too many capture groups (max 255)"),
            Error::TooManyRegisters => write!(f, "too many registers (max 255)"),
            Error::PatternTooLarge => write!(f, "pattern too large"),
            Error::InvalidEscape(msg) => write!(f, "invalid escape: {}", msg),
            Error::InvalidCharClass(msg) => write!(f, "invalid character class: {}", msg),
            Error::InvalidQuantifier(msg) => write!(f, "invalid quantifier: {}", msg),
            Error::InvalidGroup(msg) => write!(f, "invalid group: {}", msg),
            Error::InvalidBackReference(n) => write!(f, "invalid back reference: \\{}", n),
            Error::InvalidUnicodeProperty(msg) => write!(f, "invalid unicode property: {}", msg),
            Error::OutOfMemory => write!(f, "out of memory"),
            Error::Internal(msg) => write!(f, "internal error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

/// Result type for regex operations
pub type Result<T> = std::result::Result<T, Error>;

/// Execution result codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecResult {
    /// Match found
    Match,
    /// No match
    NoMatch,
    /// Memory error during execution
    MemoryError,
    /// Execution timed out
    Timeout,
}

impl ExecResult {
    /// Convert from C-style return value
    pub fn from_i32(value: i32) -> Self {
        match value {
            1 => ExecResult::Match,
            0 => ExecResult::NoMatch,
            -1 => ExecResult::MemoryError,
            -2 => ExecResult::Timeout,
            _ => ExecResult::NoMatch,
        }
    }

    /// Check if this is a match
    pub fn is_match(self) -> bool {
        matches!(self, ExecResult::Match)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::Syntax("unexpected token".to_string());
        assert_eq!(err.to_string(), "syntax error: unexpected token");
    }

    #[test]
    fn test_exec_result() {
        assert!(ExecResult::Match.is_match());
        assert!(!ExecResult::NoMatch.is_match());
        assert!(!ExecResult::MemoryError.is_match());
    }
}
