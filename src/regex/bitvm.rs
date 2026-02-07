//! Bit-Parallel VM: Wide-Word interpreter for the same bytecode.
//!
//! Instead of maintaining a Vec<Thread>, the state is a [u64; N] bit set
//! where bit I = "thread at PC I is active". All threads advance in
//! parallel via bitwise operations:
//!   - Char match:  state &= char_mask[byte]
//!   - Split:       state |= (state & src_mask) >> shift
//!   - Match:       (state & match_bit) != 0
//!
//! This processes ALL threads in O(N/64) operations per byte instead of
//! O(N) per byte, giving 64x speedup for patterns that fit in the word width.
//!
//! For patterns with >256 NFA states, falls back to the Pike VM.

const RE_HEADER_LEN: usize = 8;

mod op {
    pub const CHAR: u8 = 1;
    pub const CHAR_I: u8 = 2;
    pub const CHAR32: u8 = 3;
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
    pub const SAVE_START: u8 = 19;
    pub const SAVE_END: u8 = 20;
    pub const SAVE_RESET: u8 = 21;
    pub const SET_I32: u8 = 27;
    pub const WORD_BOUNDARY: u8 = 28;
    pub const NOT_WORD_BOUNDARY: u8 = 30;
    pub const RANGE: u8 = 36;
    pub const RANGE_I: u8 = 37;
    pub const RANGE32: u8 = 38;
    pub const LOOP_SPLIT_GOTO_FIRST: u8 = 23;
    pub const LOOP_SPLIT_NEXT_FIRST: u8 = 24;
}

// ============================================================================
// Wide bit set: [u64; WORDS] where WORDS = ceil(num_states / 64)
// ============================================================================

const MAX_WORDS: usize = 16; // 16 * 64 = 1024 states max

/// Fixed-width bit set for NFA state tracking
#[derive(Clone, Copy)]
struct BitState {
    words: [u64; MAX_WORDS],
    num_words: usize,
}

impl BitState {
    fn new(num_states: usize) -> Self {
        BitState {
            words: [0u64; MAX_WORDS],
            num_words: (num_states + 63) / 64,
        }
    }

    #[inline(always)]
    fn set(&mut self, bit: usize) {
        let word = bit / 64;
        let pos = bit % 64;
        if word < self.num_words {
            self.words[word] |= 1u64 << pos;
        }
    }

    #[inline(always)]
    fn get(&self, bit: usize) -> bool {
        let word = bit / 64;
        let pos = bit % 64;
        word < self.num_words && (self.words[word] & (1u64 << pos)) != 0
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.words[..self.num_words].iter().all(|&w| w == 0)
    }

    #[inline(always)]
    fn and_assign(&mut self, other: &BitState) {
        for i in 0..self.num_words {
            self.words[i] &= other.words[i];
        }
    }

    #[inline(always)]
    fn or_assign(&mut self, other: &BitState) {
        for i in 0..self.num_words {
            self.words[i] |= other.words[i];
        }
    }

    #[inline(always)]
    fn clear(&mut self) {
        for i in 0..self.num_words {
            self.words[i] = 0;
        }
    }
}

// ============================================================================
// Compiled bit masks: precomputed at regex compile time
// ============================================================================

/// Pre-compiled masks for the bit-parallel VM.
/// Created once from the bytecode, reused for every match.
pub struct BitVmProgram {
    /// For each byte value (0-255), which consuming states accept it
    char_masks: Vec<BitState>,  // [256]
    /// Which states are the MATCH state
    match_mask: BitState,
    /// Epsilon closure from each state (precomputed)
    epsilon_closure: Vec<BitState>, // [num_states]
    /// Number of NFA states (bytecode PCs that are consuming instructions)
    num_states: usize,
    /// Mapping: NFA state index → bytecode PC
    state_to_pc: Vec<usize>,
    /// Mapping: bytecode PC → NFA state index (or usize::MAX if not a state)
    pc_to_state: Vec<usize>,
    /// Number of words needed
    num_words: usize,
    /// The initial state set (epsilon closure from start)
    initial_state: BitState,
}

impl BitVmProgram {
    /// Compile bytecode into bit masks. Returns None if pattern too large.
    pub fn compile(bytecode: &[u8]) -> Option<Self> {
        let bc_len = u32::from_le_bytes([
            bytecode[4], bytecode[5], bytecode[6], bytecode[7]
        ]) as usize;
        let total_pcs = RE_HEADER_LEN + bc_len;

        // Enumerate consuming (terminal) states
        let mut state_to_pc = Vec::new();
        let mut pc_to_state = vec![usize::MAX; total_pcs + 1];

        let mut pc = RE_HEADER_LEN;
        while pc < total_pcs {
            let opcode = bytecode[pc];
            let is_consuming = matches!(opcode,
                op::CHAR | op::CHAR_I | op::CHAR32 | op::DOT | op::ANY |
                op::SPACE | op::NOT_SPACE | op::RANGE | op::RANGE_I |
                op::RANGE32 | op::MATCH
            );

            if is_consuming {
                let state_idx = state_to_pc.len();
                if state_idx >= MAX_WORDS * 64 {
                    return None; // Too many states
                }
                pc_to_state[pc] = state_idx;
                state_to_pc.push(pc);
            }

            pc += instruction_size(bytecode, pc);
        }

        let num_states = state_to_pc.len();
        if num_states == 0 { return None; }
        let num_words = (num_states + 63) / 64;

        // Build character masks: for each byte, which states accept it
        let mut char_masks = vec![BitState::new(num_states); 256];
        let mut match_mask = BitState::new(num_states);

        for (state_idx, &state_pc) in state_to_pc.iter().enumerate() {
            let opcode = bytecode[state_pc];
            match opcode {
                op::MATCH => {
                    match_mask.set(state_idx);
                }
                op::ANY => {
                    for b in 0..256usize {
                        char_masks[b].set(state_idx);
                    }
                }
                op::DOT => {
                    for b in 0..256usize {
                        if b != 0x0A && b != 0x0D { // exclude \n and \r
                            char_masks[b].set(state_idx);
                        }
                    }
                }
                op::CHAR | op::CHAR_I => {
                    let expected = u16::from_le_bytes([bytecode[state_pc + 1], bytecode[state_pc + 2]]);
                    if (expected as usize) < 256 {
                        char_masks[expected as usize].set(state_idx);
                        if opcode == op::CHAR_I {
                            // Case insensitive: also accept the other case
                            let lower = to_lower(expected as u32);
                            let upper = to_upper(expected as u32);
                            if lower < 256 { char_masks[lower as usize].set(state_idx); }
                            if upper < 256 { char_masks[upper as usize].set(state_idx); }
                        }
                    }
                }
                op::SPACE => {
                    for &b in &[0x09u8, 0x0A, 0x0B, 0x0C, 0x0D, 0x20] {
                        char_masks[b as usize].set(state_idx);
                    }
                }
                op::NOT_SPACE => {
                    for b in 0..256usize {
                        if !matches!(b as u8, 0x09 | 0x0A | 0x0B | 0x0C | 0x0D | 0x20) {
                            char_masks[b].set(state_idx);
                        }
                    }
                }
                op::RANGE | op::RANGE_I => {
                    let pair_count = u16::from_le_bytes([
                        bytecode[state_pc + 1], bytecode[state_pc + 2]
                    ]) as usize;
                    for i in 0..pair_count {
                        let base = state_pc + 3 + i * 4;
                        let lo = u16::from_le_bytes([bytecode[base], bytecode[base + 1]]) as usize;
                        let hi = u16::from_le_bytes([bytecode[base + 2], bytecode[base + 3]]) as usize;
                        for b in lo..=hi.min(255) {
                            char_masks[b].set(state_idx);
                            if opcode == op::RANGE_I {
                                let bl = to_lower(b as u32) as usize;
                                let bu = to_upper(b as u32) as usize;
                                if bl < 256 { char_masks[bl].set(state_idx); }
                                if bu < 256 { char_masks[bu].set(state_idx); }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // Build epsilon closures from each consuming state to the next consuming states
        let mut epsilon_closure = vec![BitState::new(num_states); num_states];
        for (state_idx, &state_pc) in state_to_pc.iter().enumerate() {
            // Follow from the instruction AFTER this consuming state
            let next_pc = state_pc + instruction_size(bytecode, state_pc);
            let mut visited = vec![false; total_pcs + 1];
            let mut stack = vec![next_pc];

            while let Some(epc) = stack.pop() {
                if epc >= total_pcs || visited[epc] { continue; }
                visited[epc] = true;

                let eop = bytecode[epc];
                match eop {
                    op::GOTO => {
                        let offset = i32::from_le_bytes([
                            bytecode[epc + 1], bytecode[epc + 2],
                            bytecode[epc + 3], bytecode[epc + 4],
                        ]);
                        let target = ((epc + 5) as isize + offset as isize) as usize;
                        stack.push(target);
                    }
                    op::SPLIT_GOTO_FIRST | op::SPLIT_NEXT_FIRST => {
                        let offset = i32::from_le_bytes([
                            bytecode[epc + 1], bytecode[epc + 2],
                            bytecode[epc + 3], bytecode[epc + 4],
                        ]);
                        let goto_target = ((epc + 5) as isize + offset as isize) as usize;
                        stack.push(epc + 5);
                        stack.push(goto_target);
                    }
                    op::SAVE_START | op::SAVE_END => { stack.push(epc + 2); }
                    op::SAVE_RESET => { stack.push(epc + 3); }
                    op::SET_I32 => { stack.push(epc + 6); }
                    op::LOOP_SPLIT_GOTO_FIRST | op::LOOP_SPLIT_NEXT_FIRST => {
                        let offset = i32::from_le_bytes([
                            bytecode[epc + 6], bytecode[epc + 7],
                            bytecode[epc + 8], bytecode[epc + 9],
                        ]);
                        let goto_target = ((epc + 10) as isize + offset as isize) as usize;
                        stack.push(epc + 10);
                        stack.push(goto_target);
                    }
                    op::LINE_START | op::LINE_START_M | op::LINE_END | op::LINE_END_M |
                    op::WORD_BOUNDARY | op::NOT_WORD_BOUNDARY => {
                        stack.push(epc + 1);
                    }
                    _ => {
                        // Consuming state — add to epsilon closure
                        if let Some(&target_state) = pc_to_state.get(epc) {
                            if target_state < num_states {
                                epsilon_closure[state_idx].set(target_state);
                            }
                        }
                    }
                }
            }
        }

        // Compute initial state: epsilon closure from bytecode start
        let mut initial_state = BitState::new(num_states);
        {
            let mut visited = vec![false; total_pcs + 1];
            let mut stack = vec![RE_HEADER_LEN];
            while let Some(epc) = stack.pop() {
                if epc >= total_pcs || visited[epc] { continue; }
                visited[epc] = true;
                let eop = bytecode[epc];
                match eop {
                    op::GOTO => {
                        let offset = i32::from_le_bytes([
                            bytecode[epc + 1], bytecode[epc + 2],
                            bytecode[epc + 3], bytecode[epc + 4],
                        ]);
                        stack.push(((epc + 5) as isize + offset as isize) as usize);
                    }
                    op::SPLIT_GOTO_FIRST | op::SPLIT_NEXT_FIRST => {
                        let offset = i32::from_le_bytes([
                            bytecode[epc + 1], bytecode[epc + 2],
                            bytecode[epc + 3], bytecode[epc + 4],
                        ]);
                        stack.push(epc + 5);
                        stack.push(((epc + 5) as isize + offset as isize) as usize);
                    }
                    op::SAVE_START | op::SAVE_END => { stack.push(epc + 2); }
                    op::SAVE_RESET => { stack.push(epc + 3); }
                    op::SET_I32 => { stack.push(epc + 6); }
                    op::LOOP_SPLIT_GOTO_FIRST | op::LOOP_SPLIT_NEXT_FIRST => {
                        let offset = i32::from_le_bytes([
                            bytecode[epc + 6], bytecode[epc + 7],
                            bytecode[epc + 8], bytecode[epc + 9],
                        ]);
                        stack.push(epc + 10);
                        stack.push(((epc + 10) as isize + offset as isize) as usize);
                    }
                    op::LINE_START | op::LINE_START_M | op::LINE_END | op::LINE_END_M |
                    op::WORD_BOUNDARY | op::NOT_WORD_BOUNDARY => {
                        stack.push(epc + 1);
                    }
                    _ => {
                        if let Some(&state) = pc_to_state.get(epc) {
                            if state < num_states {
                                initial_state.set(state);
                            }
                        }
                    }
                }
            }
        }

        Some(BitVmProgram {
            char_masks,
            match_mask,
            epsilon_closure,
            num_states,
            state_to_pc,
            pc_to_state,
            num_words,
            initial_state,
        })
    }

    /// Count all non-overlapping matches in the input. O(N * num_words) time.
    pub fn count_matches(&self, input: &[u8]) -> usize {
        let mut count = 0;
        let mut state = self.initial_state;
        let mut was_matching = false;

        for &byte in input {
            // Step 1: Filter — which active states accept this byte?
            let mut accepted = state;
            accepted.and_assign(&self.char_masks[byte as usize]);

            // Step 2: Advance — compute next states via epsilon closure
            let mut next = BitState::new(self.num_states);
            for word_idx in 0..self.num_words {
                let mut bits = accepted.words[word_idx];
                while bits != 0 {
                    let bit = bits.trailing_zeros() as usize;
                    bits &= bits - 1;
                    let state_idx = word_idx * 64 + bit;
                    if state_idx < self.num_states {
                        next.or_assign(&self.epsilon_closure[state_idx]);
                    }
                }
            }

            // Always keep the initial state alive (prefix loop)
            next.or_assign(&self.initial_state);

            // Step 3: Check for match
            let mut has_match = false;
            for i in 0..self.num_words {
                if (next.words[i] & self.match_mask.words[i]) != 0 {
                    has_match = true;
                    break;
                }
            }

            if has_match {
                was_matching = true;
            } else if was_matching {
                // Match just ended — count it
                count += 1;
                was_matching = false;
                // Reset and re-process this byte from initial state
                let mut re_accepted = self.initial_state;
                re_accepted.and_assign(&self.char_masks[byte as usize]);
                next = BitState::new(self.num_states);
                for word_idx in 0..self.num_words {
                    let mut bits = re_accepted.words[word_idx];
                    while bits != 0 {
                        let bit = bits.trailing_zeros() as usize;
                        bits &= bits - 1;
                        let state_idx = word_idx * 64 + bit;
                        if state_idx < self.num_states {
                            next.or_assign(&self.epsilon_closure[state_idx]);
                        }
                    }
                }
                next.or_assign(&self.initial_state);
            }

            state = next;
        }

        // Count final match if input ends while matching
        if was_matching {
            count += 1;
        }

        count
    }

    /// Check if any match exists in the input.
    pub fn has_match(&self, input: &[u8]) -> bool {
        let mut state = self.initial_state;
        for &byte in input {
            let mut accepted = state;
            accepted.and_assign(&self.char_masks[byte as usize]);

            let mut next = BitState::new(self.num_states);
            for word_idx in 0..self.num_words {
                let mut bits = accepted.words[word_idx];
                while bits != 0 {
                    let bit = bits.trailing_zeros() as usize;
                    bits &= bits - 1;
                    let state_idx = word_idx * 64 + bit;
                    if state_idx < self.num_states {
                        next.or_assign(&self.epsilon_closure[state_idx]);
                    }
                }
            }
            next.or_assign(&self.initial_state);

            for i in 0..self.num_words {
                if (next.words[i] & self.match_mask.words[i]) != 0 {
                    return true;
                }
            }

            state = next;
        }
        false
    }
}

/// Get the size of an instruction at the given PC
fn instruction_size(bytecode: &[u8], pc: usize) -> usize {
    if pc >= bytecode.len() { return 1; }
    match bytecode[pc] {
        1 | 2 => 3,       // Char, CharI
        3 | 4 => 5,       // Char32, Char32I
        5 | 6 | 7 | 8 => 1, // Dot, Any, Space, NotSpace
        9 | 10 | 11 | 12 => 1, // LineStart/End
        13 | 14 | 15 => 5, // Goto, Split
        16 => 1,           // Match
        17 | 18 => 1,      // LookaheadMatch
        19 | 20 => 2,      // SaveStart, SaveEnd
        21 => 3,           // SaveReset
        22 => 6,           // Loop
        23 | 24 => 10,     // LoopSplit
        25 | 26 => 10,     // LoopCheckAdv
        27 => 6,           // SetI32
        28 | 29 | 30 | 31 => 1, // WordBoundary
        32 | 33 | 34 | 35 => {  // BackReference (variable)
            2 + bytecode.get(pc + 1).copied().unwrap_or(0) as usize
        }
        36 | 37 => {       // Range, RangeI
            let n = u16::from_le_bytes([
                bytecode.get(pc + 1).copied().unwrap_or(0),
                bytecode.get(pc + 2).copied().unwrap_or(0),
            ]) as usize;
            3 + n * 4
        }
        38 | 39 => {       // Range32, Range32I
            let n = u16::from_le_bytes([
                bytecode.get(pc + 1).copied().unwrap_or(0),
                bytecode.get(pc + 2).copied().unwrap_or(0),
            ]) as usize;
            3 + n * 8
        }
        40 | 41 => 5,     // Lookahead
        42 | 43 => 2,     // SetCharPos, CheckAdvance
        44 => 1,           // Prev
        45 | 46 => 9,     // SpanAny, SpanDot
        47 => {            // SpanClass
            let n = u16::from_le_bytes([
                bytecode.get(pc + 9).copied().unwrap_or(0),
                bytecode.get(pc + 10).copied().unwrap_or(0),
            ]) as usize;
            11 + n * 4
        }
        _ => 1,
    }
}

fn to_lower(c: u32) -> u32 {
    if c >= 'A' as u32 && c <= 'Z' as u32 { c + 32 } else { c }
}

fn to_upper(c: u32) -> u32 {
    if c >= 'a' as u32 && c <= 'z' as u32 { c - 32 } else { c }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::regex::compiler;
    use crate::regex::Flags;

    fn bit_count(pattern: &str, text: &str) -> usize {
        let bc = compiler::compile_regex(pattern, Flags::empty()).unwrap();
        let prog = BitVmProgram::compile(&bc).expect("pattern should fit in bit VM");
        prog.count_matches(text.as_bytes())
    }

    fn bit_has_match(pattern: &str, text: &str) -> bool {
        let bc = compiler::compile_regex(pattern, Flags::empty()).unwrap();
        let prog = BitVmProgram::compile(&bc).expect("pattern should fit in bit VM");
        prog.has_match(text.as_bytes())
    }

    #[test]
    fn test_literal() {
        assert!(bit_has_match("abc", "xabcx"));
        assert!(!bit_has_match("abc", "abd"));
    }

    #[test]
    fn test_count() {
        assert_eq!(bit_count("ab", "ababab"), 3);
    }

    #[test]
    fn test_alternation() {
        assert!(bit_has_match("cat|dog", "a dog"));
        assert!(!bit_has_match("cat|dog", "cow"));
    }

    #[test]
    fn test_char_class() {
        assert_eq!(bit_count("[0-9]+", "a1b22c333"), 3);
    }

    #[test]
    fn test_dot() {
        assert!(bit_has_match("a.c", "abc"));
        assert!(!bit_has_match("a.c", "a\nc"));
    }

    #[test]
    fn test_no_catastrophic() {
        // Pattern that causes exponential backtracking in backtrackers
        // Bit-parallel handles it in linear time
        let text = "a".repeat(30) + "b";
        assert!(!bit_has_match("(a+)+c", &text));
    }

    #[test]
    fn test_large_alternation() {
        // Simulates noseyparker-like pattern
        let mut parts = Vec::new();
        for i in 0..50 {
            parts.push(format!("word{:02}", i));
        }
        let pattern = parts.join("|");
        assert!(bit_has_match(&pattern, "xword42x"));
        assert_eq!(bit_count(&pattern, "word01 word25 word49"), 3);
    }
}
