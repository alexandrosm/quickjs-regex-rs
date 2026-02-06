//! Code generation from custom JS regex AST to QuickJS bytecode.

use super::{BytecodeBuilder, CompilerError, Result};
use super::parser::{Node, AnchorKind, BuiltinClass, ClassRange};
use crate::regex::{Flags, opcodes::OpCode};

pub struct CodeGenerator {
    builder: BytecodeBuilder,
    flags: Flags,
    capture_count: u32,
    register_count: u8,
}

impl CodeGenerator {
    pub fn new(flags: Flags, capture_count: u32) -> Self {
        Self {
            builder: BytecodeBuilder::new(),
            flags,
            capture_count,
            register_count: 0,
        }
    }

    pub fn compile(&mut self, ast: &Node) -> Result<()> {
        // Emit prefix for non-sticky patterns: try matching at each position
        if !self.flags.contains(Flags::STICKY) {
            let split_start = self.builder.pc();
            let split_pc = self.builder.emit_goto(OpCode::SplitGotoFirst, 0);
            self.builder.emit_op(OpCode::Any);
            let goto_pc = self.builder.pc();
            self.builder.emit_goto(OpCode::Goto, 0);
            let pattern_start = self.builder.pc();
            self.builder.patch_goto(goto_pc + 1, split_start);
            self.builder.patch_goto(split_pc, pattern_start);
        }

        // Save capture group 0
        self.builder.emit_op_u8(OpCode::SaveStart, 0);
        self.compile_node(ast)?;
        self.builder.emit_op_u8(OpCode::SaveEnd, 0);
        self.builder.emit_op(OpCode::Match);

        Ok(())
    }

    pub fn into_bytecode(self) -> Vec<u8> {
        let body = self.builder.into_vec();
        let mut result = Vec::with_capacity(8 + body.len());
        result.extend_from_slice(&self.flags.bits().to_le_bytes());
        result.push((self.capture_count + 1) as u8); // +1 for group 0
        result.push(self.register_count);
        result.extend_from_slice(&(body.len() as u32).to_le_bytes());
        result.extend_from_slice(&body);
        result
    }

    // ========================================================================
    // Node dispatch
    // ========================================================================

    fn compile_node(&mut self, node: &Node) -> Result<()> {
        match node {
            Node::Empty => Ok(()),
            Node::Literal(c) => self.compile_literal(*c),
            Node::Dot => self.compile_dot(),
            Node::Class { ranges, negated } => self.compile_class(ranges, *negated),
            Node::Builtin(cls) => self.compile_builtin(*cls),
            Node::Anchor(kind) => self.compile_anchor(*kind),
            Node::WordBoundary { negated } => self.compile_word_boundary(*negated),
            Node::BackRef(n) => self.compile_backref(*n),
            Node::Lookahead { sub, negative } => self.compile_lookahead(sub, *negative),
            Node::Lookbehind { sub, negative } => self.compile_lookbehind(sub, *negative),
            Node::Capture { index, sub, .. } => self.compile_capture(*index, sub),
            Node::Group(sub) => self.compile_node(sub),
            Node::Repeat { sub, min, max, greedy } => {
                self.compile_repeat(sub, *min, *max, *greedy)
            }
            Node::Concat(nodes) => {
                for n in nodes { self.compile_node(n)?; }
                Ok(())
            }
            Node::Alternation(alts) => self.compile_alternation(alts),
        }
    }

    // ========================================================================
    // Literals
    // ========================================================================

    fn compile_literal(&mut self, c: char) -> Result<()> {
        let code = c as u32;
        if code <= 0xFFFF {
            if self.flags.contains(Flags::IGNORE_CASE) {
                self.builder.emit_op_u16(OpCode::CharI, code as u16);
            } else {
                self.builder.emit_op_u16(OpCode::Char, code as u16);
            }
        } else {
            if self.flags.contains(Flags::IGNORE_CASE) {
                self.builder.emit_op_u32(OpCode::Char32I, code);
            } else {
                self.builder.emit_op_u32(OpCode::Char32, code);
            }
        }
        Ok(())
    }

    fn compile_dot(&mut self) -> Result<()> {
        if self.flags.contains(Flags::DOT_ALL) {
            self.builder.emit_op(OpCode::Any);
        } else {
            self.builder.emit_op(OpCode::Dot);
        }
        Ok(())
    }

    // ========================================================================
    // Character classes
    // ========================================================================

    fn compile_class(&mut self, ranges: &[ClassRange], negated: bool) -> Result<()> {
        // Expand class ranges into (lo, hi) pairs, handling builtins
        let mut pairs: Vec<(u32, u32)> = Vec::new();
        let mut has_builtins = false;

        for r in ranges {
            match r {
                ClassRange::Single(c) => pairs.push((*c as u32, *c as u32)),
                ClassRange::Range(lo, hi) => pairs.push((*lo as u32, *hi as u32)),
                ClassRange::Builtin(_) => has_builtins = true,
            }
        }

        // If we have builtins mixed with ranges, emit as alternation
        if has_builtins {
            return self.compile_class_with_builtins(ranges, negated);
        }

        if negated {
            // Negate the ranges: compute complement over [0, 0x10FFFF]
            pairs.sort_by_key(|p| p.0);
            let mut neg_pairs = Vec::new();
            let mut prev = 0u32;
            for &(lo, hi) in &pairs {
                if lo > prev {
                    neg_pairs.push((prev, lo - 1));
                }
                prev = hi + 1;
            }
            if prev <= 0x10FFFF {
                neg_pairs.push((prev, 0x10FFFF));
            }
            pairs = neg_pairs;
        }

        if pairs.is_empty() {
            // Empty class - never matches
            self.builder.emit_op_u16(OpCode::Char, 0xFFFF);
            return Ok(());
        }

        let all_bmp = pairs.iter().all(|&(_, hi)| hi <= 0xFFFF);
        if all_bmp {
            let op = if self.flags.contains(Flags::IGNORE_CASE) {
                OpCode::RangeI
            } else {
                OpCode::Range
            };
            self.builder.emit_op(op);
            self.builder.push_u16(pairs.len() as u16);
            for &(lo, hi) in &pairs {
                self.builder.push_u16(lo as u16);
                self.builder.push_u16(hi as u16);
            }
        } else {
            let op = if self.flags.contains(Flags::IGNORE_CASE) {
                OpCode::Range32I
            } else {
                OpCode::Range32
            };
            self.builder.emit_op(op);
            self.builder.push_u16(pairs.len() as u16);
            for &(lo, hi) in &pairs {
                self.builder.push_u32(lo);
                self.builder.push_u32(hi);
            }
        }
        Ok(())
    }

    fn compile_class_with_builtins(&mut self, ranges: &[ClassRange], _negated: bool) -> Result<()> {
        // Emit as alternation of individual items
        let items: Vec<&ClassRange> = ranges.iter().collect();
        if items.len() == 1 {
            return self.compile_class_item(&items[0]);
        }

        let mut goto_patches = Vec::new();
        for (i, item) in items.iter().enumerate() {
            let is_last = i == items.len() - 1;
            if !is_last {
                let split_pc = self.builder.emit_goto(OpCode::SplitNextFirst, 0);
                self.compile_class_item(item)?;
                let goto_pc = self.builder.emit_goto(OpCode::Goto, 0);
                goto_patches.push(goto_pc);
                let next_pc = self.builder.pc();
                self.builder.patch_goto(split_pc, next_pc);
            } else {
                self.compile_class_item(item)?;
            }
        }
        let end_pc = self.builder.pc();
        for goto_pc in goto_patches {
            self.builder.patch_goto(goto_pc, end_pc);
        }
        Ok(())
    }

    fn compile_class_item(&mut self, item: &ClassRange) -> Result<()> {
        match item {
            ClassRange::Single(c) => self.compile_literal(*c),
            ClassRange::Range(lo, hi) => {
                let lo = *lo as u32;
                let hi = *hi as u32;
                if lo <= 0xFFFF && hi <= 0xFFFF {
                    self.builder.emit_op(OpCode::Range);
                    self.builder.push_u16(1);
                    self.builder.push_u16(lo as u16);
                    self.builder.push_u16(hi as u16);
                } else {
                    self.builder.emit_op(OpCode::Range32);
                    self.builder.push_u16(1);
                    self.builder.push_u32(lo);
                    self.builder.push_u32(hi);
                }
                Ok(())
            }
            ClassRange::Builtin(cls) => self.compile_builtin(*cls),
        }
    }

    // ========================================================================
    // Built-in character classes (\d, \w, \s)
    // ========================================================================

    fn compile_builtin(&mut self, cls: BuiltinClass) -> Result<()> {
        match cls {
            BuiltinClass::Space => self.builder.emit_op(OpCode::Space),
            BuiltinClass::NotSpace => self.builder.emit_op(OpCode::NotSpace),
            BuiltinClass::Digit => {
                // \d = [0-9]
                self.builder.emit_op(OpCode::Range);
                self.builder.push_u16(1);
                self.builder.push_u16(0x30); // '0'
                self.builder.push_u16(0x39); // '9'
            }
            BuiltinClass::NotDigit => {
                // \D = [^0-9] = [0x00-0x2F, 0x3A-0xFFFF]
                self.builder.emit_op(OpCode::Range);
                self.builder.push_u16(2);
                self.builder.push_u16(0x00);
                self.builder.push_u16(0x2F);
                self.builder.push_u16(0x3A);
                self.builder.push_u16(0xFFFF);
            }
            BuiltinClass::Word => {
                // \w = [0-9A-Z_a-z]
                self.builder.emit_op(OpCode::Range);
                self.builder.push_u16(4);
                self.builder.push_u16(0x30); self.builder.push_u16(0x39); // 0-9
                self.builder.push_u16(0x41); self.builder.push_u16(0x5A); // A-Z
                self.builder.push_u16(0x5F); self.builder.push_u16(0x5F); // _
                self.builder.push_u16(0x61); self.builder.push_u16(0x7A); // a-z
            }
            BuiltinClass::NotWord => {
                // \W = [^0-9A-Z_a-z]
                self.builder.emit_op(OpCode::Range);
                self.builder.push_u16(5);
                self.builder.push_u16(0x00); self.builder.push_u16(0x2F);
                self.builder.push_u16(0x3A); self.builder.push_u16(0x40);
                self.builder.push_u16(0x5B); self.builder.push_u16(0x5E);
                self.builder.push_u16(0x60); self.builder.push_u16(0x60);
                self.builder.push_u16(0x7B); self.builder.push_u16(0xFFFF);
            }
        }
        Ok(())
    }

    // ========================================================================
    // Anchors and word boundaries
    // ========================================================================

    fn compile_anchor(&mut self, kind: AnchorKind) -> Result<()> {
        match kind {
            AnchorKind::Start => {
                if self.flags.contains(Flags::MULTILINE) {
                    self.builder.emit_op(OpCode::LineStartM);
                } else {
                    self.builder.emit_op(OpCode::LineStart);
                }
            }
            AnchorKind::End => {
                if self.flags.contains(Flags::MULTILINE) {
                    self.builder.emit_op(OpCode::LineEndM);
                } else {
                    self.builder.emit_op(OpCode::LineEnd);
                }
            }
        }
        Ok(())
    }

    fn compile_word_boundary(&mut self, negated: bool) -> Result<()> {
        if negated {
            self.builder.emit_op(OpCode::NotWordBoundary);
        } else {
            self.builder.emit_op(OpCode::WordBoundary);
        }
        Ok(())
    }

    // ========================================================================
    // Backreferences
    // ========================================================================

    fn compile_backref(&mut self, group: u32) -> Result<()> {
        if self.flags.contains(Flags::IGNORE_CASE) {
            self.builder.emit_op(OpCode::BackReferenceI);
        } else {
            self.builder.emit_op(OpCode::BackReference);
        }
        self.builder.push(1); // count = 1 group
        self.builder.push(group as u8); // group index
        Ok(())
    }

    // ========================================================================
    // Lookahead / Lookbehind
    // ========================================================================

    fn compile_lookahead(&mut self, sub: &Node, negative: bool) -> Result<()> {
        let op = if negative {
            OpCode::NegativeLookahead
        } else {
            OpCode::Lookahead
        };
        let lookahead_pc = self.builder.emit_goto(op, 0);

        // Compile the lookahead pattern
        self.compile_node(sub)?;

        // Emit match signal
        if negative {
            self.builder.emit_op(OpCode::NegativeLookaheadMatch);
        } else {
            self.builder.emit_op(OpCode::LookaheadMatch);
        }

        // Patch the lookahead offset to point past the match signal
        let end_pc = self.builder.pc();
        self.builder.patch_goto(lookahead_pc, end_pc);

        Ok(())
    }

    fn compile_lookbehind(&mut self, sub: &Node, negative: bool) -> Result<()> {
        // Lookbehind is implemented as: save position, move backward through
        // the pattern, then verify. For now, emit as a lookahead with Prev
        // prefix (simplified — full lookbehind needs reversed pattern compilation).
        //
        // For simple lookbehind like (?<=abc), we emit:
        //   Lookahead <end>
        //   Prev; Prev; Prev   (move back 3 chars)
        //   Char 'a'; Char 'b'; Char 'c'  (match forward)
        //   LookaheadMatch
        //
        // For complex lookbehind, this is an approximation.
        let op = if negative {
            OpCode::NegativeLookahead
        } else {
            OpCode::Lookahead
        };
        let lookahead_pc = self.builder.emit_goto(op, 0);

        // Count how many chars we need to go back
        let char_count = self.count_fixed_chars(sub);

        // Emit Prev instructions to move backward
        for _ in 0..char_count {
            self.builder.emit_op(OpCode::Prev);
        }

        // Compile the pattern (matching forward from the backed-up position)
        self.compile_node(sub)?;

        if negative {
            self.builder.emit_op(OpCode::NegativeLookaheadMatch);
        } else {
            self.builder.emit_op(OpCode::LookaheadMatch);
        }

        let end_pc = self.builder.pc();
        self.builder.patch_goto(lookahead_pc, end_pc);

        Ok(())
    }

    /// Count the number of fixed-width characters in a node (for lookbehind).
    fn count_fixed_chars(&self, node: &Node) -> u32 {
        match node {
            Node::Literal(_) | Node::Dot | Node::Builtin(_) => 1,
            Node::Class { .. } => 1,
            Node::Concat(nodes) => nodes.iter().map(|n| self.count_fixed_chars(n)).sum(),
            Node::Group(sub) | Node::Capture { sub, .. } => self.count_fixed_chars(sub),
            Node::Alternation(alts) => {
                // All alternatives must have same fixed length for lookbehind
                alts.first().map(|a| self.count_fixed_chars(a)).unwrap_or(0)
            }
            _ => 0,
        }
    }

    // ========================================================================
    // Capture groups
    // ========================================================================

    fn compile_capture(&mut self, index: u32, sub: &Node) -> Result<()> {
        let group_idx = index as u8;
        self.builder.emit_op_u8(OpCode::SaveStart, group_idx);
        self.compile_node(sub)?;
        self.builder.emit_op_u8(OpCode::SaveEnd, group_idx);
        Ok(())
    }

    // ========================================================================
    // Repetition
    // ========================================================================

    fn compile_repeat(&mut self, sub: &Node, min: u32, max: Option<u32>, greedy: bool) -> Result<()> {
        // Find capture groups inside the sub-expression for SaveReset (JS semantics:
        // captures inside a loop are reset at each iteration)
        let captures = self.find_captures(sub);

        match (min, max) {
            (0, Some(1)) => { // ?
                let split_op = if greedy { OpCode::SplitNextFirst } else { OpCode::SplitGotoFirst };
                let split_pc = self.builder.emit_goto(split_op, 0);
                self.compile_node(sub)?;
                let end_pc = self.builder.pc();
                self.builder.patch_goto(split_pc, end_pc);
            }
            (0, None) => { // *
                let split_op = if greedy { OpCode::SplitNextFirst } else { OpCode::SplitGotoFirst };
                let loop_start = self.builder.pc();
                let split_pc = self.builder.emit_goto(split_op, 0);
                self.emit_save_reset(&captures);
                self.compile_node(sub)?;
                let goto_pc = self.builder.pc();
                self.builder.emit_goto(OpCode::Goto, 0);
                self.builder.patch_goto(goto_pc + 1, loop_start);
                let end_pc = self.builder.pc();
                self.builder.patch_goto(split_pc, end_pc);
            }
            (1, None) => { // +
                let split_op = if greedy { OpCode::SplitGotoFirst } else { OpCode::SplitNextFirst };
                let loop_start = self.builder.pc();
                self.emit_save_reset(&captures);
                self.compile_node(sub)?;
                let split_pc = self.builder.emit_goto(split_op, 0);
                self.builder.patch_goto(split_pc, loop_start);
            }
            (n, Some(m)) if n == m => { // {n}
                for _ in 0..n { self.compile_node(sub)?; }
            }
            (n, Some(m)) => { // {n,m}
                for _ in 0..n { self.compile_node(sub)?; }
                for _ in n..m {
                    let split_op = if greedy { OpCode::SplitNextFirst } else { OpCode::SplitGotoFirst };
                    let split_pc = self.builder.emit_goto(split_op, 0);
                    self.compile_node(sub)?;
                    let end_pc = self.builder.pc();
                    self.builder.patch_goto(split_pc, end_pc);
                }
            }
            (n, None) => { // {n,}
                for _ in 0..n { self.compile_node(sub)?; }
                let split_op = if greedy { OpCode::SplitNextFirst } else { OpCode::SplitGotoFirst };
                let loop_start = self.builder.pc();
                let split_pc = self.builder.emit_goto(split_op, 0);
                self.emit_save_reset(&captures);
                self.compile_node(sub)?;
                let goto_pc = self.builder.pc();
                self.builder.emit_goto(OpCode::Goto, 0);
                self.builder.patch_goto(goto_pc + 1, loop_start);
                let end_pc = self.builder.pc();
                self.builder.patch_goto(split_pc, end_pc);
            }
        }
        Ok(())
    }

    /// Find all capture group indices inside a node.
    fn find_captures(&self, node: &Node) -> Vec<u32> {
        let mut caps = Vec::new();
        self.walk_captures(node, &mut caps);
        caps
    }

    fn walk_captures(&self, node: &Node, caps: &mut Vec<u32>) {
        match node {
            Node::Capture { index, sub, .. } => {
                caps.push(*index);
                self.walk_captures(sub, caps);
            }
            Node::Concat(nodes) | Node::Alternation(nodes) => {
                for n in nodes { self.walk_captures(n, caps); }
            }
            Node::Group(sub) => self.walk_captures(sub, caps),
            Node::Repeat { sub, .. } => self.walk_captures(sub, caps),
            Node::Lookahead { sub, .. } | Node::Lookbehind { sub, .. } => {
                self.walk_captures(sub, caps);
            }
            _ => {}
        }
    }

    /// Emit SaveReset opcodes for captures inside a loop body (JS semantics).
    fn emit_save_reset(&mut self, captures: &[u32]) {
        if captures.is_empty() { return; }
        let min_cap = *captures.iter().min().unwrap();
        let max_cap = *captures.iter().max().unwrap();
        self.builder.emit_op(OpCode::SaveReset);
        self.builder.push(min_cap as u8);
        self.builder.push(max_cap as u8);
    }

    // ========================================================================
    // Alternation
    // ========================================================================

    fn compile_alternation(&mut self, alts: &[Node]) -> Result<()> {
        if alts.len() == 1 {
            return self.compile_node(&alts[0]);
        }

        let mut goto_patches = Vec::new();
        for (i, alt) in alts.iter().enumerate() {
            let is_last = i == alts.len() - 1;
            if !is_last {
                let split_pc = self.builder.emit_goto(OpCode::SplitNextFirst, 0);
                self.compile_node(alt)?;
                let goto_pc = self.builder.emit_goto(OpCode::Goto, 0);
                goto_patches.push(goto_pc);
                let next_pc = self.builder.pc();
                self.builder.patch_goto(split_pc, next_pc);
            } else {
                self.compile_node(alt)?;
            }
        }
        let end_pc = self.builder.pc();
        for goto_pc in goto_patches {
            self.builder.patch_goto(goto_pc, end_pc);
        }
        Ok(())
    }
}
