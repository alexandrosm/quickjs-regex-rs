//! Ordered Pike VM: thread-list regex execution with JS greedy/lazy semantics.
//!
//! Unlike a standard Pike VM (leftmost-longest), this implementation maintains
//! strict thread priority ordering to match JavaScript/PCRE semantics:
//! - Greedy: SPLIT inserts Loop before Exit (prefer longer match)
//! - Lazy: SPLIT inserts Exit before Loop (prefer shorter match)
//!
//! Thread priority is implicit in list position. Lower index = higher priority.
//! When two threads reach the same state, the higher-priority one shadows the
//! lower-priority one (deduplicated via sparse set). This guarantees O(n*m)
//! time while preserving greedy/lazy behavior.
//!
//! Match resolution uses "shadowing": if a low-priority thread matches while
//! a higher-priority thread is still alive, the match is saved as a candidate
//! but execution continues. Only when the matching thread IS the highest
//! priority, or all higher-priority threads die, does the match finalize.

const RE_HEADER_LEN: usize = 8;
const RE_HEADER_FLAGS: usize = 0;
const RE_HEADER_CAPTURE_COUNT: usize = 2;
const FLAG_UNICODE: u16 = 0x10;

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
    pub const SAVE_START: u8 = 19;
    pub const SAVE_END: u8 = 20;
    pub const SAVE_RESET: u8 = 21;
    pub const LOOP_SPLIT_GOTO_FIRST: u8 = 23;
    pub const LOOP_SPLIT_NEXT_FIRST: u8 = 24;
    pub const SET_I32: u8 = 27;
    pub const WORD_BOUNDARY: u8 = 28;
    pub const NOT_WORD_BOUNDARY: u8 = 30;
    pub const RANGE: u8 = 36;
    pub const RANGE_I: u8 = 37;
    pub const RANGE32: u8 = 38;
    pub const RANGE32_I: u8 = 39;
    pub const SPAN_ANY: u8 = 45;
    pub const SPAN_DOT: u8 = 46;
    pub const SPAN_CLASS: u8 = 47;
}

// ============================================================================
// Ordered thread list with sparse-set deduplication
// ============================================================================

/// Thread list maintaining strict priority order.
/// Index 0 = highest priority. Sparse set prevents duplicate states.
/// Uses generational indexing: `seen` stores the generation when each PC was
/// last added. Checking `seen[pc] == generation` replaces a boolean lookup.
/// `clear()` increments the generation in O(1) instead of touching every entry.
struct ThreadList {
    /// Ordered list of (pc, slot_index) pairs
    threads: Vec<(u32, u32)>,
    /// Generational sparse set: pc → generation when last seen
    seen: Vec<u32>,
    /// Current generation (incremented on clear)
    generation: u32,
    /// Capture slots: each thread has capture_stride slots
    slots: Vec<Option<usize>>,
    /// Register values per thread
    regs: Vec<usize>,
    capture_stride: usize,
    reg_stride: usize,
    slot_cursor: u32,
}

impl ThreadList {
    fn new(num_pcs: usize, capture_count: usize, register_count: usize) -> Self {
        let cs = capture_count * 2;
        let rs = register_count;
        // Pre-allocate enough slots for worst case (every PC active)
        let max_threads = num_pcs.min(4096);
        ThreadList {
            threads: Vec::with_capacity(max_threads),
            seen: vec![0; num_pcs],
            generation: 1, // Start at 1 so initial 0-filled seen is "unseen"
            slots: vec![None; max_threads * cs],
            regs: vec![0; max_threads * rs],
            capture_stride: cs,
            reg_stride: rs,
            slot_cursor: 0,
        }
    }

    /// O(1) clear via generation bump. No memory touched.
    #[inline]
    fn clear(&mut self) {
        self.threads.clear();
        self.slot_cursor = 0;
        self.generation = self.generation.wrapping_add(1);
        if self.generation == 0 { self.generation = 1; } // skip 0 (init value)
    }

    /// Add a thread. Returns false if state already occupied (shadowed).
    #[inline]
    fn add(&mut self, pc: u32, caps: &[Option<usize>], regs: &[usize]) -> bool {
        let pc_idx = pc as usize;
        if pc_idx >= self.seen.len() || self.seen[pc_idx] == self.generation {
            return false; // Shadowed by higher-priority thread
        }
        self.seen[pc_idx] = self.generation;

        let slot_idx = self.slot_cursor;
        self.slot_cursor += 1;
        let si = slot_idx as usize;

        // Ensure capacity
        let cs = self.capture_stride;
        let needed = (si + 1) * cs;
        if needed > self.slots.len() {
            self.slots.resize(needed * 2, None);
        }
        let base = si * cs;
        self.slots[base..base + cs].copy_from_slice(&caps[..cs.min(caps.len())]);

        let rs = self.reg_stride;
        if rs > 0 {
            let r_needed = (si + 1) * rs;
            if r_needed > self.regs.len() {
                self.regs.resize(r_needed * 2, 0);
            }
            let r_base = si * rs;
            let copy_len = rs.min(regs.len());
            self.regs[r_base..r_base + copy_len].copy_from_slice(&regs[..copy_len]);
        }

        self.threads.push((pc, slot_idx));
        true
    }

    fn get_caps(&self, slot_idx: u32) -> &[Option<usize>] {
        let base = slot_idx as usize * self.capture_stride;
        &self.slots[base..base + self.capture_stride]
    }

    fn get_regs(&self, slot_idx: u32) -> &[usize] {
        if self.reg_stride == 0 { return &[]; }
        let base = slot_idx as usize * self.reg_stride;
        &self.regs[base..base + self.reg_stride]
    }
}

// ============================================================================
// Epsilon closure frame
// ============================================================================

enum EpsFrame {
    Explore(usize),
    RestoreCapture(usize, Option<usize>),
    RestoreRegister(usize, usize),
}

// ============================================================================
// Pike VM
// ============================================================================

/// Lazy DFA: assigns integer IDs to state sets and caches transitions.
/// Each byte lookup is O(1) on cache hits: `transitions[state_id][byte]`.
/// On cache misses, computes the transition via Pike VM and stores it.
const MAX_DFA_STATES: usize = 8192;

pub struct LazyDfa {
    /// State set → state ID mapping
    state_map: std::collections::HashMap<Vec<u32>, u32>,
    /// Transition table: state_id × byte → next state_id (None = not yet computed)
    transitions: Vec<[Option<u32>; 256]>,
    /// State sets by ID (for epsilon closure on cache miss)
    state_sets: Vec<Vec<u32>>,
    /// Which states contain MATCH
    has_match: Vec<bool>,
    next_id: u32,
}

impl LazyDfa {
    pub fn new() -> Self {
        LazyDfa {
            state_map: std::collections::HashMap::new(),
            transitions: Vec::new(),
            state_sets: Vec::new(),
            has_match: Vec::new(),
            next_id: 0,
        }
    }

    /// Get or create a state ID for a given state set
    fn get_or_create_state(&mut self, states: &[u32], contains_match: bool) -> Option<u32> {
        if let Some(&id) = self.state_map.get(states) {
            return Some(id);
        }
        if self.next_id as usize >= MAX_DFA_STATES {
            return None; // Cache full
        }
        let id = self.next_id;
        self.next_id += 1;
        self.state_map.insert(states.to_vec(), id);
        self.state_sets.push(states.to_vec());
        self.transitions.push([None; 256]);
        self.has_match.push(contains_match);
        Some(id)
    }

    /// Look up a transition. Returns None if not cached yet.
    #[inline]
    fn lookup(&self, state_id: u32, byte: u8) -> Option<u32> {
        self.transitions.get(state_id as usize)
            .and_then(|t| t[byte as usize])
    }

    /// Store a transition
    fn store(&mut self, state_id: u32, byte: u8, next_state_id: u32) {
        if let Some(t) = self.transitions.get_mut(state_id as usize) {
            t[byte as usize] = Some(next_state_id);
        }
    }

    fn state_has_match(&self, state_id: u32) -> bool {
        self.has_match.get(state_id as usize).copied().unwrap_or(false)
    }

    fn get_state_set(&self, state_id: u32) -> &[u32] {
        self.state_sets.get(state_id as usize).map(|v| v.as_slice()).unwrap_or(&[])
    }
}

pub struct PikeVm<'a> {
    bytecode: &'a [u8],
    input: &'a [u8],
    input_len: usize,
    capture_count: usize,
    register_count: usize,
    num_pcs: usize,
    unicode_mode: bool,
}

pub enum PikeResult {
    Match(Vec<Option<usize>>),
    NoMatch,
}

impl<'a> PikeVm<'a> {
    pub fn new(bytecode: &'a [u8], input: &'a [u8]) -> Self {
        let flags = u16::from_le_bytes([bytecode[0], bytecode[1]]);
        let unicode_mode = (flags & 0x10) != 0; // Flags::UNICODE = 1 << 4
        let capture_count = bytecode[RE_HEADER_CAPTURE_COUNT] as usize;
        let register_count = bytecode[RE_HEADER_CAPTURE_COUNT + 1] as usize;
        let bc_len = u32::from_le_bytes([
            bytecode[4], bytecode[5], bytecode[6], bytecode[7]
        ]) as usize;

        PikeVm {
            bytecode,
            input,
            input_len: input.len(),
            capture_count,
            register_count,
            num_pcs: RE_HEADER_LEN + bc_len + 1,
            unicode_mode,
        }
    }

    /// Create a persistent scanner with lazy DFA for repeated matching.
    pub fn scanner(&self) -> PikeScanner<'a> {
        PikeScanner::new(self.bytecode, self.input)
    }

    /// Capture-free execution: just returns whether a match exists.
    pub fn find_match(&self, start_pos: usize) -> Option<usize> {
        // For one-shot calls, use the full exec (it has the prefix loop)
        match self.exec(start_pos) {
            PikeResult::Match(caps) => caps.get(1).copied().flatten(),
            PikeResult::NoMatch => None,
        }
    }

    /// Fast epsilon closure for capture-free mode: just collects terminal PCs.
    fn eps_closure_fast(
        &self,
        states: &mut Vec<u32>,
        seen: &mut [bool],
        stack: &mut Vec<(usize, bool)>,
        start_pc: usize,
        at: usize,
    ) {
        stack.clear();
        stack.push((start_pc, false));

        while let Some((pc, _is_restore)) = stack.pop() {
            if pc >= self.bytecode.len() { continue; }
            if seen[pc] { continue; }

            let opcode = self.bytecode[pc];
            match opcode {
                op::GOTO => {
                    let offset = self.read_i32(pc + 1);
                    let target = ((pc + 5) as isize + offset as isize) as usize;
                    stack.push((target, false));
                }
                op::SPLIT_GOTO_FIRST => {
                    let offset = self.read_i32(pc + 1);
                    let goto = ((pc + 5) as isize + offset as isize) as usize;
                    stack.push((pc + 5, false));
                    stack.push((goto, false));
                }
                op::SPLIT_NEXT_FIRST => {
                    let offset = self.read_i32(pc + 1);
                    let goto = ((pc + 5) as isize + offset as isize) as usize;
                    stack.push((goto, false));
                    stack.push((pc + 5, false));
                }
                op::SAVE_START | op::SAVE_END => {
                    stack.push((pc + 2, false));
                }
                op::SAVE_RESET => {
                    stack.push((pc + 3, false));
                }
                op::SET_I32 => {
                    // Skip register ops in capture-free mode
                    stack.push((pc + 6, false));
                }
                op::LOOP_SPLIT_GOTO_FIRST | op::LOOP_SPLIT_NEXT_FIRST => {
                    // Without registers, treat as simple split
                    let offset = self.read_i32(pc + 6);
                    let goto = ((pc + 10) as isize + offset as isize) as usize;
                    let next = pc + 10;
                    if opcode == op::LOOP_SPLIT_GOTO_FIRST {
                        stack.push((next, false));
                        stack.push((goto, false));
                    } else {
                        stack.push((goto, false));
                        stack.push((next, false));
                    }
                }
                op::LINE_START | op::LINE_START_M => {
                    if at == 0 || (opcode == op::LINE_START_M && at > 0 && self.input[at - 1] == b'\n') {
                        stack.push((pc + 1, false));
                    }
                }
                op::LINE_END | op::LINE_END_M => {
                    if at == self.input_len || (opcode == op::LINE_END_M && at < self.input_len && self.input[at] == b'\n') {
                        stack.push((pc + 1, false));
                    }
                }
                op::WORD_BOUNDARY | op::NOT_WORD_BOUNDARY => {
                    let before = if at > 0 { is_word_char_at(self.input, at - 1) } else { false };
                    let after = if at < self.input_len { is_word_char_at(self.input, at) } else { false };
                    if (opcode == op::WORD_BOUNDARY) == (before != after) {
                        stack.push((pc + 1, false));
                    }
                }
                _ => {
                    // Terminal (consuming) state
                    seen[pc] = true;
                    states.push(pc as u32);
                }
            }
        }
    }

    /// Full execution with captures and greedy/lazy semantics.
    pub fn exec(&self, start_pos: usize) -> PikeResult {
        let mut curr = ThreadList::new(self.num_pcs, self.capture_count, self.register_count);
        let mut next = ThreadList::new(self.num_pcs, self.capture_count, self.register_count);
        let mut eps_stack: Vec<EpsFrame> = Vec::with_capacity(64);
        let mut tmp_caps = vec![None; self.capture_count * 2];
        let mut tmp_regs = vec![0usize; self.register_count];
        self.exec_inner(&mut curr, &mut next, &mut eps_stack, &mut tmp_caps, &mut tmp_regs, start_pos)
    }

    /// Full execution reusing shared Scratch (avoids per-call allocation).
    pub fn exec_with_scratch(&self, scratch: &mut Scratch, start_pos: usize) -> PikeResult {
        self.exec_reuse(
            &mut scratch.curr, &mut scratch.next,
            &mut scratch.eps_stack, &mut scratch.tmp_caps, &mut scratch.tmp_regs,
            start_pos,
        )
    }

    /// Full execution reusing pre-allocated buffers (avoids per-call allocation).
    fn exec_reuse(
        &self,
        curr: &mut ThreadList,
        next: &mut ThreadList,
        eps_stack: &mut Vec<EpsFrame>,
        tmp_caps: &mut Vec<Option<usize>>,
        tmp_regs: &mut Vec<usize>,
        start_pos: usize,
    ) -> PikeResult {
        // Ensure buffers are sized correctly for this VM
        if curr.seen.len() < self.num_pcs {
            *curr = ThreadList::new(self.num_pcs, self.capture_count, self.register_count);
        } else {
            curr.clear();
        }
        if next.seen.len() < self.num_pcs {
            *next = ThreadList::new(self.num_pcs, self.capture_count, self.register_count);
        } else {
            next.clear();
        }
        tmp_caps.resize(self.capture_count * 2, None);
        for v in tmp_caps.iter_mut() { *v = None; }
        tmp_regs.resize(self.register_count, 0);
        for v in tmp_regs.iter_mut() { *v = 0; }
        self.exec_inner(curr, next, eps_stack, tmp_caps, tmp_regs, start_pos)
    }

    /// Core exec implementation with external buffers.
    fn exec_inner(
        &self,
        curr: &mut ThreadList,
        next: &mut ThreadList,
        eps_stack: &mut Vec<EpsFrame>,
        tmp_caps: &mut [Option<usize>],
        tmp_regs: &mut [usize],
        start_pos: usize,
    ) -> PikeResult {
        let mut candidate: Option<Vec<Option<usize>>> = None;

        // Initialize: epsilon closure from bytecode start
        self.epsilon_closure(curr, eps_stack, tmp_caps, tmp_regs, RE_HEADER_LEN, start_pos);

        let mut at = start_pos;
        loop {
            // === MATCH CHECK WITH SHADOWING ===
            // Scan threads in priority order (index 0 = highest priority).
            // If the highest-priority thread that reached MATCH exists, check
            // if ANY higher-priority non-MATCH thread is still alive.
            let mut first_match_idx: Option<usize> = None;
            for (i, &(pc, _slot_idx)) in curr.threads.iter().enumerate() {
                if (pc as usize) < self.bytecode.len() && self.bytecode[pc as usize] == op::MATCH {
                    first_match_idx = Some(i);
                    break; // First MATCH in priority order
                }
            }

            if let Some(match_idx) = first_match_idx {
                let (_, slot_idx) = curr.threads[match_idx];
                let caps = curr.get_caps(slot_idx).to_vec();

                #[cfg(test)]
                eprintln!("[pike] at={} match_idx={} caps={:?} candidate={:?} threads={}",
                    at, match_idx, &caps[..2.min(caps.len())], candidate.as_ref().map(|c| &c[..2.min(c.len())]), curr.threads.len());

                if match_idx == 0 {
                    if let Some(prev) = candidate {
                        if prev.get(0) < caps.get(0) {
                            return PikeResult::Match(prev);
                        }
                    }
                    return PikeResult::Match(caps);
                }

                match &candidate {
                    None => { candidate = Some(caps); }
                    Some(prev) => {
                        if caps.get(0) == prev.get(0) {
                            candidate = Some(caps);
                        }
                    }
                }
            }

            if at >= self.input_len {
                break;
            }

            // === ADVANCE: consume input[at] ===
            let (c, char_len) = self.next_char(at);
            if char_len == 0 { break; }

            for i in 0..curr.threads.len() {
                let (pc, slot_idx) = curr.threads[i];
                let pc_usize = pc as usize;
                if pc_usize >= self.bytecode.len() { continue; }

                let opcode = self.bytecode[pc_usize];
                if let Some((next_pc, advance)) = self.try_consume(pc_usize, opcode, at, c) {
                    let actual_advance = if advance > char_len { advance } else { char_len };
                    let cs = curr.capture_stride;
                    let src = slot_idx as usize * cs;
                    for j in 0..cs.min(tmp_caps.len()) {
                        tmp_caps[j] = curr.slots[src + j];
                    }
                    let rs = curr.reg_stride;
                    if rs > 0 {
                        let src_r = slot_idx as usize * rs;
                        for j in 0..rs.min(tmp_regs.len()) {
                            tmp_regs[j] = curr.regs[src_r + j];
                        }
                    }

                    self.epsilon_closure(next, eps_stack, tmp_caps, tmp_regs, next_pc, at + actual_advance);
                }
            }

            std::mem::swap(curr, next);
            next.clear();
            at += char_len;

            if curr.threads.is_empty() {
                break;
            }
        }

        // Final match check: prefer candidate (leftmost) over any final-position match
        for &(pc, slot_idx) in &curr.threads {
            if (pc as usize) < self.bytecode.len() && self.bytecode[pc as usize] == op::MATCH {
                let caps = curr.get_caps(slot_idx).to_vec();
                // Check candidate first — leftmost wins
                if let Some(ref prev) = candidate {
                    if prev.get(0) <= caps.get(0) {
                        return PikeResult::Match(candidate.unwrap());
                    }
                }
                return PikeResult::Match(caps);
            }
        }

        match candidate {
            Some(caps) => PikeResult::Match(caps),
            None => PikeResult::NoMatch,
        }
    }

    /// Epsilon closure: follow all non-consuming transitions, maintaining priority order.
    fn epsilon_closure(
        &self,
        threads: &mut ThreadList,
        stack: &mut Vec<EpsFrame>,
        caps: &mut [Option<usize>],
        regs: &mut [usize],
        start_pc: usize,
        at: usize,
    ) {
        stack.clear();
        stack.push(EpsFrame::Explore(start_pc));

        while let Some(frame) = stack.pop() {
            match frame {
                EpsFrame::Explore(pc) => {
                    if pc >= self.bytecode.len() { continue; }

                    let opcode = self.bytecode[pc];
                    match opcode {
                        op::GOTO => {
                            let offset = self.read_i32(pc + 1);
                            let target = ((pc + 5) as isize + offset as isize) as usize;
                            stack.push(EpsFrame::Explore(target));
                        }

                        op::SPLIT_GOTO_FIRST => {
                            // Greedy: try goto first (higher priority)
                            let offset = self.read_i32(pc + 1);
                            let goto_target = ((pc + 5) as isize + offset as isize) as usize;
                            let next_target = pc + 5;
                            // Push in REVERSE order (stack is LIFO)
                            stack.push(EpsFrame::Explore(next_target));  // lower priority
                            stack.push(EpsFrame::Explore(goto_target));  // higher priority (popped first)
                        }

                        op::SPLIT_NEXT_FIRST => {
                            // Lazy: try next first (higher priority)
                            let offset = self.read_i32(pc + 1);
                            let goto_target = ((pc + 5) as isize + offset as isize) as usize;
                            let next_target = pc + 5;
                            stack.push(EpsFrame::Explore(goto_target));  // lower priority
                            stack.push(EpsFrame::Explore(next_target));  // higher priority (popped first)
                        }

                        op::SAVE_START => {
                            let idx = self.bytecode[pc + 1] as usize;
                            let cap_idx = idx * 2;
                            if cap_idx < caps.len() {
                                let old = caps[cap_idx];
                                caps[cap_idx] = Some(at);
                                stack.push(EpsFrame::RestoreCapture(cap_idx, old));
                                stack.push(EpsFrame::Explore(pc + 2));
                            }
                        }

                        op::SAVE_END => {
                            let idx = self.bytecode[pc + 1] as usize;
                            let cap_idx = idx * 2 + 1;
                            if cap_idx < caps.len() {
                                let old = caps[cap_idx];
                                caps[cap_idx] = Some(at);
                                stack.push(EpsFrame::RestoreCapture(cap_idx, old));
                                stack.push(EpsFrame::Explore(pc + 2));
                            }
                        }

                        op::SAVE_RESET => {
                            let start_idx = self.bytecode[pc + 1] as usize;
                            let end_idx = self.bytecode[pc + 2] as usize;
                            for idx in (start_idx..=end_idx).rev() {
                                let cs = idx * 2;
                                let ce = idx * 2 + 1;
                                if ce < caps.len() {
                                    stack.push(EpsFrame::RestoreCapture(ce, caps[ce]));
                                    stack.push(EpsFrame::RestoreCapture(cs, caps[cs]));
                                    caps[cs] = None;
                                    caps[ce] = None;
                                }
                            }
                            stack.push(EpsFrame::Explore(pc + 3));
                        }

                        op::SET_I32 => {
                            let reg = self.bytecode[pc + 1] as usize;
                            let val = self.read_u32(pc + 2) as usize;
                            if reg < regs.len() {
                                let old = regs[reg];
                                regs[reg] = val;
                                stack.push(EpsFrame::RestoreRegister(reg, old));
                                stack.push(EpsFrame::Explore(pc + 6));
                            }
                        }

                        op::LOOP_SPLIT_GOTO_FIRST | op::LOOP_SPLIT_NEXT_FIRST => {
                            let reg = self.bytecode[pc + 1] as usize;
                            let limit = self.read_u32(pc + 2) as usize;
                            let offset = self.read_i32(pc + 6);
                            let goto_target = ((pc + 10) as isize + offset as isize) as usize;
                            let next_target = pc + 10;

                            if reg < regs.len() {
                                let count = regs[reg];
                                let new_count = count.saturating_sub(1);
                                let old = regs[reg];
                                regs[reg] = new_count;
                                stack.push(EpsFrame::RestoreRegister(reg, old));

                                if new_count > limit {
                                    stack.push(EpsFrame::Explore(goto_target));
                                } else if new_count == 0 {
                                    stack.push(EpsFrame::Explore(next_target));
                                } else {
                                    if opcode == op::LOOP_SPLIT_GOTO_FIRST {
                                        stack.push(EpsFrame::Explore(next_target));
                                        stack.push(EpsFrame::Explore(goto_target));
                                    } else {
                                        stack.push(EpsFrame::Explore(goto_target));
                                        stack.push(EpsFrame::Explore(next_target));
                                    }
                                }
                            }
                        }

                        op::LINE_START | op::LINE_START_M => {
                            if at == 0 || (opcode == op::LINE_START_M && at > 0 && self.input[at - 1] == b'\n') {
                                stack.push(EpsFrame::Explore(pc + 1));
                            }
                        }

                        op::LINE_END | op::LINE_END_M => {
                            if at == self.input_len || (opcode == op::LINE_END_M && at < self.input_len && self.input[at] == b'\n') {
                                stack.push(EpsFrame::Explore(pc + 1));
                            }
                        }

                        op::WORD_BOUNDARY | op::NOT_WORD_BOUNDARY => {
                            let before = if at > 0 { is_word_char_at(self.input, at - 1) } else { false };
                            let after = if at < self.input_len { is_word_char_at(self.input, at) } else { false };
                            let is_boundary = before != after;
                            if (opcode == op::WORD_BOUNDARY) == is_boundary {
                                stack.push(EpsFrame::Explore(pc + 1));
                            }
                        }

                        // Terminal (consuming) state: add to thread list with current captures
                        _ => {
                            threads.add(pc as u32, caps, regs);
                        }
                    }
                }

                EpsFrame::RestoreCapture(idx, old_val) => {
                    if idx < caps.len() { caps[idx] = old_val; }
                }

                EpsFrame::RestoreRegister(idx, old_val) => {
                    if idx < regs.len() { regs[idx] = old_val; }
                }
            }
        }
    }

    fn try_consume(&self, pc: usize, opcode: u8, at: usize, c: u32) -> Option<(usize, usize)> {
        let char_len = if c < 0x80 { 1 } else { char::from_u32(c).map(|ch| ch.len_utf8()).unwrap_or(1) };

        match opcode {
            op::CHAR | op::CHAR_I => {
                let expected = self.read_u16(pc + 1) as u32;
                let ok = if opcode == op::CHAR_I { to_lower(c) == to_lower(expected) } else { c == expected };
                if ok { Some((pc + 3, char_len)) } else { None }
            }
            op::CHAR32 | op::CHAR32_I => {
                let expected = self.read_u32(pc + 1);
                let ok = if opcode == op::CHAR32_I { to_lower(c) == to_lower(expected) } else { c == expected };
                if ok { Some((pc + 5, char_len)) } else { None }
            }
            op::DOT => {
                if !matches!(c, 0x0A | 0x0D | 0x2028 | 0x2029) { Some((pc + 1, char_len)) } else { None }
            }
            op::ANY => Some((pc + 1, char_len)),
            op::SPACE => { if is_space(c) { Some((pc + 1, char_len)) } else { None } }
            op::NOT_SPACE => { if !is_space(c) { Some((pc + 1, char_len)) } else { None } }
            op::RANGE | op::RANGE_I => {
                let n = self.read_u16(pc + 1) as usize;
                let matched = if self.unicode_mode && is_word_range16(self.bytecode, pc + 3, n) {
                    is_word_char_unicode(c)
                } else {
                    let check = if opcode == op::RANGE_I { to_lower(c) } else { c };
                    self.check_range16(check, pc + 3, n)
                };
                if matched { Some((pc + 3 + n * 4, char_len)) } else { None }
            }
            op::RANGE32 | op::RANGE32_I => {
                let n = self.read_u16(pc + 1) as usize;
                let check = if opcode == op::RANGE32_I { to_lower(c) } else { c };
                if self.check_range32(check, pc + 3, n) { Some((pc + 3 + n * 8, char_len)) } else { None }
            }
            op::SPAN_ANY => {
                // Consume min..max of any character
                let min_count = self.read_u32(pc + 1) as usize;
                let max_count = self.read_u32(pc + 5) as usize;
                let next_pc = pc + 9;
                let consumed = self.span_any(at, min_count, max_count);
                if consumed >= min_count {
                    Some((next_pc, consumed))
                } else {
                    None
                }
            }
            op::SPAN_DOT => {
                let min_count = self.read_u32(pc + 1) as usize;
                let max_count = self.read_u32(pc + 5) as usize;
                let next_pc = pc + 9;
                let consumed = self.span_dot(at, min_count, max_count);
                if consumed >= min_count {
                    Some((next_pc, consumed))
                } else {
                    None
                }
            }
            op::SPAN_CLASS => {
                let min_count = self.read_u32(pc + 1) as usize;
                let max_count = self.read_u32(pc + 5) as usize;
                let pair_count = self.read_u16(pc + 9) as usize;
                let data_start = pc + 11;
                let next_pc = data_start + pair_count * 4;
                let consumed = self.span_class(at, min_count, max_count, data_start, pair_count);
                if consumed >= min_count {
                    Some((next_pc, consumed))
                } else {
                    None
                }
            }
            op::MATCH => None,
            _ => None,
        }
    }

    /// Consume up to max_count of any character from position at. Returns byte count consumed.
    fn span_any(&self, at: usize, _min: usize, max: usize) -> usize {
        let mut pos = at;
        let mut count = 0;
        while count < max && pos < self.input_len {
            let (_, clen) = self.next_char(pos);
            if clen == 0 { break; }
            pos += clen;
            count += 1;
        }
        pos - at // byte count
    }

    /// Consume up to max_count of DOT (any except line terminators).
    fn span_dot(&self, at: usize, _min: usize, max: usize) -> usize {
        let mut pos = at;
        let mut count = 0;
        while count < max && pos < self.input_len {
            let (c, clen) = self.next_char(pos);
            if clen == 0 || matches!(c, 0x0A | 0x0D | 0x2028 | 0x2029) { break; }
            pos += clen;
            count += 1;
        }
        pos - at
    }

    /// Consume up to max_count chars matching a character class.
    fn span_class(&self, at: usize, _min: usize, max: usize, data_start: usize, pair_count: usize) -> usize {
        let mut pos = at;
        let mut count = 0;
        while count < max && pos < self.input_len {
            let (c, clen) = self.next_char(pos);
            if clen == 0 { break; }
            if !self.check_range16(c, data_start, pair_count) { break; }
            pos += clen;
            count += 1;
        }
        pos - at
    }

    #[inline] fn next_char(&self, pos: usize) -> (u32, usize) {
        if pos >= self.input_len { return (0, 0); }
        let b = self.input[pos];
        if b < 0x80 { return (b as u32, 1); }
        std::str::from_utf8(&self.input[pos..]).ok()
            .and_then(|s| s.chars().next())
            .map(|ch| (ch as u32, ch.len_utf8()))
            .unwrap_or((b as u32, 1))
    }

    #[inline] fn read_u16(&self, pc: usize) -> u16 { u16::from_le_bytes([self.bytecode[pc], self.bytecode[pc+1]]) }
    #[inline] fn read_u32(&self, pc: usize) -> u32 { u32::from_le_bytes([self.bytecode[pc], self.bytecode[pc+1], self.bytecode[pc+2], self.bytecode[pc+3]]) }
    #[inline] fn read_i32(&self, pc: usize) -> i32 { i32::from_le_bytes([self.bytecode[pc], self.bytecode[pc+1], self.bytecode[pc+2], self.bytecode[pc+3]]) }

    fn check_range16(&self, c: u32, start: usize, n: usize) -> bool {
        for i in 0..n {
            let b = start + i * 4;
            let lo = u16::from_le_bytes([self.bytecode[b], self.bytecode[b+1]]) as u32;
            let hi = u16::from_le_bytes([self.bytecode[b+2], self.bytecode[b+3]]) as u32;
            if c >= lo && c <= hi { return true; }
        }
        false
    }

    fn check_range32(&self, c: u32, start: usize, n: usize) -> bool {
        for i in 0..n {
            let b = start + i * 8;
            let lo = u32::from_le_bytes([self.bytecode[b], self.bytecode[b+1], self.bytecode[b+2], self.bytecode[b+3]]);
            let hi = u32::from_le_bytes([self.bytecode[b+4], self.bytecode[b+5], self.bytecode[b+6], self.bytecode[b+7]]);
            if c >= lo && c <= hi { return true; }
        }
        false
    }
}

// ============================================================================
// Bucket Queue Scanner: timeline-based execution with DFA cache
// ============================================================================

use std::collections::BTreeMap;

/// Bucket Queue: maps position → list of thread PCs waiting at that position.
/// Threads can schedule themselves at future positions via skip instructions.
/// Processing order: lowest position first (BTreeMap gives this for free).
struct BucketQueue {
    buckets: BTreeMap<usize, Vec<u32>>,
}

impl BucketQueue {
    fn new() -> Self {
        BucketQueue { buckets: BTreeMap::new() }
    }

    /// Schedule a thread at a future position
    #[inline]
    fn schedule(&mut self, pos: usize, pc: u32) {
        self.buckets.entry(pos).or_insert_with(Vec::new).push(pc);
    }

    /// Get the next position that has waiting threads
    fn next_pos(&self) -> Option<usize> {
        self.buckets.keys().next().copied()
    }

    /// Take all threads waiting at `pos`
    fn take(&mut self, pos: usize) -> Vec<u32> {
        self.buckets.remove(&pos).unwrap_or_default()
    }

    fn is_empty(&self) -> bool {
        self.buckets.is_empty()
    }
}

use std::cell::RefCell;

/// Reusable scratch space for regex execution. Allocated once, passed to
/// `find_at` / `captures_at` for zero-allocation matching on hot paths.
///
/// Create via `Regex::create_scratch()`. Thread-local: do NOT share across threads.
/// Each thread should have its own Scratch.
pub struct Scratch {
    // Exec buffers (for bounded exec pass 2)
    curr: ThreadList,
    next: ThreadList,
    eps_stack: Vec<EpsFrame>,
    tmp_caps: Vec<Option<usize>>,
    tmp_regs: Vec<usize>,
    // DFA scan buffers (for pass 1)
    dfa_curr_states: Vec<u32>,
    dfa_next_states: Vec<u32>,
    dfa_seen: Vec<bool>,
    dfa_eps_stack: Vec<(usize, bool)>,
    /// Persistent DFA cache — survives across find_at calls for warm O(1)/byte scanning.
    dfa: RefCell<LazyDfa>,
}

impl Scratch {
    pub fn new(num_pcs: usize, capture_count: usize, register_count: usize) -> Self {
        Scratch {
            curr: ThreadList::new(num_pcs, capture_count, register_count),
            next: ThreadList::new(num_pcs, capture_count, register_count),
            eps_stack: Vec::with_capacity(64),
            tmp_caps: vec![None; capture_count * 2],
            tmp_regs: vec![0; register_count],
            dfa_curr_states: Vec::with_capacity(128),
            dfa_next_states: Vec::with_capacity(128),
            dfa_seen: vec![false; num_pcs],
            dfa_eps_stack: Vec::with_capacity(64),
            dfa: RefCell::new(LazyDfa::new()),
        }
    }

    /// Two-pass find: Wide NFA for fast match_end + bounded exec for correct semantics.
    /// Pass 1: Wide NFA scans at O(states/64)/byte — finds where a match ends.
    /// Pass 2: Bounded exec on input[..match_end] — correct greedy/lazy/assertion semantics.
    pub fn find_at(&mut self, vm: &PikeVm, wide_nfa: &super::bitvm::BitVmProgram, start_pos: usize) -> Option<(usize, usize)> {
        let match_end = wide_nfa.find_match_end(vm.input, start_pos)?;

        let bounded_vm = PikeVm::new(vm.bytecode, &vm.input[..match_end]);
        match bounded_vm.exec_reuse(
            &mut self.curr, &mut self.next,
            &mut self.eps_stack, &mut self.tmp_caps, &mut self.tmp_regs,
            start_pos,
        ) {
            PikeResult::Match(caps) => {
                let s = caps.get(0).copied().flatten()?;
                let e = caps.get(1).copied().flatten()?;
                Some((s, e))
            }
            PikeResult::NoMatch => {
                // Wide NFA false positive — fall back to full exec
                match vm.exec_reuse(
                    &mut self.curr, &mut self.next,
                    &mut self.eps_stack, &mut self.tmp_caps, &mut self.tmp_regs,
                    start_pos,
                ) {
                    PikeResult::Match(caps) => {
                        let s = caps.get(0).copied().flatten()?;
                        let e = caps.get(1).copied().flatten()?;
                        Some((s, e))
                    }
                    PikeResult::NoMatch => None,
                }
            }
        }
    }
}

/// DFA cache storage: either owned by the scanner or borrowed from a Regex.
enum DfaStorage<'a> {
    Owned(LazyDfa),
    Borrowed(&'a RefCell<LazyDfa>),
}

/// Persistent scanner with warm DFA cache for repeated matching.
/// Holds all state needed to scan efficiently across multiple find_next calls.
/// Reuses exec buffers (ThreadLists) across calls to avoid per-call allocation.
pub struct PikeScanner<'a> {
    vm: PikeVm<'a>,
    dfa: DfaStorage<'a>,
    // DFA scan buffers
    curr_states: Vec<u32>,
    next_states: Vec<u32>,
    seen: Vec<bool>,
    eps_stack: Vec<(usize, bool)>,
    // Exec buffers (reused across find_next calls to avoid allocation)
    exec_curr: ThreadList,
    exec_next: ThreadList,
    exec_eps_stack: Vec<EpsFrame>,
    exec_tmp_caps: Vec<Option<usize>>,
    exec_tmp_regs: Vec<usize>,
}

impl<'a> PikeScanner<'a> {
    pub fn new(bytecode: &'a [u8], input: &'a [u8]) -> Self {
        let vm = PikeVm::new(bytecode, input);
        let num_pcs = vm.num_pcs;
        let cc = vm.capture_count;
        let rc = vm.register_count;
        PikeScanner {
            exec_curr: ThreadList::new(num_pcs, cc, rc),
            exec_next: ThreadList::new(num_pcs, cc, rc),
            exec_eps_stack: Vec::with_capacity(64),
            exec_tmp_caps: vec![None; cc * 2],
            exec_tmp_regs: vec![0; rc],
            vm,
            dfa: DfaStorage::Owned(LazyDfa::new()),
            curr_states: Vec::with_capacity(128),
            next_states: Vec::with_capacity(128),
            seen: vec![false; num_pcs],
            eps_stack: Vec::with_capacity(64),
        }
    }

    /// Create a scanner that borrows a shared DFA cache from a Regex.
    /// The DFA state map persists across calls, giving O(1) per byte on warm cache.
    pub fn with_cache(bytecode: &'a [u8], input: &'a [u8], cache: &'a RefCell<LazyDfa>) -> Self {
        let vm = PikeVm::new(bytecode, input);
        let num_pcs = vm.num_pcs;
        let cc = vm.capture_count;
        let rc = vm.register_count;
        PikeScanner {
            exec_curr: ThreadList::new(num_pcs, cc, rc),
            exec_next: ThreadList::new(num_pcs, cc, rc),
            exec_eps_stack: Vec::with_capacity(64),
            exec_tmp_caps: vec![None; cc * 2],
            exec_tmp_regs: vec![0; rc],
            vm,
            dfa: DfaStorage::Borrowed(cache),
            curr_states: Vec::with_capacity(128),
            next_states: Vec::with_capacity(128),
            seen: vec![false; num_pcs],
            eps_stack: Vec::with_capacity(64),
        }
    }

    /// Find the next match starting at or after `start_pos`.
    /// Returns Some((match_start, match_end)) or None.
    ///
    /// Two-pass strategy:
    ///   1. DFA scan (O(1)/byte on cache hits) → finds match_end
    ///   2. Bounded exec on input[..match_end] → finds match_start
    ///      Only processes ~match_length bytes, not the entire remaining text.
    pub fn find_next(&mut self, start_pos: usize) -> Option<(usize, usize)> {
        // Pass 1: DFA scan for match_end (fast)
        let match_end = self.find_match_cached(start_pos)?;

        // Pass 2: bounded exec only up to match_end to find match_start.
        // This is critical for complex patterns (e.g. date regex with 3000 states):
        // full exec would process O(remaining_text × states), but bounded exec
        // only processes O(match_length × states).
        let bounded_vm = PikeVm::new(self.vm.bytecode, &self.vm.input[..match_end]);
        match bounded_vm.exec_reuse(
            &mut self.exec_curr, &mut self.exec_next,
            &mut self.exec_eps_stack, &mut self.exec_tmp_caps, &mut self.exec_tmp_regs,
            start_pos,
        ) {
            PikeResult::Match(caps) => {
                let s = caps.get(0).copied().flatten()?;
                let e = caps.get(1).copied().flatten()?;
                Some((s, e))
            }
            PikeResult::NoMatch => {
                // DFA said match exists but bounded exec disagrees.
                // Fall back to full exec (handles edge cases with assertions).
                match self.vm.exec_reuse(
                    &mut self.exec_curr, &mut self.exec_next,
                    &mut self.exec_eps_stack, &mut self.exec_tmp_caps, &mut self.exec_tmp_regs,
                    start_pos,
                ) {
                    PikeResult::Match(caps) => {
                        let s = caps.get(0).copied().flatten()?;
                        let e = caps.get(1).copied().flatten()?;
                        Some((s, e))
                    }
                    PikeResult::NoMatch => None,
                }
            }
        }
    }

    /// Count all non-overlapping matches.
    /// Uses Bucket Queue with DFA cache for all patterns (handles registers via timeline).
    pub fn count_all(&mut self) -> usize {
        let mut count = 0;
        let mut pos = 0;

        // Use DFA-cached scan for register-free ASCII-safe patterns.
        // Unicode mode patterns need exec_reuse (DFA allocates per non-ASCII byte).
        if self.vm.register_count == 0 && !self.vm.unicode_mode {
            while pos <= self.vm.input_len {
                match self.find_match_cached(pos) {
                    Some(end) => {
                        count += 1;
                        pos = if end > pos { end } else { pos + 1 };
                    }
                    None => break,
                }
            }
            return count;
        }

        // For register patterns: use full exec with reused buffers
        while pos <= self.vm.input_len {
            match self.vm.exec_reuse(
                &mut self.exec_curr, &mut self.exec_next,
                &mut self.exec_eps_stack, &mut self.exec_tmp_caps, &mut self.exec_tmp_regs,
                pos,
            ) {
                PikeResult::Match(caps) => {
                    count += 1;
                    let end = caps.get(1).copied().flatten().unwrap_or(pos + 1);
                    pos = if end > pos { end } else { pos + 1 };
                }
                PikeResult::NoMatch => break,
            }
        }

        count
    }

    /// Capture-free scan with lazy DFA. O(1) per byte on cache hits.
    fn find_match_cached(&mut self, start_pos: usize) -> Option<usize> {
        match &self.dfa {
            DfaStorage::Owned(_) => {
                // Reborrow: take owned DFA mutably
                let DfaStorage::Owned(ref mut dfa) = self.dfa else { unreachable!() };
                Self::find_match_cached_inner(
                    &self.vm, dfa,
                    &mut self.curr_states, &mut self.next_states,
                    &mut self.seen, &mut self.eps_stack,
                    start_pos,
                )
            }
            DfaStorage::Borrowed(cell) => {
                let mut dfa = cell.borrow_mut();
                Self::find_match_cached_inner(
                    &self.vm, &mut dfa,
                    &mut self.curr_states, &mut self.next_states,
                    &mut self.seen, &mut self.eps_stack,
                    start_pos,
                )
            }
        }
    }

    /// Core DFA-cached scan logic, operating on a mutable DFA reference.
    pub(crate) fn find_match_cached_inner(
        vm: &PikeVm<'a>,
        dfa: &mut LazyDfa,
        curr_states: &mut Vec<u32>,
        next_states: &mut Vec<u32>,
        seen: &mut Vec<bool>,
        eps_stack: &mut Vec<(usize, bool)>,
        start_pos: usize,
    ) -> Option<usize> {
        curr_states.clear();
        seen.fill(false);

        // Bootstrap: epsilon closure from start
        vm.eps_closure_fast(
            curr_states, seen, eps_stack,
            RE_HEADER_LEN, start_pos,
        );

        // Check if initial state has match
        let init_has_match = curr_states.iter()
            .any(|&pc| (pc as usize) < vm.bytecode.len() && vm.bytecode[pc as usize] == op::MATCH);

        // Register initial state in DFA
        let mut current_dfa_state = match dfa.get_or_create_state(curr_states, init_has_match) {
            Some(id) => id,
            None => {
                // DFA cache full — fall back to uncached Pike VM
                return Self::find_match_uncached_vm(vm, start_pos);
            }
        };

        let mut at = start_pos;
        let mut best_end: Option<usize> = None;

        loop {
            // Check for match
            if dfa.state_has_match(current_dfa_state) {
                best_end = Some(at);
                // Check if MATCH is highest priority (first in state set)
                let states = dfa.get_state_set(current_dfa_state);
                if let Some(&first_pc) = states.first() {
                    if (first_pc as usize) < vm.bytecode.len()
                        && vm.bytecode[first_pc as usize] == op::MATCH
                    {
                        return best_end; // Highest priority match → immediate win
                    }
                }
            }

            if at >= vm.input_len { break; }

            let b = vm.input[at];
            if b >= 128 {
                // Non-ASCII: process this char via NFA (no DFA caching), then resume DFA
                let (c, char_len) = vm.next_char(at);
                if char_len == 0 { break; }

                let states_vec = dfa.get_state_set(current_dfa_state).to_vec();
                next_states.clear();
                seen.fill(false);

                for &pc in &states_vec {
                    let pc_usize = pc as usize;
                    if pc_usize >= vm.bytecode.len() { continue; }
                    let opcode = vm.bytecode[pc_usize];
                    if let Some((next_pc, _)) = vm.try_consume(pc_usize, opcode, at, c) {
                        vm.eps_closure_fast(
                            next_states, seen, eps_stack,
                            next_pc, at + char_len,
                        );
                    }
                }

                let next_has_match = next_states.iter()
                    .any(|&pc| (pc as usize) < vm.bytecode.len() && vm.bytecode[pc as usize] == op::MATCH);

                match dfa.get_or_create_state(next_states, next_has_match) {
                    Some(next_id) => {
                        // Don't cache transition for non-ASCII first byte
                        // (same first byte can produce different chars in different contexts)
                        current_dfa_state = next_id;
                    }
                    None => {
                        return best_end.or_else(|| Self::find_match_uncached_vm(vm, at));
                    }
                }

                at += char_len;
                continue;
            }

            // O(1) DFA transition lookup
            if let Some(next_id) = dfa.lookup(current_dfa_state, b) {
                current_dfa_state = next_id;
                at += 1;
                continue;
            }

            // Cache miss: compute transition via Pike VM
            let states = dfa.get_state_set(current_dfa_state).to_vec();
            next_states.clear();
            seen.fill(false);

            let (c, char_len) = vm.next_char(at);
            if char_len == 0 { break; }

            for &pc in &states {
                let pc_usize = pc as usize;
                if pc_usize >= vm.bytecode.len() { continue; }
                let opcode = vm.bytecode[pc_usize];
                if let Some((next_pc, _)) = vm.try_consume(pc_usize, opcode, at, c) {
                    vm.eps_closure_fast(
                        next_states, seen, eps_stack,
                        next_pc, at + char_len,
                    );
                }
            }

            let next_has_match = next_states.iter()
                .any(|&pc| (pc as usize) < vm.bytecode.len() && vm.bytecode[pc as usize] == op::MATCH);

            match dfa.get_or_create_state(next_states, next_has_match) {
                Some(next_id) => {
                    dfa.store(current_dfa_state, b, next_id);
                    current_dfa_state = next_id;
                }
                None => {
                    // DFA cache full
                    return best_end.or_else(|| Self::find_match_uncached_vm(vm, at));
                }
            }

            at += char_len;
        }

        // Final check
        if best_end.is_none() && dfa.state_has_match(current_dfa_state) {
            best_end = Some(at);
        }

        best_end
    }

    /// Fallback uncached scan (for non-ASCII or DFA overflow)
    fn find_match_uncached_vm(vm: &PikeVm<'a>, start_pos: usize) -> Option<usize> {
        match vm.exec(start_pos) {
            PikeResult::Match(caps) => caps.get(1).copied().flatten(),
            PikeResult::NoMatch => None,
        }
    }
}

#[inline] fn to_lower(c: u32) -> u32 {
    if c >= 'A' as u32 && c <= 'Z' as u32 { c + 32 }
    else if c >= 0x80 { char::from_u32(c).and_then(|ch| { let mut l = ch.to_lowercase(); let lc = l.next()?; if l.next().is_none() { Some(lc as u32) } else { None } }).unwrap_or(c) }
    else { c }
}

#[inline] fn is_space(c: u32) -> bool {
    matches!(c, 0x09 | 0x0A | 0x0B | 0x0C | 0x0D | 0x20 | 0xA0 | 0x1680 | 0x2000..=0x200A | 0x2028 | 0x2029 | 0x202F | 0x205F | 0x3000 | 0xFEFF)
}

#[inline] fn is_word_char_at(input: &[u8], pos: usize) -> bool {
    let b = input[pos];
    if b < 0x80 { b.is_ascii_alphanumeric() || b == b'_' }
    else { std::str::from_utf8(&input[pos..]).ok().and_then(|s| s.chars().next()).map(|c| c.is_alphanumeric() || c == '_').unwrap_or(false) }
}

/// Detect if a Range16 bytecode is the \w character class pattern
#[inline] fn is_word_range16(bytecode: &[u8], data_start: usize, pair_count: usize) -> bool {
    pair_count == 4 && data_start + 16 <= bytecode.len()
        && bytecode[data_start..data_start + 16] == [
            0x30, 0x00, 0x39, 0x00,  // 0-9
            0x41, 0x00, 0x5A, 0x00,  // A-Z
            0x5F, 0x00, 0x5F, 0x00,  // _
            0x61, 0x00, 0x7A, 0x00,  // a-z
        ]
}

/// Unicode-aware word character check (ID_Continue-like)
#[inline] fn is_word_char_unicode(c: u32) -> bool {
    if c < 128 {
        (c >= b'0' as u32 && c <= b'9' as u32)
            || (c >= b'A' as u32 && c <= b'Z' as u32)
            || c == b'_' as u32
            || (c >= b'a' as u32 && c <= b'z' as u32)
    } else {
        char::from_u32(c).map(|ch| ch.is_alphanumeric() || ch == '_').unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::regex::compiler;
    use crate::regex::Flags;

    fn pike_match(pattern: &str, text: &str) -> bool {
        let bc = compiler::compile_regex(pattern, Flags::empty()).unwrap();
        matches!(PikeVm::new(&bc, text.as_bytes()).exec(0), PikeResult::Match(_))
    }
    fn pike_find(pattern: &str, text: &str) -> Option<(usize, usize)> {
        let bc = compiler::compile_regex(pattern, Flags::empty()).unwrap();
        match PikeVm::new(&bc, text.as_bytes()).exec(0) {
            PikeResult::Match(c) => Some((c[0]?, c[1]?)),
            _ => None,
        }
    }

    #[test] fn test_literal() { assert!(pike_match("abc", "abc")); assert!(pike_match("abc", "xabcx")); assert!(!pike_match("abc", "abd")); }
    #[test] fn test_dot() { assert!(pike_match("a.c", "abc")); assert!(!pike_match("a.c", "a\nc")); }
    #[test] fn test_alt() { assert!(pike_match("cat|dog", "cat")); assert!(pike_match("cat|dog", "dog")); assert!(!pike_match("cat|dog", "cow")); }
    #[test] fn test_star() { assert!(pike_match("ab*c", "ac")); assert!(pike_match("ab*c", "abbc")); }
    #[test] fn test_plus() { assert!(pike_match("ab+c", "abc")); assert!(pike_match("ab+c", "abbc")); }
    #[test] fn test_question() { assert!(pike_match("ab?c", "ac")); assert!(pike_match("ab?c", "abc")); }
    #[test] fn test_char_class() { assert!(pike_match("[abc]", "b")); assert!(pike_match("[a-z]+", "hello")); }
    #[test] fn test_anchors() { assert!(pike_match("^abc", "abc")); assert!(!pike_match("^abc", "xabc")); }
    #[test] fn test_word_boundary() { assert!(pike_match(r"\bfoo\b", "foo bar")); assert!(!pike_match(r"\bfoo\b", "foobar")); }
    #[test] fn test_digit() { assert!(pike_match(r"\d+", "123")); }
    #[test] fn test_find() { assert_eq!(pike_find("world", "hello world"), Some((6, 11))); }
    #[test] fn test_bounded() { assert!(pike_match("[A-Za-z]{8,13}", "abcdefghij")); }

    #[test]
    fn test_greedy_star() {
        // Greedy: a* on "aaa" → matches all 3
        assert_eq!(pike_find("a*", "aaa"), Some((0, 3)));
    }

    #[test]
    fn test_lazy_star() {
        // Lazy: a*? on "aaa" → matches empty (shortest at position 0)
        assert_eq!(pike_find("a*?", "aaa"), Some((0, 0)));
    }

    #[test]
    fn test_greedy_bounded() {
        // Greedy: a{2,4} on "aaaa" → matches 4
        assert_eq!(pike_find("a{2,4}", "aaaa"), Some((0, 4)));
    }

    #[test]
    fn test_lazy_bounded() {
        // Lazy: a{2,4}? on "aaaa" → matches 2
        assert_eq!(pike_find("a{2,4}?", "aaaa"), Some((0, 2)));
    }

    #[test]
    fn test_captures() {
        let bc = compiler::compile_regex("(a)(b)", Flags::empty()).unwrap();
        match PikeVm::new(&bc, b"ab").exec(0) {
            PikeResult::Match(c) => {
                assert_eq!(c[0], Some(0));
                assert_eq!(c[1], Some(2));
                assert_eq!(c[2], Some(0));
                assert_eq!(c[3], Some(1));
            }
            _ => panic!("should match"),
        }
    }

    #[test]
    fn test_no_catastrophic_backtracking() {
        // Exponential in backtracker, linear in Pike VM
        assert!(!pike_match("(?:[A-Z][a-z]+\\s*){10,100}", &"Abcd ".repeat(9)));
    }
}
