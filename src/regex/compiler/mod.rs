//! Pure Rust regex compiler
//!
//! Replaces the C-based lre_compile with a pure Rust implementation.
//! Parses regex patterns using regex-syntax and compiles to QuickJS bytecode.

mod bytecode_builder;
mod codegen;

use crate::regex::Flags;
use regex_syntax::ParserBuilder;
use std::error::Error as StdError;
use std::fmt;

pub use bytecode_builder::BytecodeBuilder;
pub use codegen::CodeGenerator;

#[derive(Debug, Clone)]
pub struct CompilerError {
    message: String,
}

impl CompilerError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Regex compiler error: {}", self.message)
    }
}

impl StdError for CompilerError {}

pub type Result<T> = std::result::Result<T, CompilerError>;

/// Compile a regex pattern to bytecode (pure Rust replacement for lre_compile)
pub fn compile_regex(pattern: &str, flags: Flags) -> Result<Vec<u8>> {
    let is_unicode = flags.contains(Flags::UNICODE);
    let hir = ParserBuilder::new()
        .unicode(is_unicode)
        .case_insensitive(flags.contains(Flags::IGNORE_CASE))
        .multi_line(flags.contains(Flags::MULTILINE))
        .dot_matches_new_line(flags.contains(Flags::DOT_ALL))
        .utf8(is_unicode) // Only enforce UTF-8 validity in Unicode mode
        .build()
        .parse(pattern)
        .map_err(|e| CompilerError::new(format!("Parse error: {}", e)))?;

    let mut codegen = CodeGenerator::new(flags);
    codegen.compile(&hir)?;
    Ok(codegen.into_bytecode())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::regex::interpreter::{ExecContext, ExecResult};

    fn compile_and_match(pattern: &str, flags: Flags, text: &str) -> bool {
        let bytecode = compile_regex(pattern, flags).expect("compile failed");
        let text_bytes = text.as_bytes();
        let mut ctx = ExecContext::new(&bytecode, text_bytes);
        matches!(ctx.exec(0), ExecResult::Match)
    }

    fn compile_and_find(pattern: &str, flags: Flags, text: &str) -> Option<(usize, usize)> {
        let bytecode = compile_regex(pattern, flags).expect("compile failed");
        let text_bytes = text.as_bytes();
        let mut ctx = ExecContext::new(&bytecode, text_bytes);
        let mut pos = 0;
        while pos <= text.len() {
            match ctx.exec(pos) {
                ExecResult::Match => {
                    let start = ctx.captures.get(0).copied().flatten().unwrap_or(0);
                    let end = ctx.captures.get(1).copied().flatten().unwrap_or(0);
                    return Some((start, end));
                }
                ExecResult::NoMatch => {
                    if pos < text.len() {
                        pos += text[pos..].chars().next().map(|c| c.len_utf8()).unwrap_or(1);
                    } else {
                        break;
                    }
                    ctx.reset();
                }
            }
        }
        None
    }

    #[test]
    fn test_literal() {
        assert!(compile_and_match("abc", Flags::empty(), "abc"));
        assert!(compile_and_match("abc", Flags::empty(), "xabcx"));
        assert!(!compile_and_match("abc", Flags::empty(), "abd"));
    }

    #[test]
    fn test_dot() {
        assert!(compile_and_match("a.c", Flags::empty(), "abc"));
        assert!(compile_and_match("a.c", Flags::empty(), "axc"));
        assert!(!compile_and_match("a.c", Flags::empty(), "a\nc"));
    }

    #[test]
    fn test_alternation() {
        assert!(compile_and_match("cat|dog", Flags::empty(), "cat"));
        assert!(compile_and_match("cat|dog", Flags::empty(), "dog"));
        assert!(!compile_and_match("cat|dog", Flags::empty(), "cow"));
    }

    #[test]
    fn test_question_mark() {
        assert!(compile_and_match("ab?c", Flags::empty(), "ac"));
        assert!(compile_and_match("ab?c", Flags::empty(), "abc"));
        assert!(!compile_and_match("ab?c", Flags::empty(), "abbc"));
    }

    #[test]
    fn test_star() {
        assert!(compile_and_match("ab*c", Flags::empty(), "ac"));
        assert!(compile_and_match("ab*c", Flags::empty(), "abc"));
        assert!(compile_and_match("ab*c", Flags::empty(), "abbc"));
    }

    #[test]
    fn test_plus() {
        assert!(!compile_and_match("^ab+c$", Flags::empty(), "ac"));
        assert!(compile_and_match("ab+c", Flags::empty(), "abc"));
        assert!(compile_and_match("ab+c", Flags::empty(), "abbc"));
    }

    #[test]
    fn test_char_class() {
        assert!(compile_and_match("[abc]", Flags::empty(), "a"));
        assert!(compile_and_match("[abc]", Flags::empty(), "b"));
        assert!(!compile_and_match("^[abc]$", Flags::empty(), "d"));
    }

    #[test]
    fn test_find_position() {
        let m = compile_and_find("world", Flags::empty(), "hello world");
        assert_eq!(m, Some((6, 11)));
    }

    #[test]
    fn test_capture_group() {
        let bytecode = compile_regex("(a)(b)", Flags::empty()).expect("compile failed");
        let text = b"ab";
        let mut ctx = ExecContext::new(&bytecode, text);
        assert!(matches!(ctx.exec(0), ExecResult::Match));
        // Capture 0 = whole match
        assert_eq!(ctx.captures[0], Some(0));
        assert_eq!(ctx.captures[1], Some(2));
        // Capture 1 = first group
        assert_eq!(ctx.captures[2], Some(0));
        assert_eq!(ctx.captures[3], Some(1));
        // Capture 2 = second group
        assert_eq!(ctx.captures[4], Some(1));
        assert_eq!(ctx.captures[5], Some(2));
    }

    #[test]
    fn test_anchors() {
        assert!(compile_and_match("^abc", Flags::empty(), "abc"));
        assert!(!compile_and_match("^abc", Flags::empty(), "xabc"));
        assert!(compile_and_match("abc$", Flags::empty(), "abc"));
        assert!(!compile_and_match("abc$", Flags::empty(), "abcx"));
    }

    #[test]
    fn test_digit_class() {
        assert!(compile_and_match("\\d+", Flags::empty(), "123"));
        assert!(!compile_and_match("^\\d+$", Flags::empty(), "abc"));
    }

    #[test]
    fn test_word_boundary() {
        assert!(compile_and_match("\\bfoo\\b", Flags::empty(), "foo bar"));
        assert!(!compile_and_match("\\bfoo\\b", Flags::empty(), "foobar"));
    }

    #[test]
    fn test_unicode_literal() {
        assert!(compile_and_match("Шерлок", Flags::empty(), "Шерлок Холмс"));
    }

    #[test]
    fn test_bounded_repetition() {
        assert!(compile_and_match("a{3}", Flags::empty(), "aaa"));
        assert!(!compile_and_match("^a{3}$", Flags::empty(), "aa"));
        assert!(compile_and_match("a{2,4}", Flags::empty(), "aaa"));
    }
}
