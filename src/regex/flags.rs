//! Regex flags
//!
//! Translated from QuickJS libregexp.h
//! Copyright (c) 2017-2018 Fabrice Bellard - MIT License

use std::fmt;

/// Regex flags
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Flags {
    bits: u16,
}

impl Flags {
    /// Global search (g flag)
    pub const GLOBAL: u16 = 1 << 0;
    /// Case-insensitive search (i flag)
    pub const IGNORE_CASE: u16 = 1 << 1;
    /// Multi-line mode (m flag)
    pub const MULTILINE: u16 = 1 << 2;
    /// Dot matches newlines (s flag)
    pub const DOT_ALL: u16 = 1 << 3;
    /// Unicode mode (u flag)
    pub const UNICODE: u16 = 1 << 4;
    /// Sticky mode (y flag)
    pub const STICKY: u16 = 1 << 5;
    /// Indices mode (d flag) - unused by engine, just recorded
    pub const INDICES: u16 = 1 << 6;
    /// Named groups are present
    pub const NAMED_GROUPS: u16 = 1 << 7;
    /// Unicode sets mode (v flag)
    pub const UNICODE_SETS: u16 = 1 << 8;

    /// Create empty flags
    pub const fn empty() -> Self {
        Self { bits: 0 }
    }

    /// Create flags from raw bits
    pub const fn from_bits(bits: u16) -> Self {
        Self { bits }
    }

    /// Get raw bits
    pub const fn bits(self) -> u16 {
        self.bits
    }

    /// Check if a flag is set
    pub const fn contains(self, flag: u16) -> bool {
        (self.bits & flag) != 0
    }

    /// Set a flag
    pub fn insert(&mut self, flag: u16) {
        self.bits |= flag;
    }

    /// Clear a flag
    pub fn remove(&mut self, flag: u16) {
        self.bits &= !flag;
    }

    /// Check if global flag is set
    pub const fn is_global(self) -> bool {
        self.contains(Self::GLOBAL)
    }

    /// Check if ignore case flag is set
    pub const fn is_ignore_case(self) -> bool {
        self.contains(Self::IGNORE_CASE)
    }

    /// Check if multiline flag is set
    pub const fn is_multiline(self) -> bool {
        self.contains(Self::MULTILINE)
    }

    /// Check if dot-all flag is set
    pub const fn is_dot_all(self) -> bool {
        self.contains(Self::DOT_ALL)
    }

    /// Check if unicode flag is set
    pub const fn is_unicode(self) -> bool {
        self.contains(Self::UNICODE)
    }

    /// Check if sticky flag is set
    pub const fn is_sticky(self) -> bool {
        self.contains(Self::STICKY)
    }

    /// Check if unicode sets flag is set
    pub const fn is_unicode_sets(self) -> bool {
        self.contains(Self::UNICODE_SETS)
    }

    /// Parse flags from a string like "gi" or "gim"
    pub fn parse(s: &str) -> Result<Self, InvalidFlag> {
        let mut flags = Self::empty();
        for c in s.chars() {
            let flag = match c {
                'g' => Self::GLOBAL,
                'i' => Self::IGNORE_CASE,
                'm' => Self::MULTILINE,
                's' => Self::DOT_ALL,
                'u' => Self::UNICODE,
                'y' => Self::STICKY,
                'd' => Self::INDICES,
                'v' => Self::UNICODE_SETS,
                _ => return Err(InvalidFlag(c)),
            };
            if flags.contains(flag) {
                return Err(InvalidFlag(c)); // Duplicate flag
            }
            flags.insert(flag);
        }

        // Unicode sets implies unicode
        if flags.is_unicode_sets() {
            flags.insert(Self::UNICODE);
        }

        Ok(flags)
    }
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_global() {
            write!(f, "g")?;
        }
        if self.is_ignore_case() {
            write!(f, "i")?;
        }
        if self.is_multiline() {
            write!(f, "m")?;
        }
        if self.is_dot_all() {
            write!(f, "s")?;
        }
        if self.is_unicode() && !self.is_unicode_sets() {
            write!(f, "u")?;
        }
        if self.is_unicode_sets() {
            write!(f, "v")?;
        }
        if self.is_sticky() {
            write!(f, "y")?;
        }
        if self.contains(Self::INDICES) {
            write!(f, "d")?;
        }
        Ok(())
    }
}

/// Error for invalid flag character
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidFlag(pub char);

impl fmt::Display for InvalidFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid regex flag: '{}'", self.0)
    }
}

impl std::error::Error for InvalidFlag {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_flags() {
        let flags = Flags::empty();
        assert!(!flags.is_global());
        assert!(!flags.is_ignore_case());
        assert!(!flags.is_multiline());
    }

    #[test]
    fn test_parse_flags() {
        let flags = Flags::parse("gi").unwrap();
        assert!(flags.is_global());
        assert!(flags.is_ignore_case());
        assert!(!flags.is_multiline());
    }

    #[test]
    fn test_parse_all_flags() {
        let flags = Flags::parse("gimsuy").unwrap();
        assert!(flags.is_global());
        assert!(flags.is_ignore_case());
        assert!(flags.is_multiline());
        assert!(flags.is_dot_all());
        assert!(flags.is_unicode());
        assert!(flags.is_sticky());
    }

    #[test]
    fn test_invalid_flag() {
        let result = Flags::parse("gix");
        assert!(result.is_err());
    }

    #[test]
    fn test_duplicate_flag() {
        let result = Flags::parse("gg");
        assert!(result.is_err());
    }

    #[test]
    fn test_display() {
        let flags = Flags::parse("gim").unwrap();
        assert_eq!(flags.to_string(), "gim");
    }

    #[test]
    fn test_unicode_sets_implies_unicode() {
        let flags = Flags::parse("v").unwrap();
        assert!(flags.is_unicode());
        assert!(flags.is_unicode_sets());
    }
}
