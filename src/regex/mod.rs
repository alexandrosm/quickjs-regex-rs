//! High-level regex API
//!
//! This module provides a safe, idiomatic Rust API built on top of
//! the c2rust-translated QuickJS regex engine.

mod opcodes;
mod flags;
mod error;

pub use opcodes::OpCode;
pub use flags::{Flags, InvalidFlag};
pub use error::{Error, Result, ExecResult};

// TODO: Phase 3+ - Add safe wrappers around the generated code
// For now, the generated module is private and we'll build up
// the public API incrementally.
