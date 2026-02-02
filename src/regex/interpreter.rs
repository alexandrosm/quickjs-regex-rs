//! Complete Rust bytecode interpreter for the regex engine.
//!
//! Optimized version with:
//! - Binary search for character class ranges
//! - ASCII fast paths for common opcodes
//! - Precomputed bitmaps for \w, \d, \s
//! - Efficient backtracking with truncate instead of pop loops
//! - Bounds check elimination in release mode
//! - Unicode-aware \w and \b when UNICODE flag is set

use super::unicode::lre_is_id_continue;

// ============================================================================
// Constants
// ============================================================================

const RE_HEADER_FLAGS: usize = 0;
/// UNICODE flag bit (from flags.rs: 1 << 4)
const FLAG_UNICODE: u16 = 0x10;
const RE_HEADER_CAPTURE_COUNT: usize = 2;
const RE_HEADER_REGISTER_COUNT: usize = 3;
const RE_HEADER_BYTECODE_LEN: usize = 4;
const RE_HEADER_LEN: usize = 8;

// ============================================================================
// Precomputed ASCII bitmaps for character classes
// ============================================================================

/// Bitmap for \w: [a-zA-Z0-9_]
/// Bits 0-63 in WORD_CHAR_LO, bits 64-127 in WORD_CHAR_HI
const WORD_CHAR_LO: u64 = 0x03ff_0000_0000_0000; // 0-9: bits 48-57
const WORD_CHAR_HI: u64 = 0x07ff_fffe_87ff_fffe; // A-Z: bits 65-90, _: bit 95, a-z: bits 97-122

/// Bitmap for \d: [0-9]
const DIGIT_CHAR_LO: u64 = 0x03ff_0000_0000_0000; // 0-9: bits 48-57
const DIGIT_CHAR_HI: u64 = 0;

/// Bitmap for \s: whitespace (ASCII subset)
/// Tab(9), LF(10), VT(11), FF(12), CR(13), Space(32)
const SPACE_CHAR_LO: u64 = (1 << 9) | (1 << 10) | (1 << 11) | (1 << 12) | (1 << 13) | (1 << 32);
const SPACE_CHAR_HI: u64 = 0;

#[inline(always)]
fn bitmap_contains(c: u32, lo: u64, hi: u64) -> bool {
    if c < 64 {
        (lo & (1u64 << c)) != 0
    } else if c < 128 {
        (hi & (1u64 << (c - 64))) != 0
    } else {
        false
    }
}

#[inline(always)]
fn is_word_char_fast(c: u32) -> bool {
    bitmap_contains(c, WORD_CHAR_LO, WORD_CHAR_HI)
}

/// Unicode-aware word character check.
/// In Unicode mode, \w matches Unicode ID_Continue characters (letters, digits, connectors).
/// This includes Cyrillic, Greek, CJK, and other scripts.
#[inline(always)]
fn is_word_char_unicode(c: u32) -> bool {
    // Fast path: ASCII word chars
    if c < 128 {
        bitmap_contains(c, WORD_CHAR_LO, WORD_CHAR_HI)
    } else {
        // Unicode word character: ID_Continue property
        lre_is_id_continue(c) != 0
    }
}

/// Check if a RANGE bytecode represents the \w character class.
/// Word char ranges (16-bit format):
/// 0x30-0x39 (0-9), 0x41-0x5A (A-Z), 0x5F-0x5F (_), 0x61-0x7A (a-z)
#[inline(always)]
fn is_word_char_range(data: &[u8], pair_count: usize) -> bool {
    // \w has exactly 4 pairs
    if pair_count != 4 {
        return false;
    }
    // Check the expected ranges (each pair is 4 bytes: low u16, high u16)
    data == [
        0x30, 0x00, 0x39, 0x00,  // 0-9
        0x41, 0x00, 0x5A, 0x00,  // A-Z
        0x5F, 0x00, 0x5F, 0x00,  // _
        0x61, 0x00, 0x7A, 0x00,  // a-z
    ]
}

/// Check if a RANGE32 bytecode represents the \w character class.
/// Word char ranges (32-bit format):
/// 0x30-0x39 (0-9), 0x41-0x5A (A-Z), 0x5F-0x5F (_), 0x61-0x7A (a-z)
#[inline(always)]
fn is_word_char_range32(data: &[u8], pair_count: usize) -> bool {
    // \w has exactly 4 pairs
    if pair_count != 4 {
        return false;
    }
    // Check the expected ranges (each pair is 8 bytes: low u32, high u32)
    data == [
        0x30, 0x00, 0x00, 0x00, 0x39, 0x00, 0x00, 0x00,  // 0-9
        0x41, 0x00, 0x00, 0x00, 0x5A, 0x00, 0x00, 0x00,  // A-Z
        0x5F, 0x00, 0x00, 0x00, 0x5F, 0x00, 0x00, 0x00,  // _
        0x61, 0x00, 0x00, 0x00, 0x7A, 0x00, 0x00, 0x00,  // a-z
    ]
}

#[inline(always)]
fn is_digit_char_fast(c: u32) -> bool {
    bitmap_contains(c, DIGIT_CHAR_LO, DIGIT_CHAR_HI)
}

#[inline(always)]
fn is_space_char_fast(c: u32) -> bool {
    if c < 128 {
        bitmap_contains(c, SPACE_CHAR_LO, SPACE_CHAR_HI)
    } else {
        // Unicode whitespace
        matches!(c, 0xA0 | 0x1680 | 0x2000..=0x200A | 0x2028 | 0x2029 | 0x202F | 0x205F | 0x3000 | 0xFEFF)
    }
}

// ============================================================================
// Opcodes
// ============================================================================

mod op {
    pub const CHAR: u8 = 1;
    pub const CHAR_I: u8 = 2;
    pub const CHAR32: u8 = 3;
    pub const CHAR32_I: u8 = 4;
    pub const DOT: u8 = 5;
    pub const ANY: u8 = 6;
    pub const SPACE: u8 = 7;
    pub const NOT_SPACE: u8 = 8;
    pub const LINE_START: u8 = 9;
    pub const LINE_START_M: u8 = 10;
    pub const LINE_END: u8 = 11;
    pub const LINE_END_M: u8 = 12;
    pub const GOTO: u8 = 13;
    pub const SPLIT_GOTO_FIRST: u8 = 14;
    pub const SPLIT_NEXT_FIRST: u8 = 15;
    pub const MATCH: u8 = 16;
    pub const LOOKAHEAD_MATCH: u8 = 17;
    pub const NEGATIVE_LOOKAHEAD_MATCH: u8 = 18;
    pub const SAVE_START: u8 = 19;
    pub const SAVE_END: u8 = 20;
    pub const SAVE_RESET: u8 = 21;
    pub const LOOP: u8 = 22;
    pub const LOOP_SPLIT_GOTO_FIRST: u8 = 23;
    pub const LOOP_SPLIT_NEXT_FIRST: u8 = 24;
    pub const LOOP_CHECK_ADV_SPLIT_GOTO_FIRST: u8 = 25;
    pub const LOOP_CHECK_ADV_SPLIT_NEXT_FIRST: u8 = 26;
    pub const SET_I32: u8 = 27;
    pub const WORD_BOUNDARY: u8 = 28;
    pub const WORD_BOUNDARY_I: u8 = 29;
    pub const NOT_WORD_BOUNDARY: u8 = 30;
    pub const NOT_WORD_BOUNDARY_I: u8 = 31;
    pub const BACK_REFERENCE: u8 = 32;
    pub const BACK_REFERENCE_I: u8 = 33;
    pub const BACKWARD_BACK_REFERENCE: u8 = 34;
    pub const BACKWARD_BACK_REFERENCE_I: u8 = 35;
    pub const RANGE: u8 = 36;
    pub const RANGE_I: u8 = 37;
    pub const RANGE32: u8 = 38;
    pub const RANGE32_I: u8 = 39;
    pub const LOOKAHEAD: u8 = 40;
    pub const NEGATIVE_LOOKAHEAD: u8 = 41;
    pub const SET_CHAR_POS: u8 = 42;
    pub const CHECK_ADVANCE: u8 = 43;
    pub const PREV: u8 = 44;
}

// ============================================================================
// Execution State
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StateType {
    Split,
    Lookahead,
    NegativeLookahead,
}

#[derive(Clone, Copy, Debug)]
struct StackFrame {
    pc: usize,
    pos: usize,
    capture_save_idx: u32,
    register_save_idx: u32,
    state_type: StateType,
}

/// Result of execution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecResult {
    Match,
    NoMatch,
}

// ============================================================================
// Execution Context
// ============================================================================

pub struct ExecContext<'a> {
    input: &'a [u8],
    input_len: usize,  // Cache length to avoid repeated .len() calls
    bytecode: &'a [u8],

    // Captures: indices into input (start, end pairs)
    pub captures: Vec<Option<usize>>,
    capture_count: usize,

    // Registers for loop counters and char positions
    registers: Vec<usize>,

    // Backtracking stack
    stack: Vec<StackFrame>,

    // Save stacks for backtracking (packed for cache efficiency)
    capture_saves: Vec<(u32, Option<usize>)>,  // u32 index is enough
    register_saves: Vec<(u32, usize)>,

    // Unicode mode flag - affects \w and \b behavior
    unicode_mode: bool,
}

impl<'a> ExecContext<'a> {
    #[inline]
    pub fn new(bytecode: &'a [u8], input: &'a [u8]) -> Self {
        let capture_count = bytecode[RE_HEADER_CAPTURE_COUNT] as usize;
        let register_count = bytecode[RE_HEADER_REGISTER_COUNT] as usize;

        // Read flags from bytecode header (2 bytes at offset 0)
        let flags = u16::from_le_bytes([bytecode[RE_HEADER_FLAGS], bytecode[RE_HEADER_FLAGS + 1]]);
        let unicode_mode = (flags & FLAG_UNICODE) != 0;

        // Estimate stack sizes based on bytecode complexity
        let bc_len = bytecode.len();
        let estimated_stack = (bc_len / 8).max(32).min(256);

        ExecContext {
            input,
            input_len: input.len(),
            bytecode,
            captures: vec![None; capture_count * 2],
            capture_count,
            registers: vec![0; register_count],
            stack: Vec::with_capacity(estimated_stack),
            capture_saves: Vec::with_capacity(estimated_stack),
            register_saves: Vec::with_capacity(estimated_stack / 2),
            unicode_mode,
        }
    }

    /// Reset for reuse at a new position
    #[inline]
    pub fn reset(&mut self) {
        self.captures.fill(None);
        self.registers.fill(0);
        self.stack.clear();
        self.capture_saves.clear();
        self.register_saves.clear();
    }

    // ========================================================================
    // Bytecode reading helpers - with bounds check elimination in release
    // ========================================================================

    #[inline(always)]
    fn read_u8(&self, pc: usize) -> u8 {
        debug_assert!(pc < self.bytecode.len());
        unsafe { *self.bytecode.get_unchecked(pc) }
    }

    #[inline(always)]
    fn read_u16(&self, pc: usize) -> u16 {
        debug_assert!(pc + 1 < self.bytecode.len());
        unsafe {
            let ptr = self.bytecode.as_ptr().add(pc);
            u16::from_le_bytes([*ptr, *ptr.add(1)])
        }
    }

    #[inline(always)]
    fn read_u32(&self, pc: usize) -> u32 {
        debug_assert!(pc + 3 < self.bytecode.len());
        unsafe {
            let ptr = self.bytecode.as_ptr().add(pc);
            u32::from_le_bytes([*ptr, *ptr.add(1), *ptr.add(2), *ptr.add(3)])
        }
    }

    #[inline(always)]
    fn read_i32(&self, pc: usize) -> i32 {
        self.read_u32(pc) as i32
    }

    // ========================================================================
    // UTF-8 helpers - optimized with ASCII fast paths
    // ========================================================================

    /// Decode next UTF-8 char, return (codepoint, new_pos)
    /// ASCII fast path is critical - most text is ASCII
    #[inline(always)]
    fn next_char(&self, pos: usize) -> Option<(u32, usize)> {
        if pos >= self.input_len {
            return None;
        }

        let b0 = unsafe { *self.input.get_unchecked(pos) };

        // ASCII fast path (most common case ~95%+ of text)
        if b0 < 0x80 {
            return Some((b0 as u32, pos + 1));
        }

        self.next_char_multibyte(pos, b0)
    }

    /// Handle multi-byte UTF-8 sequences (cold path)
    #[cold]
    #[inline(never)]
    fn next_char_multibyte(&self, pos: usize, b0: u8) -> Option<(u32, usize)> {
        // 2-byte
        if b0 < 0xE0 {
            if pos + 1 >= self.input_len {
                return Some((b0 as u32, pos + 1));
            }
            let b1 = unsafe { *self.input.get_unchecked(pos + 1) };
            let cp = ((b0 as u32 & 0x1F) << 6) | (b1 as u32 & 0x3F);
            return Some((cp, pos + 2));
        }

        // 3-byte
        if b0 < 0xF0 {
            if pos + 2 >= self.input_len {
                return Some((b0 as u32, pos + 1));
            }
            let b1 = unsafe { *self.input.get_unchecked(pos + 1) };
            let b2 = unsafe { *self.input.get_unchecked(pos + 2) };
            let cp = ((b0 as u32 & 0x0F) << 12) | ((b1 as u32 & 0x3F) << 6) | (b2 as u32 & 0x3F);
            return Some((cp, pos + 3));
        }

        // 4-byte
        if pos + 3 >= self.input_len {
            return Some((b0 as u32, pos + 1));
        }
        let b1 = unsafe { *self.input.get_unchecked(pos + 1) };
        let b2 = unsafe { *self.input.get_unchecked(pos + 2) };
        let b3 = unsafe { *self.input.get_unchecked(pos + 3) };
        let cp = ((b0 as u32 & 0x07) << 18) | ((b1 as u32 & 0x3F) << 12) |
                 ((b2 as u32 & 0x3F) << 6) | (b3 as u32 & 0x3F);
        Some((cp, pos + 4))
    }

    /// Get previous char (for lookbehind, word boundary)
    #[inline(always)]
    fn prev_char(&self, pos: usize) -> Option<(u32, usize)> {
        if pos == 0 {
            return None;
        }

        // ASCII fast path
        let b = unsafe { *self.input.get_unchecked(pos - 1) };
        if b < 0x80 {
            return Some((b as u32, pos - 1));
        }

        self.prev_char_multibyte(pos)
    }

    #[cold]
    #[inline(never)]
    fn prev_char_multibyte(&self, pos: usize) -> Option<(u32, usize)> {
        // Walk back through continuation bytes
        let mut start = pos - 1;
        while start > 0 && (unsafe { *self.input.get_unchecked(start) } & 0xC0) == 0x80 {
            start -= 1;
        }
        self.next_char(start)
    }

    /// Move back one character position
    #[inline(always)]
    fn prev_pos(&self, pos: usize) -> Option<usize> {
        if pos == 0 {
            return None;
        }

        // ASCII fast path
        let b = unsafe { *self.input.get_unchecked(pos - 1) };
        if b < 0x80 {
            return Some(pos - 1);
        }

        let mut p = pos - 1;
        while p > 0 && (unsafe { *self.input.get_unchecked(p) } & 0xC0) == 0x80 {
            p -= 1;
        }
        Some(p)
    }

    // ========================================================================
    // Backtracking - optimized with truncate instead of pop loops
    // ========================================================================

    #[inline(always)]
    fn push_state(&mut self, pc: usize, pos: usize, state_type: StateType) {
        self.stack.push(StackFrame {
            pc,
            pos,
            capture_save_idx: self.capture_saves.len() as u32,
            register_save_idx: self.register_saves.len() as u32,
            state_type,
        });
    }

    #[inline(always)]
    fn save_capture(&mut self, idx: usize) {
        self.capture_saves.push((idx as u32, self.captures[idx]));
    }

    #[inline(always)]
    fn save_register(&mut self, idx: usize) {
        self.register_saves.push((idx as u32, self.registers[idx]));
    }

    /// Restore captures and registers from save point
    #[inline(always)]
    fn restore_state(&mut self, capture_idx: u32, register_idx: u32) {
        // Restore captures in reverse order
        let cap_start = capture_idx as usize;
        for i in (cap_start..self.capture_saves.len()).rev() {
            let (idx, val) = self.capture_saves[i];
            self.captures[idx as usize] = val;
        }
        self.capture_saves.truncate(cap_start);

        // Restore registers in reverse order
        let reg_start = register_idx as usize;
        for i in (reg_start..self.register_saves.len()).rev() {
            let (idx, val) = self.register_saves[i];
            self.registers[idx as usize] = val;
        }
        self.register_saves.truncate(reg_start);
    }

    /// Backtrack to previous state
    #[inline]
    fn backtrack(&mut self) -> Option<(usize, usize)> {
        while let Some(frame) = self.stack.pop() {
            self.restore_state(frame.capture_save_idx, frame.register_save_idx);

            match frame.state_type {
                StateType::Split => {
                    return Some((frame.pc, frame.pos));
                }
                StateType::Lookahead => {
                    // Positive lookahead failed - continue backtracking
                    continue;
                }
                StateType::NegativeLookahead => {
                    // Negative lookahead failed to match = SUCCESS!
                    return Some((frame.pc, frame.pos));
                }
            }
        }
        None
    }

    /// Pop until we find a lookahead frame (for lookahead_match)
    fn pop_to_lookahead(&mut self) -> Option<(usize, usize)> {
        while let Some(frame) = self.stack.pop() {
            self.restore_state(frame.capture_save_idx, frame.register_save_idx);

            if frame.state_type == StateType::Lookahead {
                return Some((frame.pc, frame.pos));
            }
        }
        None
    }

    /// Pop until we find a negative lookahead frame
    fn pop_to_negative_lookahead(&mut self) -> Option<(usize, usize)> {
        while let Some(frame) = self.stack.pop() {
            self.restore_state(frame.capture_save_idx, frame.register_save_idx);

            if frame.state_type == StateType::NegativeLookahead {
                return Some((frame.pc, frame.pos));
            }
        }
        None
    }

    // ========================================================================
    // Main execution loop
    // ========================================================================

    pub fn exec(&mut self, start_pos: usize) -> ExecResult {
        let mut pc = RE_HEADER_LEN;
        let mut pos = start_pos;

        // Main dispatch loop
        loop {
            let opcode = self.read_u8(pc);
            pc += 1;

            // Opcodes ordered roughly by frequency for better branch prediction
            match opcode {
                // ============================================================
                // CHAR (1) - Match 16-bit char - MOST COMMON
                // ============================================================
                op::CHAR => {
                    let expected = self.read_u16(pc);
                    pc += 2;

                    // ASCII fast path (most common)
                    if expected < 128 && pos < self.input_len {
                        let b = unsafe { *self.input.get_unchecked(pos) };
                        if b == expected as u8 {
                            pos += 1;
                            continue;
                        }
                        // ASCII mismatch - backtrack
                        if b < 0x80 {
                            if let Some((p, s)) = self.backtrack() {
                                pc = p; pos = s; continue;
                            }
                            return ExecResult::NoMatch;
                        }
                    }

                    // Full UTF-8 path
                    if let Some((c, new_pos)) = self.next_char(pos) {
                        if c == expected as u32 {
                            pos = new_pos;
                            continue;
                        }
                    }
                    if let Some((p, s)) = self.backtrack() {
                        pc = p; pos = s; continue;
                    }
                    return ExecResult::NoMatch;
                }

                // ============================================================
                // DOT (5) - Match any except line terminator
                // ============================================================
                op::DOT => {
                    if pos < self.input_len {
                        let b = unsafe { *self.input.get_unchecked(pos) };
                        // ASCII fast path - check for \n and \r
                        if b < 0x80 {
                            if b != b'\n' && b != b'\r' {
                                pos += 1;
                                continue;
                            }
                            // Line terminator - backtrack
                            if let Some((p, s)) = self.backtrack() {
                                pc = p; pos = s; continue;
                            }
                            return ExecResult::NoMatch;
                        }
                    }

                    // Full UTF-8 path
                    if let Some((c, new_pos)) = self.next_char(pos) {
                        if !is_line_terminator(c) {
                            pos = new_pos;
                            continue;
                        }
                    }
                    if let Some((p, s)) = self.backtrack() {
                        pc = p; pos = s; continue;
                    }
                    return ExecResult::NoMatch;
                }

                // ============================================================
                // MATCH (16) - Success!
                // ============================================================
                op::MATCH => {
                    return ExecResult::Match;
                }

                // ============================================================
                // SPLIT_GOTO_FIRST (14) - Try jump first, backtrack to next
                // ============================================================
                op::SPLIT_GOTO_FIRST => {
                    let offset = self.read_i32(pc);
                    pc += 4;
                    self.push_state(pc, pos, StateType::Split);
                    pc = (pc as isize).wrapping_add(offset as isize) as usize;
                }

                // ============================================================
                // SPLIT_NEXT_FIRST (15) - Try next first, backtrack to jump
                // ============================================================
                op::SPLIT_NEXT_FIRST => {
                    let offset = self.read_i32(pc);
                    pc += 4;
                    let alt_pc = (pc as isize).wrapping_add(offset as isize) as usize;
                    self.push_state(alt_pc, pos, StateType::Split);
                }

                // ============================================================
                // SAVE_START (19) - Save capture start
                // ============================================================
                op::SAVE_START => {
                    let idx = self.read_u8(pc) as usize;
                    pc += 1;
                    let cap_idx = idx * 2;
                    self.save_capture(cap_idx);
                    self.captures[cap_idx] = Some(pos);
                }

                // ============================================================
                // SAVE_END (20) - Save capture end
                // ============================================================
                op::SAVE_END => {
                    let idx = self.read_u8(pc) as usize;
                    pc += 1;
                    let cap_idx = idx * 2 + 1;
                    self.save_capture(cap_idx);
                    self.captures[cap_idx] = Some(pos);
                }

                // ============================================================
                // RANGE (36) - Character class [a-z]
                // ============================================================
                op::RANGE => {
                    let pair_count = self.read_u16(pc) as usize;
                    pc += 2;
                    let data_start = pc;
                    let data_len = pair_count * 4;
                    pc += data_len;

                    if let Some((c, new_pos)) = self.next_char(pos) {
                        // In Unicode mode, check for \w pattern and use Unicode-aware matching
                        let range_data = &self.bytecode[data_start..data_start + data_len];
                        let matched = if self.unicode_mode && is_word_char_range(range_data, pair_count) {
                            is_word_char_unicode(c)
                        } else {
                            check_range16_binary(c, range_data, pair_count)
                        };
                        if matched {
                            pos = new_pos;
                            continue;
                        }
                    }
                    if let Some((p, s)) = self.backtrack() {
                        pc = p; pos = s; continue;
                    }
                    return ExecResult::NoMatch;
                }

                // ============================================================
                // CHAR_I (2) - Match 16-bit char case-insensitive
                // ============================================================
                op::CHAR_I => {
                    let expected = self.read_u16(pc) as u32;
                    pc += 2;

                    if let Some((c, new_pos)) = self.next_char(pos) {
                        if char_eq_ignore_case_fast(c, expected) {
                            pos = new_pos;
                            continue;
                        }
                    }
                    if let Some((p, s)) = self.backtrack() {
                        pc = p; pos = s; continue;
                    }
                    return ExecResult::NoMatch;
                }

                // ============================================================
                // CHAR32 (3) - Match 32-bit char
                // ============================================================
                op::CHAR32 => {
                    let expected = self.read_u32(pc);
                    pc += 4;

                    if let Some((c, new_pos)) = self.next_char(pos) {
                        if c == expected {
                            pos = new_pos;
                            continue;
                        }
                    }
                    if let Some((p, s)) = self.backtrack() {
                        pc = p; pos = s; continue;
                    }
                    return ExecResult::NoMatch;
                }

                // ============================================================
                // CHAR32_I (4) - Match 32-bit char case-insensitive
                // ============================================================
                op::CHAR32_I => {
                    let expected = self.read_u32(pc);
                    pc += 4;

                    if let Some((c, new_pos)) = self.next_char(pos) {
                        if char_eq_ignore_case_fast(c, expected) {
                            pos = new_pos;
                            continue;
                        }
                    }
                    if let Some((p, s)) = self.backtrack() {
                        pc = p; pos = s; continue;
                    }
                    return ExecResult::NoMatch;
                }

                // ============================================================
                // ANY (6) - Match any char (dotall mode)
                // ============================================================
                op::ANY => {
                    if let Some((_, new_pos)) = self.next_char(pos) {
                        pos = new_pos;
                        continue;
                    }
                    if let Some((p, s)) = self.backtrack() {
                        pc = p; pos = s; continue;
                    }
                    return ExecResult::NoMatch;
                }

                // ============================================================
                // SPACE (7) - Match whitespace \s
                // ============================================================
                op::SPACE => {
                    if let Some((c, new_pos)) = self.next_char(pos) {
                        if is_space_char_fast(c) {
                            pos = new_pos;
                            continue;
                        }
                    }
                    if let Some((p, s)) = self.backtrack() {
                        pc = p; pos = s; continue;
                    }
                    return ExecResult::NoMatch;
                }

                // ============================================================
                // NOT_SPACE (8) - Match non-whitespace \S
                // ============================================================
                op::NOT_SPACE => {
                    if let Some((c, new_pos)) = self.next_char(pos) {
                        if !is_space_char_fast(c) {
                            pos = new_pos;
                            continue;
                        }
                    }
                    if let Some((p, s)) = self.backtrack() {
                        pc = p; pos = s; continue;
                    }
                    return ExecResult::NoMatch;
                }

                // ============================================================
                // LINE_START (9) - ^ anchor
                // ============================================================
                op::LINE_START => {
                    if pos == 0 {
                        continue;
                    }
                    if let Some((p, s)) = self.backtrack() {
                        pc = p; pos = s; continue;
                    }
                    return ExecResult::NoMatch;
                }

                // ============================================================
                // LINE_START_M (10) - ^ anchor multiline
                // ============================================================
                op::LINE_START_M => {
                    if pos == 0 {
                        continue;
                    }
                    if let Some((c, _)) = self.prev_char(pos) {
                        if is_line_terminator(c) {
                            continue;
                        }
                    }
                    if let Some((p, s)) = self.backtrack() {
                        pc = p; pos = s; continue;
                    }
                    return ExecResult::NoMatch;
                }

                // ============================================================
                // LINE_END (11) - $ anchor
                // ============================================================
                op::LINE_END => {
                    if pos == self.input_len {
                        continue;
                    }
                    if let Some((p, s)) = self.backtrack() {
                        pc = p; pos = s; continue;
                    }
                    return ExecResult::NoMatch;
                }

                // ============================================================
                // LINE_END_M (12) - $ anchor multiline
                // ============================================================
                op::LINE_END_M => {
                    if pos == self.input_len {
                        continue;
                    }
                    if let Some((c, _)) = self.next_char(pos) {
                        if is_line_terminator(c) {
                            continue;
                        }
                    }
                    if let Some((p, s)) = self.backtrack() {
                        pc = p; pos = s; continue;
                    }
                    return ExecResult::NoMatch;
                }

                // ============================================================
                // GOTO (13) - Unconditional jump
                // ============================================================
                op::GOTO => {
                    let offset = self.read_i32(pc);
                    pc = ((pc + 4) as isize).wrapping_add(offset as isize) as usize;
                }

                // ============================================================
                // LOOKAHEAD_MATCH (17) - Positive lookahead succeeded
                // ============================================================
                op::LOOKAHEAD_MATCH => {
                    if let Some((cont_pc, saved_pos)) = self.pop_to_lookahead() {
                        pc = cont_pc;
                        pos = saved_pos;
                        continue;
                    }
                    return ExecResult::NoMatch;
                }

                // ============================================================
                // NEGATIVE_LOOKAHEAD_MATCH (18) - Inside negative lookahead matched (fail)
                // ============================================================
                op::NEGATIVE_LOOKAHEAD_MATCH => {
                    if let Some((p, s)) = self.pop_to_negative_lookahead() {
                        pc = p;
                        pos = s;
                        if let Some((p2, s2)) = self.backtrack() {
                            pc = p2; pos = s2; continue;
                        }
                    }
                    return ExecResult::NoMatch;
                }

                // ============================================================
                // SAVE_RESET (21) - Reset captures in range
                // ============================================================
                op::SAVE_RESET => {
                    let start_idx = self.read_u8(pc) as usize;
                    let end_idx = self.read_u8(pc + 1) as usize;
                    pc += 2;

                    for idx in start_idx..=end_idx {
                        let cap_start = idx * 2;
                        let cap_end = idx * 2 + 1;
                        self.save_capture(cap_start);
                        self.save_capture(cap_end);
                        self.captures[cap_start] = None;
                        self.captures[cap_end] = None;
                    }
                }

                // ============================================================
                // LOOP (22) - Decrement counter, then jump if result != 0
                // ============================================================
                op::LOOP => {
                    let reg_idx = self.read_u8(pc) as usize;
                    let offset = self.read_i32(pc + 1);
                    pc += 5;

                    if reg_idx < self.registers.len() {
                        let count = self.registers[reg_idx];
                        // IMPORTANT: Decrement first, THEN check if result != 0
                        // This matches the C engine semantics
                        let new_count = count.wrapping_sub(1);
                        self.save_register(reg_idx);
                        self.registers[reg_idx] = new_count;
                        if new_count != 0 {
                            pc = (pc as isize).wrapping_add(offset as isize) as usize;
                            continue;
                        }
                    }
                }

                // ============================================================
                // LOOP_SPLIT_GOTO_FIRST (23) - Loop with greedy preference
                // ============================================================
                op::LOOP_SPLIT_GOTO_FIRST => {
                    let reg_idx = self.read_u8(pc) as usize;
                    let limit = self.read_u32(pc + 1) as usize;
                    let offset = self.read_i32(pc + 5);
                    pc += 9;

                    if reg_idx < self.registers.len() {
                        let count = self.registers[reg_idx];
                        let new_count = count.saturating_sub(1);
                        self.save_register(reg_idx);
                        self.registers[reg_idx] = new_count;

                        if new_count > limit {
                            pc = (pc as isize).wrapping_add(offset as isize) as usize;
                            continue;
                        }
                        if new_count == 0 {
                            continue;
                        }
                        self.push_state(pc, pos, StateType::Split);
                        pc = (pc as isize).wrapping_add(offset as isize) as usize;
                    }
                }

                // ============================================================
                // LOOP_SPLIT_NEXT_FIRST (24) - Loop with lazy preference
                // ============================================================
                op::LOOP_SPLIT_NEXT_FIRST => {
                    let reg_idx = self.read_u8(pc) as usize;
                    let limit = self.read_u32(pc + 1) as usize;
                    let offset = self.read_i32(pc + 5);
                    pc += 9;

                    if reg_idx < self.registers.len() {
                        let count = self.registers[reg_idx];
                        let new_count = count.saturating_sub(1);
                        self.save_register(reg_idx);
                        self.registers[reg_idx] = new_count;

                        if new_count > limit {
                            pc = (pc as isize).wrapping_add(offset as isize) as usize;
                            continue;
                        }
                        if new_count == 0 {
                            continue;
                        }
                        let alt_pc = (pc as isize).wrapping_add(offset as isize) as usize;
                        self.push_state(alt_pc, pos, StateType::Split);
                    }
                }

                // ============================================================
                // LOOP_CHECK_ADV_SPLIT_GOTO_FIRST (25)
                // ============================================================
                op::LOOP_CHECK_ADV_SPLIT_GOTO_FIRST => {
                    let reg_idx = self.read_u8(pc) as usize;
                    let limit = self.read_u32(pc + 1) as usize;
                    let offset = self.read_i32(pc + 5);
                    pc += 9;

                    if reg_idx < self.registers.len() {
                        let count = self.registers[reg_idx];
                        let new_count = count.saturating_sub(1);
                        self.save_register(reg_idx);
                        self.registers[reg_idx] = new_count;

                        if new_count > limit {
                            pc = (pc as isize).wrapping_add(offset as isize) as usize;
                            continue;
                        }
                        if new_count == 0 {
                            continue;
                        }
                        self.push_state(pc, pos, StateType::Split);
                        pc = (pc as isize).wrapping_add(offset as isize) as usize;
                    }
                }

                // ============================================================
                // LOOP_CHECK_ADV_SPLIT_NEXT_FIRST (26)
                // ============================================================
                op::LOOP_CHECK_ADV_SPLIT_NEXT_FIRST => {
                    let reg_idx = self.read_u8(pc) as usize;
                    let limit = self.read_u32(pc + 1) as usize;
                    let offset = self.read_i32(pc + 5);
                    pc += 9;

                    if reg_idx < self.registers.len() {
                        let count = self.registers[reg_idx];
                        let new_count = count.saturating_sub(1);
                        self.save_register(reg_idx);
                        self.registers[reg_idx] = new_count;

                        if new_count > limit {
                            pc = (pc as isize).wrapping_add(offset as isize) as usize;
                            continue;
                        }
                        if new_count == 0 {
                            continue;
                        }
                        let alt_pc = (pc as isize).wrapping_add(offset as isize) as usize;
                        self.push_state(alt_pc, pos, StateType::Split);
                    }
                }

                // ============================================================
                // SET_I32 (27) - Set register to value
                // ============================================================
                op::SET_I32 => {
                    let reg_idx = self.read_u8(pc) as usize;
                    let value = self.read_u32(pc + 1) as usize;
                    pc += 5;

                    if reg_idx < self.registers.len() {
                        self.save_register(reg_idx);
                        self.registers[reg_idx] = value;
                    }
                }

                // ============================================================
                // WORD_BOUNDARY (28) - \b
                // ============================================================
                op::WORD_BOUNDARY => {
                    let (prev_word, next_word) = if self.unicode_mode {
                        (self.prev_char(pos).map_or(false, |(c, _)| is_word_char_unicode(c)),
                         self.next_char(pos).map_or(false, |(c, _)| is_word_char_unicode(c)))
                    } else {
                        (self.prev_char(pos).map_or(false, |(c, _)| is_word_char_fast(c)),
                         self.next_char(pos).map_or(false, |(c, _)| is_word_char_fast(c)))
                    };

                    if prev_word != next_word {
                        continue;
                    }
                    if let Some((p, s)) = self.backtrack() {
                        pc = p; pos = s; continue;
                    }
                    return ExecResult::NoMatch;
                }

                // ============================================================
                // WORD_BOUNDARY_I (29) - \b case-insensitive
                // ============================================================
                op::WORD_BOUNDARY_I => {
                    let (prev_word, next_word) = if self.unicode_mode {
                        (self.prev_char(pos).map_or(false, |(c, _)| is_word_char_unicode(c)),
                         self.next_char(pos).map_or(false, |(c, _)| is_word_char_unicode(c)))
                    } else {
                        (self.prev_char(pos).map_or(false, |(c, _)| is_word_char_fast(c)),
                         self.next_char(pos).map_or(false, |(c, _)| is_word_char_fast(c)))
                    };

                    if prev_word != next_word {
                        continue;
                    }
                    if let Some((p, s)) = self.backtrack() {
                        pc = p; pos = s; continue;
                    }
                    return ExecResult::NoMatch;
                }

                // ============================================================
                // NOT_WORD_BOUNDARY (30) - \B
                // ============================================================
                op::NOT_WORD_BOUNDARY => {
                    let (prev_word, next_word) = if self.unicode_mode {
                        (self.prev_char(pos).map_or(false, |(c, _)| is_word_char_unicode(c)),
                         self.next_char(pos).map_or(false, |(c, _)| is_word_char_unicode(c)))
                    } else {
                        (self.prev_char(pos).map_or(false, |(c, _)| is_word_char_fast(c)),
                         self.next_char(pos).map_or(false, |(c, _)| is_word_char_fast(c)))
                    };

                    if prev_word == next_word {
                        continue;
                    }
                    if let Some((p, s)) = self.backtrack() {
                        pc = p; pos = s; continue;
                    }
                    return ExecResult::NoMatch;
                }

                // ============================================================
                // NOT_WORD_BOUNDARY_I (31) - \B case-insensitive
                // ============================================================
                op::NOT_WORD_BOUNDARY_I => {
                    let (prev_word, next_word) = if self.unicode_mode {
                        (self.prev_char(pos).map_or(false, |(c, _)| is_word_char_unicode(c)),
                         self.next_char(pos).map_or(false, |(c, _)| is_word_char_unicode(c)))
                    } else {
                        (self.prev_char(pos).map_or(false, |(c, _)| is_word_char_fast(c)),
                         self.next_char(pos).map_or(false, |(c, _)| is_word_char_fast(c)))
                    };

                    if prev_word == next_word {
                        continue;
                    }
                    if let Some((p, s)) = self.backtrack() {
                        pc = p; pos = s; continue;
                    }
                    return ExecResult::NoMatch;
                }

                // ============================================================
                // BACK_REFERENCE (32) - \1, \2, etc.
                // ============================================================
                op::BACK_REFERENCE => {
                    let count = self.read_u8(pc) as usize;
                    pc += 1;
                    let groups_start = pc;
                    pc += count;

                    let mut matched = true;
                    for i in 0..count {
                        let group_idx = self.bytecode[groups_start + i] as usize;
                        let start = self.captures.get(group_idx * 2).copied().flatten();
                        let end = self.captures.get(group_idx * 2 + 1).copied().flatten();

                        if let (Some(s), Some(e)) = (start, end) {
                            let captured = &self.input[s..e];
                            let remaining = &self.input[pos..];

                            if remaining.starts_with(captured) {
                                pos += captured.len();
                            } else {
                                matched = false;
                                break;
                            }
                        }
                    }

                    if matched {
                        continue;
                    }
                    if let Some((p, s)) = self.backtrack() {
                        pc = p; pos = s; continue;
                    }
                    return ExecResult::NoMatch;
                }

                // ============================================================
                // BACK_REFERENCE_I (33) - case-insensitive
                // ============================================================
                op::BACK_REFERENCE_I => {
                    let count = self.read_u8(pc) as usize;
                    pc += 1;
                    let groups_start = pc;
                    pc += count;

                    let mut matched = true;
                    for i in 0..count {
                        let group_idx = self.bytecode[groups_start + i] as usize;
                        let start = self.captures.get(group_idx * 2).copied().flatten();
                        let end = self.captures.get(group_idx * 2 + 1).copied().flatten();

                        if let (Some(s), Some(e)) = (start, end) {
                            let captured = &self.input[s..e];

                            if match_slice_ignore_case(&self.input[pos..], captured) {
                                pos += captured.len();
                            } else {
                                matched = false;
                                break;
                            }
                        }
                    }

                    if matched {
                        continue;
                    }
                    if let Some((p, s)) = self.backtrack() {
                        pc = p; pos = s; continue;
                    }
                    return ExecResult::NoMatch;
                }

                // ============================================================
                // BACKWARD_BACK_REFERENCE (34)
                // ============================================================
                op::BACKWARD_BACK_REFERENCE => {
                    let count = self.read_u8(pc) as usize;
                    pc += 1;
                    let groups_start = pc;
                    pc += count;

                    let mut matched = true;
                    for i in (0..count).rev() {
                        let group_idx = self.bytecode[groups_start + i] as usize;
                        let start = self.captures.get(group_idx * 2).copied().flatten();
                        let end = self.captures.get(group_idx * 2 + 1).copied().flatten();

                        if let (Some(s), Some(e)) = (start, end) {
                            let captured = &self.input[s..e];
                            let cap_len = captured.len();

                            if pos >= cap_len && &self.input[pos - cap_len..pos] == captured {
                                pos -= cap_len;
                            } else {
                                matched = false;
                                break;
                            }
                        }
                    }

                    if matched {
                        continue;
                    }
                    if let Some((p, s)) = self.backtrack() {
                        pc = p; pos = s; continue;
                    }
                    return ExecResult::NoMatch;
                }

                // ============================================================
                // BACKWARD_BACK_REFERENCE_I (35)
                // ============================================================
                op::BACKWARD_BACK_REFERENCE_I => {
                    let count = self.read_u8(pc) as usize;
                    pc += 1;
                    let groups_start = pc;
                    pc += count;

                    let mut matched = true;
                    for i in (0..count).rev() {
                        let group_idx = self.bytecode[groups_start + i] as usize;
                        let start = self.captures.get(group_idx * 2).copied().flatten();
                        let end = self.captures.get(group_idx * 2 + 1).copied().flatten();

                        if let (Some(s), Some(e)) = (start, end) {
                            let captured = &self.input[s..e];
                            let cap_len = captured.len();

                            if pos >= cap_len && match_slice_ignore_case(&self.input[pos - cap_len..pos], captured) {
                                pos -= cap_len;
                            } else {
                                matched = false;
                                break;
                            }
                        }
                    }

                    if matched {
                        continue;
                    }
                    if let Some((p, s)) = self.backtrack() {
                        pc = p; pos = s; continue;
                    }
                    return ExecResult::NoMatch;
                }

                // ============================================================
                // RANGE_I (37) - Character class case-insensitive
                // ============================================================
                op::RANGE_I => {
                    let pair_count = self.read_u16(pc) as usize;
                    pc += 2;
                    let data_start = pc;
                    let data_len = pair_count * 4;
                    pc += data_len;

                    if let Some((c, new_pos)) = self.next_char(pos) {
                        let range_data = &self.bytecode[data_start..data_start + data_len];
                        // In Unicode mode, check for \w pattern and use Unicode-aware matching
                        let matched = if self.unicode_mode && is_word_char_range(range_data, pair_count) {
                            is_word_char_unicode(c)
                        } else {
                            let c_lower = to_lower_fast(c);
                            check_range16_binary(c_lower, range_data, pair_count)
                        };
                        if matched {
                            pos = new_pos;
                            continue;
                        }
                    }
                    if let Some((p, s)) = self.backtrack() {
                        pc = p; pos = s; continue;
                    }
                    return ExecResult::NoMatch;
                }

                // ============================================================
                // RANGE32 (38) - Character class 32-bit ranges
                // ============================================================
                op::RANGE32 => {
                    let pair_count = self.read_u16(pc) as usize;
                    pc += 2;
                    let data_start = pc;
                    let data_len = pair_count * 8;
                    pc += data_len;

                    if let Some((c, new_pos)) = self.next_char(pos) {
                        let range_data = &self.bytecode[data_start..data_start + data_len];
                        // In Unicode mode, check for \w pattern and use Unicode-aware matching
                        let matched = if self.unicode_mode && is_word_char_range32(range_data, pair_count) {
                            is_word_char_unicode(c)
                        } else {
                            check_range32_binary(c, range_data, pair_count)
                        };
                        if matched {
                            pos = new_pos;
                            continue;
                        }
                    }
                    if let Some((p, s)) = self.backtrack() {
                        pc = p; pos = s; continue;
                    }
                    return ExecResult::NoMatch;
                }

                // ============================================================
                // RANGE32_I (39) - Character class 32-bit case-insensitive
                // ============================================================
                op::RANGE32_I => {
                    let pair_count = self.read_u16(pc) as usize;
                    pc += 2;
                    let data_start = pc;
                    let data_len = pair_count * 8;
                    pc += data_len;

                    if let Some((c, new_pos)) = self.next_char(pos) {
                        let range_data = &self.bytecode[data_start..data_start + data_len];
                        // In Unicode mode, check for \w pattern and use Unicode-aware matching
                        let matched = if self.unicode_mode && is_word_char_range32(range_data, pair_count) {
                            is_word_char_unicode(c)
                        } else {
                            let c_lower = to_lower_fast(c);
                            check_range32_binary(c_lower, range_data, pair_count)
                        };
                        if matched {
                            pos = new_pos;
                            continue;
                        }
                    }
                    if let Some((p, s)) = self.backtrack() {
                        pc = p; pos = s; continue;
                    }
                    return ExecResult::NoMatch;
                }

                // ============================================================
                // LOOKAHEAD (40) - Positive lookahead (?=...)
                // ============================================================
                op::LOOKAHEAD => {
                    let offset = self.read_i32(pc);
                    pc += 4;
                    let cont_pc = (pc as isize).wrapping_add(offset as isize) as usize;
                    self.push_state(cont_pc, pos, StateType::Lookahead);
                }

                // ============================================================
                // NEGATIVE_LOOKAHEAD (41) - Negative lookahead (?!...)
                // ============================================================
                op::NEGATIVE_LOOKAHEAD => {
                    let offset = self.read_i32(pc);
                    pc += 4;
                    let cont_pc = (pc as isize).wrapping_add(offset as isize) as usize;
                    self.push_state(cont_pc, pos, StateType::NegativeLookahead);
                }

                // ============================================================
                // SET_CHAR_POS (42) - Set character position register
                // ============================================================
                op::SET_CHAR_POS => {
                    let reg_idx = self.read_u8(pc) as usize;
                    pc += 1;

                    let actual_idx = self.capture_count * 2 + reg_idx;
                    if actual_idx < self.registers.len() {
                        self.save_register(actual_idx);
                        self.registers[actual_idx] = pos;
                    }
                }

                // ============================================================
                // CHECK_ADVANCE (43) - Check if position advanced
                // ============================================================
                op::CHECK_ADVANCE => {
                    let reg_idx = self.read_u8(pc) as usize;
                    pc += 1;

                    let actual_idx = self.capture_count * 2 + reg_idx;
                    if actual_idx < self.registers.len() {
                        let last_pos = self.registers[actual_idx];
                        if pos == last_pos {
                            if let Some((p, s)) = self.backtrack() {
                                pc = p; pos = s; continue;
                            }
                            return ExecResult::NoMatch;
                        }
                    }
                }

                // ============================================================
                // PREV (44) - Move back one character
                // ============================================================
                op::PREV => {
                    if let Some(new_pos) = self.prev_pos(pos) {
                        pos = new_pos;
                        continue;
                    }
                    if let Some((p, s)) = self.backtrack() {
                        pc = p; pos = s; continue;
                    }
                    return ExecResult::NoMatch;
                }

                // ============================================================
                // Unknown opcode
                // ============================================================
                _ => {
                    #[cfg(debug_assertions)]
                    panic!("Unknown opcode: {} at pc={}", opcode, pc - 1);

                    #[cfg(not(debug_assertions))]
                    return ExecResult::NoMatch;
                }
            }
        }
    }
}

// ============================================================================
// Character helpers - optimized
// ============================================================================

#[inline(always)]
fn is_line_terminator(c: u32) -> bool {
    // Ordered by frequency: \n is most common
    c == 0x0A || c == 0x0D || c == 0x2028 || c == 0x2029
}

#[inline(always)]
fn to_lower_fast(c: u32) -> u32 {
    // ASCII fast path
    if c >= b'A' as u32 && c <= b'Z' as u32 {
        c + 32
    } else {
        c
    }
}

#[inline(always)]
fn char_eq_ignore_case_fast(a: u32, b: u32) -> bool {
    if a == b {
        return true;
    }
    // Both must be ASCII letters for case folding to apply
    let a_lower = to_lower_fast(a);
    let b_lower = to_lower_fast(b);
    a_lower == b_lower && a_lower >= b'a' as u32 && a_lower <= b'z' as u32
}

/// Check if slice matches another slice case-insensitively (ASCII only)
#[inline]
fn match_slice_ignore_case(haystack: &[u8], needle: &[u8]) -> bool {
    if haystack.len() < needle.len() {
        return false;
    }
    for i in 0..needle.len() {
        let h = unsafe { *haystack.get_unchecked(i) };
        let n = unsafe { *needle.get_unchecked(i) };
        let h_lower = if h >= b'A' && h <= b'Z' { h + 32 } else { h };
        let n_lower = if n >= b'A' && n <= b'Z' { n + 32 } else { n };
        if h_lower != n_lower {
            return false;
        }
    }
    true
}

/// Check 16-bit range table with binary search for large tables
#[inline]
fn check_range16_binary(c: u32, data: &[u8], pair_count: usize) -> bool {
    // Linear search for small tables (better cache behavior)
    if pair_count <= 4 {
        for i in 0..pair_count {
            let base = i * 4;
            let lo = unsafe {
                u16::from_le_bytes([*data.get_unchecked(base), *data.get_unchecked(base + 1)]) as u32
            };
            let hi = unsafe {
                u16::from_le_bytes([*data.get_unchecked(base + 2), *data.get_unchecked(base + 3)]) as u32
            };
            if c >= lo && c <= hi {
                return true;
            }
        }
        return false;
    }

    // Binary search for larger tables
    let mut left = 0;
    let mut right = pair_count;

    while left < right {
        let mid = left + (right - left) / 2;
        let base = mid * 4;
        let lo = unsafe {
            u16::from_le_bytes([*data.get_unchecked(base), *data.get_unchecked(base + 1)]) as u32
        };
        let hi = unsafe {
            u16::from_le_bytes([*data.get_unchecked(base + 2), *data.get_unchecked(base + 3)]) as u32
        };

        if c < lo {
            right = mid;
        } else if c > hi {
            left = mid + 1;
        } else {
            return true;
        }
    }
    false
}

/// Check 32-bit range table with binary search for large tables
#[inline]
fn check_range32_binary(c: u32, data: &[u8], pair_count: usize) -> bool {
    // Linear search for small tables
    if pair_count <= 4 {
        for i in 0..pair_count {
            let base = i * 8;
            let lo = unsafe {
                u32::from_le_bytes([
                    *data.get_unchecked(base),
                    *data.get_unchecked(base + 1),
                    *data.get_unchecked(base + 2),
                    *data.get_unchecked(base + 3),
                ])
            };
            let hi = unsafe {
                u32::from_le_bytes([
                    *data.get_unchecked(base + 4),
                    *data.get_unchecked(base + 5),
                    *data.get_unchecked(base + 6),
                    *data.get_unchecked(base + 7),
                ])
            };
            if c >= lo && c <= hi {
                return true;
            }
        }
        return false;
    }

    // Binary search for larger tables
    let mut left = 0;
    let mut right = pair_count;

    while left < right {
        let mid = left + (right - left) / 2;
        let base = mid * 8;
        let lo = unsafe {
            u32::from_le_bytes([
                *data.get_unchecked(base),
                *data.get_unchecked(base + 1),
                *data.get_unchecked(base + 2),
                *data.get_unchecked(base + 3),
            ])
        };
        let hi = unsafe {
            u32::from_le_bytes([
                *data.get_unchecked(base + 4),
                *data.get_unchecked(base + 5),
                *data.get_unchecked(base + 6),
                *data.get_unchecked(base + 7),
            ])
        };

        if c < lo {
            right = mid;
        } else if c > hi {
            left = mid + 1;
        } else {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_helpers() {
        // Test bitmap-based functions
        assert!(is_space_char_fast(0x20));
        assert!(is_space_char_fast(0x09));
        assert!(!is_space_char_fast(b'a' as u32));

        assert!(is_word_char_fast(b'a' as u32));
        assert!(is_word_char_fast(b'Z' as u32));
        assert!(is_word_char_fast(b'5' as u32));
        assert!(is_word_char_fast(b'_' as u32));
        assert!(!is_word_char_fast(b'-' as u32));

        assert!(is_digit_char_fast(b'0' as u32));
        assert!(is_digit_char_fast(b'9' as u32));
        assert!(!is_digit_char_fast(b'a' as u32));

        assert!(is_line_terminator(0x0A));
        assert!(!is_line_terminator(b'x' as u32));

        assert_eq!(to_lower_fast(b'A' as u32), b'a' as u32);
        assert_eq!(to_lower_fast(b'z' as u32), b'z' as u32);

        assert!(char_eq_ignore_case_fast(b'A' as u32, b'a' as u32));
    }

    #[test]
    fn test_match_slice_ignore_case() {
        assert!(match_slice_ignore_case(b"Hello", b"hello"));
        assert!(match_slice_ignore_case(b"HELLO", b"hello"));
        assert!(!match_slice_ignore_case(b"Hello", b"world"));
    }

    #[test]
    fn test_bitmap_contains() {
        // Test word char bitmap
        assert!(bitmap_contains(b'a' as u32, WORD_CHAR_LO, WORD_CHAR_HI));
        assert!(bitmap_contains(b'z' as u32, WORD_CHAR_LO, WORD_CHAR_HI));
        assert!(bitmap_contains(b'A' as u32, WORD_CHAR_LO, WORD_CHAR_HI));
        assert!(bitmap_contains(b'Z' as u32, WORD_CHAR_LO, WORD_CHAR_HI));
        assert!(bitmap_contains(b'0' as u32, WORD_CHAR_LO, WORD_CHAR_HI));
        assert!(bitmap_contains(b'9' as u32, WORD_CHAR_LO, WORD_CHAR_HI));
        assert!(bitmap_contains(b'_' as u32, WORD_CHAR_LO, WORD_CHAR_HI));
        assert!(!bitmap_contains(b'-' as u32, WORD_CHAR_LO, WORD_CHAR_HI));
        assert!(!bitmap_contains(b' ' as u32, WORD_CHAR_LO, WORD_CHAR_HI));
    }

    #[test]
    fn test_range_binary_search() {
        // Create a range table for [a-z] (97-122)
        let mut data = vec![];
        data.extend_from_slice(&97u16.to_le_bytes());
        data.extend_from_slice(&122u16.to_le_bytes());

        assert!(check_range16_binary(b'a' as u32, &data, 1));
        assert!(check_range16_binary(b'm' as u32, &data, 1));
        assert!(check_range16_binary(b'z' as u32, &data, 1));
        assert!(!check_range16_binary(b'A' as u32, &data, 1));
        assert!(!check_range16_binary(b'0' as u32, &data, 1));
    }
}
