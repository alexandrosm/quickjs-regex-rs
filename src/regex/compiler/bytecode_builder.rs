//! Bytecode builder utilities for emitting QuickJS regex bytecode.

use crate::regex::opcodes::OpCode;
use crate::regex::util::ByteBuffer;

pub struct BytecodeBuilder {
    buf: ByteBuffer,
}

impl BytecodeBuilder {
    pub fn new() -> Self {
        Self {
            buf: ByteBuffer::new(),
        }
    }

    /// Current position in bytecode body (0-based, not counting header).
    pub fn pc(&self) -> usize {
        self.buf.len()
    }

    /// Emit a single opcode (1 byte).
    pub fn emit_op(&mut self, op: OpCode) {
        self.buf.push(op as u8);
    }

    /// Emit opcode + u8 operand (2 bytes total).
    pub fn emit_op_u8(&mut self, op: OpCode, val: u8) {
        self.buf.push(op as u8);
        self.buf.push(val);
    }

    /// Emit opcode + u16 operand (3 bytes total).
    pub fn emit_op_u16(&mut self, op: OpCode, val: u16) {
        self.buf.push(op as u8);
        self.buf.push_u16(val);
    }

    /// Emit opcode + u32 operand (5 bytes total).
    pub fn emit_op_u32(&mut self, op: OpCode, val: u32) {
        self.buf.push(op as u8);
        self.buf.push_u32(val);
    }

    /// Emit a goto/split instruction with a placeholder i32 offset.
    /// Returns the byte position of the offset field (for patching).
    pub fn emit_goto(&mut self, op: OpCode, _placeholder: i32) -> usize {
        self.buf.push(op as u8);
        let offset_pos = self.buf.len();
        self.buf.push_u32(0); // placeholder
        offset_pos
    }

    /// Patch a goto/split offset.
    ///
    /// The interpreter computes the jump target as:
    ///   new_pc = (offset_pos + 4) + offset
    /// where offset_pos+4 is the byte right after the i32 offset field.
    /// So: offset = target_pc - (offset_pos + 4)
    ///
    /// But these positions are in the bytecode body (not including header).
    /// The interpreter adds RE_HEADER_LEN=8 to get the absolute position,
    /// but since both source and target are in the same body, it cancels out.
    pub fn patch_goto(&mut self, offset_pos: usize, target_body_pc: usize) {
        let after_instr = offset_pos + 4;
        let offset = (target_body_pc as i32) - (after_instr as i32);
        let bytes = offset.to_le_bytes();
        let buf = self.buf.as_mut_slice();
        buf[offset_pos..offset_pos + 4].copy_from_slice(&bytes);
    }

    /// Emit a raw byte.
    pub fn push(&mut self, val: u8) {
        self.buf.push(val);
    }

    /// Emit a raw u16 (little-endian).
    pub fn push_u16(&mut self, val: u16) {
        self.buf.push_u16(val);
    }

    /// Emit a raw u32 (little-endian).
    pub fn push_u32(&mut self, val: u32) {
        self.buf.push_u32(val);
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.buf.into_vec()
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.buf.as_mut_slice()
    }
}

impl Default for BytecodeBuilder {
    fn default() -> Self {
        Self::new()
    }
}
