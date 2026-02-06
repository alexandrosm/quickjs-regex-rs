//! Code generation from regex-syntax HIR to QuickJS bytecode.

use super::{BytecodeBuilder, CompilerError, Result};
use crate::regex::{Flags, opcodes::OpCode};
use regex_syntax::hir::{self, Hir, HirKind, Look, Class, ClassUnicode, ClassBytes};

/// Compiles regex HIR to QuickJS bytecode.
pub struct CodeGenerator {
    builder: BytecodeBuilder,
    flags: Flags,
    capture_count: u32,
    register_count: u8,
}

impl CodeGenerator {
    pub fn new(flags: Flags) -> Self {
        Self {
            builder: BytecodeBuilder::new(),
            flags,
            capture_count: 0,
            register_count: 0,
        }
    }

    /// Count capture groups in HIR tree (needed for header).
    fn count_captures(hir: &Hir) -> u32 {
        let mut max_idx = 0u32;
        Self::walk_captures(hir, &mut max_idx);
        max_idx
    }

    fn walk_captures(hir: &Hir, max_idx: &mut u32) {
        match hir.kind() {
            HirKind::Capture(cap) => {
                if cap.index + 1 > *max_idx {
                    *max_idx = cap.index + 1;
                }
                Self::walk_captures(&cap.sub, max_idx);
            }
            HirKind::Concat(subs) | HirKind::Alternation(subs) => {
                for sub in subs {
                    Self::walk_captures(sub, max_idx);
                }
            }
            HirKind::Repetition(rep) => {
                Self::walk_captures(&rep.sub, max_idx);
            }
            _ => {}
        }
    }

    /// Main entry point: compile HIR to bytecode.
    pub fn compile(&mut self, hir: &Hir) -> Result<()> {
        // Count capture groups. regex-syntax reserves index 0 for the overall
        // match and assigns explicit groups starting at index 1.
        // count_captures returns max_index + 1.
        let max_capture = Self::count_captures(hir);
        // Total captures = max of (1 for group 0, max from explicit groups)
        self.capture_count = max_capture.max(1);

        // Emit prefix for non-sticky patterns: try matching at each position.
        // SplitGotoFirst: try GOTO target (pattern) first, on failure backtrack to NEXT (ANY).
        if !self.flags.contains(Flags::STICKY) {
            let split_start = self.builder.pc();
            let split_pc = self.builder.emit_goto(OpCode::SplitGotoFirst, 0);
            self.builder.emit_op(OpCode::Any);
            let goto_pc = self.builder.pc();
            self.builder.emit_goto(OpCode::Goto, 0);
            let pattern_start = self.builder.pc();
            // Patch GOTO to jump back to split_start
            self.builder.patch_goto(goto_pc + 1, split_start);
            // Patch SPLIT to jump to pattern_start
            self.builder.patch_goto(split_pc, pattern_start);
        }

        // Save capture group 0 start
        self.builder.emit_op_u8(OpCode::SaveStart, 0);

        // Compile pattern body
        self.compile_node(hir)?;

        // Save capture group 0 end
        self.builder.emit_op_u8(OpCode::SaveEnd, 0);

        // Emit MATCH
        self.builder.emit_op(OpCode::Match);

        Ok(())
    }

    /// Build final bytecode with 8-byte header.
    pub fn into_bytecode(self) -> Vec<u8> {
        let body = self.builder.into_vec();
        let mut result = Vec::with_capacity(8 + body.len());
        // Header: flags(u16) + capture_count(u8) + register_count(u8) + bytecode_len(u32)
        result.extend_from_slice(&self.flags.bits().to_le_bytes());
        result.push(self.capture_count as u8);
        result.push(self.register_count);
        result.extend_from_slice(&(body.len() as u32).to_le_bytes());
        result.extend_from_slice(&body);
        result
    }

    // ========================================================================
    // Node compilation
    // ========================================================================

    fn compile_node(&mut self, hir: &Hir) -> Result<()> {
        match hir.kind() {
            HirKind::Empty => Ok(()),
            HirKind::Literal(lit) => self.compile_literal(lit),
            HirKind::Class(class) => self.compile_class(class),
            HirKind::Look(look) => self.compile_look(look),
            HirKind::Repetition(rep) => self.compile_repetition(rep),
            HirKind::Capture(cap) => self.compile_capture(cap),
            HirKind::Concat(subs) => {
                for sub in subs {
                    self.compile_node(sub)?;
                }
                Ok(())
            }
            HirKind::Alternation(alts) => self.compile_alternation(alts),
        }
    }

    // ========================================================================
    // Literals
    // ========================================================================

    fn compile_literal(&mut self, lit: &hir::Literal) -> Result<()> {
        // Literal bytes are UTF-8 encoded
        let s = std::str::from_utf8(&lit.0)
            .map_err(|_| CompilerError::new("Invalid UTF-8 in literal"))?;

        for c in s.chars() {
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
        }
        Ok(())
    }

    // ========================================================================
    // Character classes
    // ========================================================================

    fn compile_class(&mut self, class: &Class) -> Result<()> {
        match class {
            Class::Unicode(cls) => self.compile_unicode_class(cls),
            Class::Bytes(cls) => self.compile_byte_class(cls),
        }
    }

    fn compile_unicode_class(&mut self, class: &ClassUnicode) -> Result<()> {
        let ranges: Vec<_> = class.ranges().to_vec();
        if ranges.is_empty() {
            self.builder.emit_op_u16(OpCode::Char, 0xFFFF);
            return Ok(());
        }

        // Detect Unicode dot: class covering all codepoints except line terminators
        if self.is_dot_unicode_class(&ranges) {
            if self.flags.contains(Flags::DOT_ALL) {
                self.builder.emit_op(OpCode::Any);
            } else {
                self.builder.emit_op(OpCode::Dot);
            }
            return Ok(());
        }

        // Check if all codepoints fit in u16
        let all_bmp = ranges.iter().all(|r| (r.end() as u32) <= 0xFFFF);

        if all_bmp {
            let op = if self.flags.contains(Flags::IGNORE_CASE) {
                OpCode::RangeI
            } else {
                OpCode::Range
            };
            self.builder.emit_op(op);
            self.builder.push_u16(ranges.len() as u16);
            for r in &ranges {
                self.builder.push_u16(r.start() as u16);
                self.builder.push_u16(r.end() as u16);
            }
        } else {
            let op = if self.flags.contains(Flags::IGNORE_CASE) {
                OpCode::Range32I
            } else {
                OpCode::Range32
            };
            self.builder.emit_op(op);
            self.builder.push_u16(ranges.len() as u16);
            for r in &ranges {
                self.builder.push_u32(r.start() as u32);
                self.builder.push_u32(r.end() as u32);
            }
        }

        Ok(())
    }

    fn compile_byte_class(&mut self, class: &ClassBytes) -> Result<()> {
        let ranges: Vec<_> = class.ranges().to_vec();
        if ranges.is_empty() {
            self.builder.emit_op_u16(OpCode::Char, 0xFFFF);
            return Ok(());
        }

        // Detect "dot" pattern: byte class matching everything except \n (0x0A).
        // regex-syntax with unicode(false) turns `.` into Class::Bytes([0x00-0x09, 0x0B-0xFF]).
        if self.is_dot_byte_class(&ranges) {
            if self.flags.contains(Flags::DOT_ALL) {
                self.builder.emit_op(OpCode::Any);
            } else {
                self.builder.emit_op(OpCode::Dot);
            }
            return Ok(());
        }

        // Detect "any" pattern (dot-all mode): [0x00-0xFF]
        if ranges.len() == 1 && ranges[0].start() == 0 && ranges[0].end() == 0xFF {
            self.builder.emit_op(OpCode::Any);
            return Ok(());
        }

        // Regular byte ranges - fit in u16
        let op = if self.flags.contains(Flags::IGNORE_CASE) {
            OpCode::RangeI
        } else {
            OpCode::Range
        };
        self.builder.emit_op(op);
        self.builder.push_u16(ranges.len() as u16);
        for r in &ranges {
            self.builder.push_u16(r.start() as u16);
            self.builder.push_u16(r.end() as u16);
        }

        Ok(())
    }

    /// Check if a Unicode class represents "dot" (excludes line terminators).
    fn is_dot_unicode_class(&self, ranges: &[hir::ClassUnicodeRange]) -> bool {
        // Unicode dot in regex-syntax: excludes \n(0x0A), \r(0x0D), \u2028, \u2029
        // Typically produces ranges like: [0x0000-0x0009, 0x000B-0x000C, 0x000E-0x2027, 0x202A-0x10FFFF]
        if ranges.len() >= 3 {
            let first = &ranges[0];
            let last = &ranges[ranges.len() - 1];
            if first.start() == '\0' && last.end() == '\u{10FFFF}' {
                return true;
            }
        }
        false
    }

    /// Check if a byte class represents the "dot" pattern (any byte except \n, \r, \u2028, \u2029).
    fn is_dot_byte_class(&self, ranges: &[hir::ClassBytesRange]) -> bool {
        // regex-syntax dot with unicode(false) = [0x00-0x09, 0x0B-0xFF] (2 ranges, excludes 0x0A)
        if ranges.len() == 2
            && ranges[0].start() == 0x00 && ranges[0].end() == 0x09
            && ranges[1].start() == 0x0B && ranges[1].end() == 0xFF
        {
            return true;
        }
        // Also match [0x00-0x09, 0x0B-0x0C, 0x0E-0xFF] (excludes \n and \r)
        if ranges.len() == 3
            && ranges[0].start() == 0x00 && ranges[0].end() == 0x09
            && ranges[1].start() == 0x0B && ranges[1].end() == 0x0C
            && ranges[2].start() == 0x0E && ranges[2].end() == 0xFF
        {
            return true;
        }
        false
    }

    // ========================================================================
    // Look-around assertions
    // ========================================================================

    fn compile_look(&mut self, look: &Look) -> Result<()> {
        match look {
            // Non-multiline anchors
            Look::Start => self.builder.emit_op(OpCode::LineStart),
            Look::End => self.builder.emit_op(OpCode::LineEnd),
            // Multiline anchors
            Look::StartLF | Look::StartCRLF => self.builder.emit_op(OpCode::LineStartM),
            Look::EndLF | Look::EndCRLF => self.builder.emit_op(OpCode::LineEndM),
            // Word boundaries
            Look::WordAscii => self.builder.emit_op(OpCode::WordBoundary),
            Look::WordAsciiNegate => self.builder.emit_op(OpCode::NotWordBoundary),
            Look::WordUnicode => self.builder.emit_op(OpCode::WordBoundary),
            Look::WordUnicodeNegate => self.builder.emit_op(OpCode::NotWordBoundary),
            _ => {
                return Err(CompilerError::new(format!(
                    "Unsupported look-around: {:?}", look
                )));
            }
        }
        Ok(())
    }

    // ========================================================================
    // Repetition
    // ========================================================================

    fn compile_repetition(&mut self, rep: &hir::Repetition) -> Result<()> {
        let min = rep.min;
        let max = rep.max;
        let greedy = rep.greedy;

        match (min, max) {
            // ? = {0,1}
            (0, Some(1)) => {
                let split_op = if greedy {
                    OpCode::SplitNextFirst
                } else {
                    OpCode::SplitGotoFirst
                };
                let split_pc = self.builder.emit_goto(split_op, 0);
                self.compile_node(&rep.sub)?;
                let end_pc = self.builder.pc();
                self.builder.patch_goto(split_pc, end_pc);
            }
            // * = {0,}
            (0, None) => {
                let split_op = if greedy {
                    OpCode::SplitNextFirst
                } else {
                    OpCode::SplitGotoFirst
                };
                let loop_start = self.builder.pc();
                let split_pc = self.builder.emit_goto(split_op, 0);
                self.compile_node(&rep.sub)?;
                // GOTO back to loop_start
                let goto_pc = self.builder.pc();
                self.builder.emit_goto(OpCode::Goto, 0);
                self.builder.patch_goto(goto_pc + 1, loop_start);
                // Patch SPLIT to skip past GOTO
                let end_pc = self.builder.pc();
                self.builder.patch_goto(split_pc, end_pc);
            }
            // + = {1,}
            (1, None) => {
                let split_op = if greedy {
                    OpCode::SplitGotoFirst
                } else {
                    OpCode::SplitNextFirst
                };
                let loop_start = self.builder.pc();
                self.compile_node(&rep.sub)?;
                let split_pc = self.builder.emit_goto(split_op, 0);
                self.builder.patch_goto(split_pc, loop_start);
            }
            // {n} = exact
            (n, Some(m)) if n == m => {
                for _ in 0..n {
                    self.compile_node(&rep.sub)?;
                }
            }
            // {n,m} = bounded
            (n, Some(m)) => {
                // Emit mandatory part
                for _ in 0..n {
                    self.compile_node(&rep.sub)?;
                }
                // Emit optional part (m-n times)
                for _ in n..m {
                    let split_op = if greedy {
                        OpCode::SplitNextFirst
                    } else {
                        OpCode::SplitGotoFirst
                    };
                    let split_pc = self.builder.emit_goto(split_op, 0);
                    self.compile_node(&rep.sub)?;
                    let end_pc = self.builder.pc();
                    self.builder.patch_goto(split_pc, end_pc);
                }
            }
            // {n,} = at least n
            (n, None) => {
                // Emit mandatory part
                for _ in 0..n {
                    self.compile_node(&rep.sub)?;
                }
                // Then emit * for the rest
                let split_op = if greedy {
                    OpCode::SplitNextFirst
                } else {
                    OpCode::SplitGotoFirst
                };
                let loop_start = self.builder.pc();
                let split_pc = self.builder.emit_goto(split_op, 0);
                self.compile_node(&rep.sub)?;
                let goto_pc = self.builder.pc();
                self.builder.emit_goto(OpCode::Goto, 0);
                self.builder.patch_goto(goto_pc + 1, loop_start);
                let end_pc = self.builder.pc();
                self.builder.patch_goto(split_pc, end_pc);
            }
        }

        Ok(())
    }

    // ========================================================================
    // Capture groups
    // ========================================================================

    fn compile_capture(&mut self, cap: &hir::Capture) -> Result<()> {
        // regex-syntax uses 0-based indices but reserves 0 for the implicit
        // overall match group. Explicit groups start at index 1.
        // We use cap.index directly as the save slot.
        let group_idx = cap.index as u8;
        self.builder.emit_op_u8(OpCode::SaveStart, group_idx);
        self.compile_node(&cap.sub)?;
        self.builder.emit_op_u8(OpCode::SaveEnd, group_idx);
        Ok(())
    }

    // ========================================================================
    // Alternation
    // ========================================================================

    fn compile_alternation(&mut self, alts: &[Hir]) -> Result<()> {
        if alts.len() == 1 {
            return self.compile_node(&alts[0]);
        }

        // For N alternatives, emit N-1 SPLIT instructions followed by GOTOs.
        // Structure:
        //   SPLIT_NEXT_FIRST alt2
        //   <alt1>
        //   GOTO end
        //   SPLIT_NEXT_FIRST alt3    <- target of first SPLIT
        //   <alt2>
        //   GOTO end
        //   <alt3>                   <- target of second SPLIT (last alt, no SPLIT)
        //   end:

        let mut goto_patches = Vec::new();

        for (i, alt) in alts.iter().enumerate() {
            let is_last = i == alts.len() - 1;

            if !is_last {
                // SPLIT_NEXT_FIRST to the next alternative
                let split_pc = self.builder.emit_goto(OpCode::SplitNextFirst, 0);

                // Compile this alternative
                self.compile_node(alt)?;

                // GOTO end
                let goto_pc = self.builder.emit_goto(OpCode::Goto, 0);
                goto_patches.push(goto_pc);

                // Patch SPLIT to point to here (start of next alternative)
                let next_alt_pc = self.builder.pc();
                self.builder.patch_goto(split_pc, next_alt_pc);
            } else {
                // Last alternative: just compile it
                self.compile_node(alt)?;
            }
        }

        // Patch all GOTOs to point to end
        let end_pc = self.builder.pc();
        for goto_pc in goto_patches {
            self.builder.patch_goto(goto_pc, end_pc);
        }

        Ok(())
    }
}
