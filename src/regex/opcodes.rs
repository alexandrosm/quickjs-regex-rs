//! Regex bytecode opcodes
//!
//! Translated from QuickJS libregexp-opcode.h
//! Copyright (c) 2017-2018 Fabrice Bellard - MIT License

/// Regex bytecode opcodes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OpCode {
    /// Never used (invalid opcode)
    Invalid = 0,
    /// Match a single character (2 bytes: char low, char high)
    Char = 1,
    /// Match a single character case-insensitive
    CharI = 2,
    /// Match a 32-bit character (4 bytes)
    Char32 = 3,
    /// Match a 32-bit character case-insensitive
    Char32I = 4,
    /// Match any character except line terminators
    Dot = 5,
    /// Match any character including line terminators
    Any = 6,
    /// Match whitespace \s
    Space = 7,
    /// Match non-whitespace \S
    NotSpace = 8,
    /// Match start of line (non-multiline)
    LineStart = 9,
    /// Match start of line (multiline mode)
    LineStartM = 10,
    /// Match end of line (non-multiline)
    LineEnd = 11,
    /// Match end of line (multiline mode)
    LineEndM = 12,
    /// Unconditional jump
    Goto = 13,
    /// Split: try goto first, then next
    SplitGotoFirst = 14,
    /// Split: try next first, then goto
    SplitNextFirst = 15,
    /// Successful match
    Match = 16,
    /// Lookahead match success
    LookaheadMatch = 17,
    /// Negative lookahead match success
    NegativeLookaheadMatch = 18,
    /// Save start position of capture group
    SaveStart = 19,
    /// Save end position of capture group
    SaveEnd = 20,
    /// Reset save positions for a range of capture groups
    SaveReset = 21,
    /// Loop: decrement counter and goto if != 0
    Loop = 22,
    /// Loop then split (goto first)
    LoopSplitGotoFirst = 23,
    /// Loop then split (next first)
    LoopSplitNextFirst = 24,
    /// Loop with advance check then split (goto first)
    LoopCheckAdvSplitGotoFirst = 25,
    /// Loop with advance check then split (next first)
    LoopCheckAdvSplitNextFirst = 26,
    /// Store immediate value to register
    SetI32 = 27,
    /// Match word boundary \b
    WordBoundary = 28,
    /// Match word boundary case-insensitive
    WordBoundaryI = 29,
    /// Match non-word boundary \B
    NotWordBoundary = 30,
    /// Match non-word boundary case-insensitive
    NotWordBoundaryI = 31,
    /// Back reference \1, \2, etc.
    BackReference = 32,
    /// Back reference case-insensitive
    BackReferenceI = 33,
    /// Backward back reference (for lookbehind)
    BackwardBackReference = 34,
    /// Backward back reference case-insensitive
    BackwardBackReferenceI = 35,
    /// Character range [a-z]
    Range = 36,
    /// Character range case-insensitive
    RangeI = 37,
    /// Character range for 32-bit chars
    Range32 = 38,
    /// Character range for 32-bit chars case-insensitive
    Range32I = 39,
    /// Positive lookahead (?=...)
    Lookahead = 40,
    /// Negative lookahead (?!...)
    NegativeLookahead = 41,
    /// Store character position to register
    SetCharPos = 42,
    /// Check that register differs from current position
    CheckAdvance = 43,
    /// Go to previous character (for lookbehind)
    Prev = 44,
}

impl OpCode {
    /// Get the base size of an opcode in bytes (not including variable-length data)
    pub const fn size(self) -> usize {
        match self {
            OpCode::Invalid => 1,
            OpCode::Char => 3,
            OpCode::CharI => 3,
            OpCode::Char32 => 5,
            OpCode::Char32I => 5,
            OpCode::Dot => 1,
            OpCode::Any => 1,
            OpCode::Space => 1,
            OpCode::NotSpace => 1,
            OpCode::LineStart => 1,
            OpCode::LineStartM => 1,
            OpCode::LineEnd => 1,
            OpCode::LineEndM => 1,
            OpCode::Goto => 5,
            OpCode::SplitGotoFirst => 5,
            OpCode::SplitNextFirst => 5,
            OpCode::Match => 1,
            OpCode::LookaheadMatch => 1,
            OpCode::NegativeLookaheadMatch => 1,
            OpCode::SaveStart => 2,
            OpCode::SaveEnd => 2,
            OpCode::SaveReset => 3,
            OpCode::Loop => 6,
            OpCode::LoopSplitGotoFirst => 10,
            OpCode::LoopSplitNextFirst => 10,
            OpCode::LoopCheckAdvSplitGotoFirst => 10,
            OpCode::LoopCheckAdvSplitNextFirst => 10,
            OpCode::SetI32 => 6,
            OpCode::WordBoundary => 1,
            OpCode::WordBoundaryI => 1,
            OpCode::NotWordBoundary => 1,
            OpCode::NotWordBoundaryI => 1,
            OpCode::BackReference => 2,
            OpCode::BackReferenceI => 2,
            OpCode::BackwardBackReference => 2,
            OpCode::BackwardBackReferenceI => 2,
            OpCode::Range => 3,      // variable length
            OpCode::RangeI => 3,     // variable length
            OpCode::Range32 => 3,    // variable length
            OpCode::Range32I => 3,   // variable length
            OpCode::Lookahead => 5,
            OpCode::NegativeLookahead => 5,
            OpCode::SetCharPos => 2,
            OpCode::CheckAdvance => 2,
            OpCode::Prev => 1,
        }
    }

    /// Convert from u8 to OpCode
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(OpCode::Invalid),
            1 => Some(OpCode::Char),
            2 => Some(OpCode::CharI),
            3 => Some(OpCode::Char32),
            4 => Some(OpCode::Char32I),
            5 => Some(OpCode::Dot),
            6 => Some(OpCode::Any),
            7 => Some(OpCode::Space),
            8 => Some(OpCode::NotSpace),
            9 => Some(OpCode::LineStart),
            10 => Some(OpCode::LineStartM),
            11 => Some(OpCode::LineEnd),
            12 => Some(OpCode::LineEndM),
            13 => Some(OpCode::Goto),
            14 => Some(OpCode::SplitGotoFirst),
            15 => Some(OpCode::SplitNextFirst),
            16 => Some(OpCode::Match),
            17 => Some(OpCode::LookaheadMatch),
            18 => Some(OpCode::NegativeLookaheadMatch),
            19 => Some(OpCode::SaveStart),
            20 => Some(OpCode::SaveEnd),
            21 => Some(OpCode::SaveReset),
            22 => Some(OpCode::Loop),
            23 => Some(OpCode::LoopSplitGotoFirst),
            24 => Some(OpCode::LoopSplitNextFirst),
            25 => Some(OpCode::LoopCheckAdvSplitGotoFirst),
            26 => Some(OpCode::LoopCheckAdvSplitNextFirst),
            27 => Some(OpCode::SetI32),
            28 => Some(OpCode::WordBoundary),
            29 => Some(OpCode::WordBoundaryI),
            30 => Some(OpCode::NotWordBoundary),
            31 => Some(OpCode::NotWordBoundaryI),
            32 => Some(OpCode::BackReference),
            33 => Some(OpCode::BackReferenceI),
            34 => Some(OpCode::BackwardBackReference),
            35 => Some(OpCode::BackwardBackReferenceI),
            36 => Some(OpCode::Range),
            37 => Some(OpCode::RangeI),
            38 => Some(OpCode::Range32),
            39 => Some(OpCode::Range32I),
            40 => Some(OpCode::Lookahead),
            41 => Some(OpCode::NegativeLookahead),
            42 => Some(OpCode::SetCharPos),
            43 => Some(OpCode::CheckAdvance),
            44 => Some(OpCode::Prev),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_values() {
        assert_eq!(OpCode::Invalid as u8, 0);
        assert_eq!(OpCode::Char as u8, 1);
        assert_eq!(OpCode::Match as u8, 16);
        assert_eq!(OpCode::Prev as u8, 44);
    }

    #[test]
    fn test_opcode_from_u8() {
        assert_eq!(OpCode::from_u8(0), Some(OpCode::Invalid));
        assert_eq!(OpCode::from_u8(16), Some(OpCode::Match));
        assert_eq!(OpCode::from_u8(44), Some(OpCode::Prev));
        assert_eq!(OpCode::from_u8(45), None);
        assert_eq!(OpCode::from_u8(255), None);
    }

    #[test]
    fn test_opcode_sizes() {
        assert_eq!(OpCode::Invalid.size(), 1);
        assert_eq!(OpCode::Char.size(), 3);
        assert_eq!(OpCode::Goto.size(), 5);
        assert_eq!(OpCode::LoopSplitGotoFirst.size(), 10);
    }
}
