//! Pure Rust regex compiler with full JavaScript regex syntax support.
//!
//! Custom parser handles backreferences, lookahead, lookbehind, and all
//! ECMAScript regex features. Compiles to QuickJS bytecode.

mod bytecode_builder;
mod codegen;
pub mod parser;

use crate::regex::Flags;
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

/// Compile a regex pattern to bytecode (pure Rust, full JS syntax)
pub fn compile_regex(pattern: &str, flags: Flags) -> Result<Vec<u8>> {
    // Detect inline flags (?i), (?m), (?s) anywhere in pattern and promote to global flags.
    // This handles PCRE-style patterns like "(?i)foo|(?i)bar" where each branch uses (?i).
    let mut final_flags = flags;
    if pattern.contains("(?i") { final_flags.insert(Flags::IGNORE_CASE); }
    if pattern.contains("(?m") { final_flags.insert(Flags::MULTILINE); }
    if pattern.contains("(?s") { final_flags.insert(Flags::DOT_ALL); }

    let ast = parser::parse(pattern, final_flags)?;
    let capture_count = parser::count_captures(pattern, final_flags)?;
    let mut codegen = CodeGenerator::new(final_flags, capture_count);
    codegen.compile(&ast)?;
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

    // === Basic features ===

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
        assert_eq!(ctx.captures[0], Some(0));
        assert_eq!(ctx.captures[1], Some(2));
        assert_eq!(ctx.captures[2], Some(0));
        assert_eq!(ctx.captures[3], Some(1));
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

    #[test]
    fn test_non_capturing_group() {
        assert!(compile_and_match("(?:abc)+", Flags::empty(), "abcabc"));
        assert!(compile_and_match("(?:a|b)c", Flags::empty(), "ac"));
        assert!(compile_and_match("(?:a|b)c", Flags::empty(), "bc"));
    }

    #[test]
    fn test_lazy_quantifiers() {
        let m = compile_and_find("a+?", Flags::empty(), "aaa");
        assert_eq!(m, Some((0, 1)));
    }

    #[test]
    fn test_whitespace_class() {
        assert!(compile_and_match("\\s+", Flags::empty(), "  \t"));
        assert!(!compile_and_match("^\\s+$", Flags::empty(), "abc"));
    }

    #[test]
    fn test_dot_star_lazy() {
        let m = compile_and_find("a.*?b", Flags::empty(), "aXXbYYb");
        assert_eq!(m, Some((0, 4)));
    }

    // === JS-specific features ===

    #[test]
    fn test_backreference() {
        assert!(compile_and_match("(a)\\1", Flags::empty(), "aa"));
        assert!(!compile_and_match("^(a)\\1$", Flags::empty(), "ab"));
    }

    #[test]
    fn test_backreference_word() {
        assert!(compile_and_match("(\\w+)\\s+\\1", Flags::empty(), "hello hello"));
        assert!(!compile_and_match("^(\\w+)\\s+\\1$", Flags::empty(), "hello world"));
    }

    #[test]
    fn test_positive_lookahead() {
        assert!(compile_and_match("foo(?=bar)", Flags::empty(), "foobar"));
        assert!(!compile_and_match("foo(?=bar)", Flags::empty(), "foobaz"));
    }

    #[test]
    fn test_negative_lookahead() {
        assert!(compile_and_match("foo(?!bar)", Flags::empty(), "foobaz"));
        assert!(!compile_and_match("foo(?!bar)", Flags::empty(), "foobar"));
    }

    // === Complex patterns ===

    #[test]
    fn test_many_capture_groups() {
        let mut parts = Vec::new();
        for i in 0..90 {
            parts.push(format!("(x{:03}y)", i));
        }
        let pattern = parts.join("|");
        let bytecode = compile_regex(&pattern, Flags::empty()).expect("compile failed");
        let text = "x042y";
        let mut ctx = ExecContext::new(&bytecode, text.as_bytes());
        let result = ctx.exec(0);
        assert!(matches!(result, ExecResult::Match));
        assert_eq!(ctx.captures[0], Some(0));
        assert_eq!(ctx.captures[1], Some(5));
        assert_eq!(ctx.captures[43 * 2], Some(0));
        assert_eq!(ctx.captures[43 * 2 + 1], Some(5));
    }

    #[test]
    fn test_nested_groups_with_captures() {
        let bytecode = compile_regex("((?:a)(b))", Flags::empty()).expect("compile failed");
        let text = b"ab";
        let mut ctx = ExecContext::new(&bytecode, text);
        assert!(matches!(ctx.exec(0), ExecResult::Match));
        assert_eq!(ctx.captures[0], Some(0));
        assert_eq!(ctx.captures[1], Some(2));
    }

    #[test]
    fn test_lexer_veryl_like_pattern() {
        let pattern = r"(\r\n|\r|\n)|([ \t]+)|(//[^\n]*)|([a-z]+)|([0-9]+)|(\+|-|\*|/)|(\(|\))|(\{|\})|(\[|\])|(;)|(,)|(\.)|(:)|(\|)";
        let bytecode = compile_regex(pattern, Flags::empty()).expect("compile failed");
        for (text, expected_group) in [
            ("\n", 1), ("  ", 2), ("// comment", 3), ("hello", 4),
            ("123", 5), ("+", 6), ("(", 7),
        ] {
            let mut ctx = ExecContext::new(&bytecode, text.as_bytes());
            let result = ctx.exec(0);
            assert!(matches!(result, ExecResult::Match), "Failed to match: {:?}", text);
            assert!(ctx.captures[expected_group * 2].is_some(),
                "Group {} should match for {:?}", expected_group, text);
        }
    }

    #[test]
    fn test_aws_keys_pattern() {
        let pattern = r"((?:ASIA|AKIA|AROA|AIDA)([A-Z0-7]{16}))";
        let _ = compile_regex(pattern, Flags::empty()).expect("compile failed");
    }

    #[test]
    fn test_aws_keys_full_pattern() {
        // Debug: test sub-patterns
        let h = r#""AIDAABCDEFGHIJKLMNOP""aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa""#;
        assert!(compile_and_match(r#"('|")AIDA"#, Flags::empty(), h), "quote+AIDA");
        assert!(compile_and_match(r"[A-Z0-7]{16}", Flags::empty(), "ABCDEFGHIJKLMNOP"), "16 chars");
        assert!(compile_and_match(r#"[a-zA-Z0-9+/]{40}"#, Flags::empty(),
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"), "40 chars");
        // First branch simplified
        assert!(compile_and_match(
            r#"('|")((?:AIDA)([A-Z0-7]{16}))('|").*?(('|")[a-zA-Z0-9+/]{40}('|"))+"#,
            Flags::empty(), h), "first branch simplified");
        // Full pattern
        let pattern = r#"(('|")((?:ASIA|AKIA|AROA|AIDA)([A-Z0-7]{16}))('|").*?(\n^.*?){0,4}(('|")[a-zA-Z0-9+/]{40}('|"))+|('|")[a-zA-Z0-9+/]{40}('|").*?(\n^.*?){0,3}('|")((?:ASIA|AKIA|AROA|AIDA)([A-Z0-7]{16}))('|"))+"#;
        assert!(compile_and_match(pattern, Flags::empty(), h),
            "aws-keys full should match test haystack");
    }

    #[test]
    fn test_bounded_repeat_context() {
        // Progressively build up to full pattern
        assert!(compile_and_match(r"a[\s\S]{0,5}b", Flags::empty(), "a12345b"), "a..b");
        assert!(compile_and_match(r"a[\s\S]{0,10}b", Flags::empty(), "a1234567890b"), "a..10..b");
        assert!(compile_and_match(r"a[\s\S]{0,20}Result", Flags::empty(), "a blah blah Result"), "a..Result");
        // Full pattern
        let pattern = r"[A-Za-z]{10}\s+[\s\S]{0,100}Result[\s\S]{0,100}\s+[A-Za-z]{10}";
        let haystack = "abcdefghij blah blah blah Result blib blab klmnopqrst";
        assert!(compile_and_match(pattern, Flags::empty(), haystack),
            "context pattern should match test haystack");
    }

    #[test]
    fn test_noseyparker_inline_flags() {
        let pattern = r"(?i)\b(p8e-[a-z0-9-]{32})(?:[^a-z0-9-]|$)";
        let _ = compile_regex(pattern, Flags::empty()).expect("noseyparker compile failed");
    }

    #[test]
    fn test_unstructured_to_json() {
        let pattern = r"^([^ ]+ [^ ]+) ([DIWEF])[1234]: ((?:(?:\[[^\]]*?\]|\([^\)]*?\)): )*)(.*?) \{([^\}]*)\}$";
        let bytecode = compile_regex(pattern, Flags::empty()).expect("unstructured compile failed");
        let text = r#"2023-01-15 12:34:56 I1: [module]: hello {key=value}"#;
        let mut ctx = ExecContext::new(&bytecode, text.as_bytes());
        let result = ctx.exec(0);
        assert!(matches!(result, ExecResult::Match), "Should match log line");
    }

    #[test]
    fn test_char_class_s_S() {
        // [\s\S] should match ANY character (like dotall)
        assert!(compile_and_match("[\\s\\S]", Flags::empty(), "a"));
        assert!(compile_and_match("[\\s\\S]", Flags::empty(), " "));
        assert!(compile_and_match("[\\s\\S]", Flags::empty(), "\n"));
    }

    #[test]
    fn test_negated_class_newline() {
        // [^\n] should match anything except newline
        assert!(compile_and_match("[^\\n]", Flags::empty(), "a"));
        assert!(!compile_and_match("^[^\\n]$", Flags::empty(), "\n"));
    }

    #[test]
    fn test_large_bounded_repeat() {
        // {12,} must match 12+ characters
        assert!(compile_and_match(r"\b[A-Za-z0-9_]{12,}\b", Flags::empty(), "abcdefghijklmnop"));
        assert!(!compile_and_match(r"^\b[A-Za-z0-9_]{12,}\b$", Flags::empty(), "short"));
        // {8,13} bounded
        assert!(compile_and_match("[A-Za-z]{8,13}", Flags::empty(), "abcdefghij"));
        assert!(!compile_and_match("^[A-Za-z]{8,13}$", Flags::empty(), "short"));
    }

    #[test]
    fn test_inline_flag_group() {
        // (?i) should enable case-insensitive for the rest
        assert!(compile_and_match("(?i)hello", Flags::empty(), "HELLO"));
        assert!(compile_and_match("(?i:hello)", Flags::empty(), "HELLO"));
    }

    #[test]
    fn test_newline_in_class_with_caret() {
        // (\n^.*?) - newline followed by start-of-line
        let flags = Flags::from_bits(Flags::MULTILINE);
        assert!(compile_and_match("\\n^test", flags, "line1\ntest"));
    }
}
