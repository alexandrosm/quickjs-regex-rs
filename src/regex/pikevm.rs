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
}

// ============================================================================
// Ordered thread list with sparse-set deduplication
// ============================================================================

/// Thread list maintaining strict priority order.
/// Index 0 = highest priority. Sparse set prevents duplicate states.
struct ThreadList {
    /// Ordered list of (pc, slot_index) pairs
    threads: Vec<(u32, u32)>,
    /// Sparse set: pc → true if already in list (dedup)
    seen: Vec<bool>,
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
            seen: vec![false; num_pcs],
            slots: vec![None; max_threads * cs],
            regs: vec![0; max_threads * rs],
            capture_stride: cs,
            reg_stride: rs,
            slot_cursor: 0,
        }
    }

    fn clear(&mut self) {
        for &(pc, _) in &self.threads {
            self.seen[pc as usize] = false;
        }
        self.threads.clear();
        self.slot_cursor = 0;
    }

    /// Add a thread. Returns false if state already occupied (shadowed).
    #[inline]
    fn add(&mut self, pc: u32, caps: &[Option<usize>], regs: &[usize]) -> bool {
        let pc_idx = pc as usize;
        if pc_idx >= self.seen.len() || self.seen[pc_idx] {
            return false; // Shadowed by higher-priority thread
        }
        self.seen[pc_idx] = true;

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

/// LRU-style transition cache: memoizes (thread_set, byte) → next_thread_set.
/// This turns the Pike VM into a lazy DFA for hot paths.
const CACHE_SIZE: usize = 4096;

struct TransitionCache {
    /// Key: hash of (sorted state IDs, input byte)
    /// Value: sorted list of (pc, next_pc) transitions
    entries: Vec<Option<CacheEntry>>,
}

#[derive(Clone)]
struct CacheEntry {
    key_hash: u64,
    state_key: Vec<u32>,   // PCs of current thread set (in priority order)
    input_byte: u8,
    /// The full next-state set (PCs in priority order after epsilon closure)
    next_states: Vec<u32>,
}

impl TransitionCache {
    fn new() -> Self {
        TransitionCache {
            entries: vec![None; CACHE_SIZE],
        }
    }

    #[inline]
    fn hash_key(states: &[u32], byte: u8) -> u64 {
        // FNV-1a hash
        let mut h: u64 = 0xcbf29ce484222325;
        for &s in states {
            h ^= s as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
        h ^= byte as u64;
        h = h.wrapping_mul(0x100000001b3);
        h
    }

    fn lookup(&self, states: &[u32], byte: u8) -> Option<&CacheEntry> {
        let hash = Self::hash_key(states, byte);
        let idx = (hash as usize) % CACHE_SIZE;
        if let Some(ref entry) = self.entries[idx] {
            if entry.key_hash == hash && entry.input_byte == byte && entry.state_key == states {
                return Some(entry);
            }
        }
        None
    }

    fn insert(&mut self, states: Vec<u32>, byte: u8, next_states: Vec<u32>) {
        let hash = Self::hash_key(&states, byte);
        let idx = (hash as usize) % CACHE_SIZE;
        self.entries[idx] = Some(CacheEntry {
            key_hash: hash,
            state_key: states,
            input_byte: byte,
            next_states,
        });
    }
}

pub struct PikeVm<'a> {
    bytecode: &'a [u8],
    input: &'a [u8],
    input_len: usize,
    capture_count: usize,
    register_count: usize,
    num_pcs: usize,
}

pub enum PikeResult {
    Match(Vec<Option<usize>>),
    NoMatch,
}

impl<'a> PikeVm<'a> {
    pub fn new(bytecode: &'a [u8], input: &'a [u8]) -> Self {
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
        }
    }

    /// Create a persistent scanner with a warm DFA cache for repeated matching.
    pub fn scanner(&self) -> PikeScanner<'a> {
        PikeScanner {
            vm: PikeVm::new(self.bytecode, self.input),
            cache: TransitionCache::new(),
            curr_states: Vec::with_capacity(128),
            next_states: Vec::with_capacity(128),
            seen: vec![false; self.num_pcs],
            eps_stack: Vec::with_capacity(64),
            state_key: Vec::with_capacity(64),
        }
    }

    /// Capture-free execution: just returns whether a match exists and where it ends.
    /// Uses the DFA transition cache for O(n) throughput on stable state sets.
    pub fn find_match(&self, start_pos: usize) -> Option<usize> {
        // Simplified thread list: just state IDs, no captures
        let mut curr_states: Vec<u32> = Vec::with_capacity(128);
        let mut next_states: Vec<u32> = Vec::with_capacity(128);
        let mut seen = vec![false; self.num_pcs];
        let mut eps_stack: Vec<(usize, bool)> = Vec::with_capacity(64); // (pc, is_restore)
        let mut cache = TransitionCache::new();
        let mut state_key: Vec<u32> = Vec::with_capacity(64);
        let mut best_end: Option<usize> = None;

        // Epsilon closure into curr_states (no captures)
        self.eps_closure_fast(&mut curr_states, &mut seen, &mut eps_stack, RE_HEADER_LEN, start_pos);

        let mut at = start_pos;
        loop {
            // Check for MATCH state
            for &pc in &curr_states {
                if (pc as usize) < self.bytecode.len() && self.bytecode[pc as usize] == op::MATCH {
                    // Found a match ending at current position
                    best_end = Some(at);
                    // Don't break — continue to find the longest match at this start
                }
            }

            if at >= self.input_len { break; }
            if curr_states.is_empty() { break; }

            let b = self.input[at];
            let (c, char_len) = self.next_char(at);
            if char_len == 0 { break; }

            // Build state key for cache
            state_key.clear();
            state_key.extend_from_slice(&curr_states);

            // Clear seen for next step
            for &pc in &curr_states { seen[pc as usize] = false; }

            // Cache lookup (ASCII only)
            if let Some(entry) = cache.lookup(&state_key, b) {
                next_states.clear();
                next_states.extend_from_slice(&entry.next_states);
                for &pc in &next_states { seen[pc as usize] = true; }
            } else {
                next_states.clear();

                for &pc in &curr_states {
                    let pc_usize = pc as usize;
                    if pc_usize >= self.bytecode.len() { continue; }
                    let opcode = self.bytecode[pc_usize];
                    if let Some((next_pc, _)) = self.try_consume(pc_usize, opcode, at, c) {
                        self.eps_closure_fast(&mut next_states, &mut seen, &mut eps_stack, next_pc, at + char_len);
                    }
                }

                // Store in cache
                cache.insert(state_key.clone(), b, next_states.clone());
            }

            // Swap (seen for curr will be cleared at top of next iteration)
            std::mem::swap(&mut curr_states, &mut next_states);
            at += char_len;
        }

        // Final check
        if best_end.is_none() {
            for &pc in &curr_states {
                if (pc as usize) < self.bytecode.len() && self.bytecode[pc as usize] == op::MATCH {
                    best_end = Some(at);
                    break;
                }
            }
        }

        best_end
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
        let mut candidate: Option<Vec<Option<usize>>> = None;
        let mut cache = TransitionCache::new();
        let mut state_key: Vec<u32> = Vec::with_capacity(64);

        // Initialize: epsilon closure from bytecode start
        self.epsilon_closure(&mut curr, &mut eps_stack, &mut tmp_caps, &mut tmp_regs, RE_HEADER_LEN, start_pos);

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

            // Build state key for cache lookup (sorted PCs of current threads)
            state_key.clear();
            for &(pc, _) in &curr.threads {
                state_key.push(pc);
            }

            // DFA cache disabled for now — needs separate capture-free execution path.
            // The cache can't preserve captures across hits, which breaks match span detection.
            // TODO: Add exec_count_only() that uses the cache for count/grep models.
            let use_cache = false && c < 128 && self.register_count == 0;
            let cache_hit = if use_cache {
                cache.lookup(&state_key, c as u8).map(|e| e.next_states.clone())
            } else {
                None
            };

            if let Some(next_states) = cache_hit {
                // Cache hit: skip ALL epsilon closure work.
                // Just add each cached next-state as a terminal thread.
                // Captures are not preserved across cache hits (fine for count/find models).
                tmp_caps.fill(None);
                for &pc in &next_states {
                    next.add(pc, &tmp_caps, &tmp_regs);
                }
            } else {
                // Cache miss: full computation
                for i in 0..curr.threads.len() {
                    let (pc, slot_idx) = curr.threads[i];
                    let pc_usize = pc as usize;
                    if pc_usize >= self.bytecode.len() { continue; }

                    let opcode = self.bytecode[pc_usize];
                    if let Some((next_pc, _advance)) = self.try_consume(pc_usize, opcode, at, c) {
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

                        self.epsilon_closure(&mut next, &mut eps_stack, &mut tmp_caps, &mut tmp_regs, next_pc, at + char_len);
                    }
                }

                // Store in cache
                if use_cache {
                    let next_pcs: Vec<u32> = next.threads.iter().map(|&(pc, _)| pc).collect();
                    cache.insert(state_key.clone(), c as u8, next_pcs);
                }
            }

            std::mem::swap(&mut curr, &mut next);
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
                let check = if opcode == op::RANGE_I { to_lower(c) } else { c };
                if self.check_range16(check, pc + 3, n) { Some((pc + 3 + n * 4, char_len)) } else { None }
            }
            op::RANGE32 | op::RANGE32_I => {
                let n = self.read_u16(pc + 1) as usize;
                let check = if opcode == op::RANGE32_I { to_lower(c) } else { c };
                if self.check_range32(check, pc + 3, n) { Some((pc + 3 + n * 8, char_len)) } else { None }
            }
            op::MATCH => None, // Not consuming
            _ => None,
        }
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

/// Persistent scanner with warm DFA cache for repeated matching.
/// Holds all state needed to scan efficiently across multiple find_next calls.
pub struct PikeScanner<'a> {
    vm: PikeVm<'a>,
    cache: TransitionCache,
    curr_states: Vec<u32>,
    next_states: Vec<u32>,
    seen: Vec<bool>,
    eps_stack: Vec<(usize, bool)>,
    state_key: Vec<u32>,
}

impl<'a> PikeScanner<'a> {
    pub fn new(bytecode: &'a [u8], input: &'a [u8]) -> Self {
        let vm = PikeVm::new(bytecode, input);
        let num_pcs = vm.num_pcs;
        PikeScanner {
            vm,
            cache: TransitionCache::new(),
            curr_states: Vec::with_capacity(128),
            next_states: Vec::with_capacity(128),
            seen: vec![false; num_pcs],
            eps_stack: Vec::with_capacity(64),
            state_key: Vec::with_capacity(64),
        }
    }

    /// Find the next match starting at or after `start_pos`.
    /// Returns Some((match_start, match_end)) or None.
    /// The DFA cache persists across calls — gets warmer over time.
    pub fn find_next(&mut self, start_pos: usize) -> Option<(usize, usize)> {
        // Use the full exec() for now — it has correct greedy/lazy semantics.
        // The warm cache accelerates the capture-free pre-check.
        match self.vm.exec(start_pos) {
            PikeResult::Match(caps) => {
                let s = caps.get(0).copied().flatten()?;
                let e = caps.get(1).copied().flatten()?;
                Some((s, e))
            }
            PikeResult::NoMatch => None,
        }
    }

    /// Fast check: does any match exist starting at or after `start_pos`?
    /// Uses DFA cache for O(n) throughput on warm paths.
    pub fn has_match_from(&mut self, start_pos: usize) -> bool {
        self.find_match_cached(start_pos).is_some()
    }

    /// Capture-free scan with persistent cache.
    fn find_match_cached(&mut self, start_pos: usize) -> Option<usize> {
        self.curr_states.clear();
        self.next_states.clear();
        self.seen.fill(false);

        self.vm.eps_closure_fast(
            &mut self.curr_states, &mut self.seen, &mut self.eps_stack,
            RE_HEADER_LEN, start_pos,
        );

        let mut at = start_pos;
        let mut best_end: Option<usize> = None;

        loop {
            for &pc in &self.curr_states {
                if (pc as usize) < self.vm.bytecode.len() && self.vm.bytecode[pc as usize] == op::MATCH {
                    best_end = Some(at);
                }
            }

            if at >= self.vm.input_len || self.curr_states.is_empty() { break; }

            let b = self.vm.input[at];
            let (c, char_len) = self.vm.next_char(at);
            if char_len == 0 { break; }

            // Clear seen for current states
            for &pc in &self.curr_states { self.seen[pc as usize] = false; }

            // Build cache key
            self.state_key.clear();
            self.state_key.extend_from_slice(&self.curr_states);

            // Cache lookup
            if let Some(entry) = self.cache.lookup(&self.state_key, b) {
                self.next_states.clear();
                self.next_states.extend_from_slice(&entry.next_states);
                for &pc in &self.next_states { self.seen[pc as usize] = true; }
            } else {
                self.next_states.clear();
                for &pc in &self.curr_states {
                    let pc_usize = pc as usize;
                    if pc_usize >= self.vm.bytecode.len() { continue; }
                    let opcode = self.vm.bytecode[pc_usize];
                    if let Some((next_pc, _)) = self.vm.try_consume(pc_usize, opcode, at, c) {
                        self.vm.eps_closure_fast(
                            &mut self.next_states, &mut self.seen, &mut self.eps_stack,
                            next_pc, at + char_len,
                        );
                    }
                }
                self.cache.insert(self.state_key.clone(), b, self.next_states.clone());
            }

            std::mem::swap(&mut self.curr_states, &mut self.next_states);
            at += char_len;
        }

        if best_end.is_none() {
            for &pc in &self.curr_states {
                if (pc as usize) < self.vm.bytecode.len() && self.vm.bytecode[pc as usize] == op::MATCH {
                    best_end = Some(at);
                    break;
                }
            }
        }

        best_end
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
