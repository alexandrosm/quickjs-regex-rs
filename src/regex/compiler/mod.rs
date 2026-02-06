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
    fn test_many_capture_groups() {
        // Simulate lexer-veryl: many capture groups in alternation
        // Use unique prefixes to avoid partial matching
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
        // Group 0 should match
        assert_eq!(ctx.captures[0], Some(0));
        assert_eq!(ctx.captures[1], Some(5));
        // Group 43 (regex-syntax index 43) should match
        assert_eq!(ctx.captures[43 * 2], Some(0));
        assert_eq!(ctx.captures[43 * 2 + 1], Some(5));
    }

    #[test]
    fn test_full_lexer_veryl_from_file() {
        // Read the actual 88-line lexer-veryl pattern
        let pattern_file = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/parol-veryl.txt");
        if !pattern_file.exists() {
            eprintln!("Skipping: tests/parol-veryl.txt not found");
            return;
        }
        let content = std::fs::read_to_string(&pattern_file).unwrap();
        let lines: Vec<&str> = content.lines().collect();
        let pattern = lines.join("|");
        eprintln!("Full lexer-veryl: {} chars, {} alternatives", pattern.len(), lines.len());

        let re = crate::regex::Regex::with_flags_pure_rust(&pattern, Flags::empty())
            .expect("compile failed");

        // Use a LARGE haystack to simulate benchmark conditions (150KB)
        let base = "module test_mod {\n  let x = 42;\n  // comment\n  function foo() { return x + 1; }\n}\n";
        let mut haystack = String::new();
        while haystack.len() < 150_000 {
            haystack.push_str(base);
        }
        eprintln!("Haystack size: {} bytes", haystack.len());

        let mut count = 0;
        let mut pos = 0;
        while pos < haystack.len() {
            match re.captures_at_pure_rust(&haystack, pos) {
                Some(caps) => {
                    count += caps.count_matched();
                    if let Some(m) = caps.get(0) {
                        pos = if m.end > m.start { m.end } else { m.start + 1 };
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }
        eprintln!("Full lexer-veryl: matched {} capture groups over {}KB", count, haystack.len() / 1024);
        assert!(count > 0);
    }

    #[test]
    fn test_aws_keys_pattern() {
        // The aws-keys "full" pattern
        let pattern = r#"(('|")((?:ASIA|AKIA|AROA|AIDA)([A-Z0-7]{16}))('|").*?(\n^.*?){0,4}(('|")[a-zA-Z0-9+/]{40}('|"))+|('|")[a-zA-Z0-9+/]{40}('|").*?(\n^.*?){0,3}('|")((?:ASIA|AKIA|AROA|AIDA)([A-Z0-7]{16}))('|"))+"#;
        let re = crate::regex::Regex::with_flags_pure_rust(pattern, Flags::empty())
            .expect("compile failed");

        // Test with a simple haystack
        let haystack = r#""AKIAIOSFODNN7EXAMPLE" ... "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY""#;
        let caps = re.captures_at_pure_rust(haystack, 0);
        eprintln!("aws-keys result: {:?}", caps.as_ref().map(|c| c.len()));
    }

    #[test]
    fn test_real_lexer_veryl_repeated_captures() {
        // Test repeated captures like count-captures model does
        let lines = vec![
            r"(\r\n|\r|\n)", r"([\t\v\f ]+)",
            r"((?:(?:(?://.*(?:\r\n|\r|\n))|(?:/\*.*?\*/))\s*)+)",
            r"([0-9]+(?:_[0-9]+)*\.[0-9]+(?:_[0-9]+)*[eE][+-]?[0-9]+(?:_[0-9]+)*)",
            r"([0-9]+(?:_[0-9]+)*\.[0-9]+(?:_[0-9]+)*)",
            r"([0-9]+(?:_[0-9]+)*[eE][+-]?[0-9]+(?:_[0-9]+)*)",
            r"([0-9]+(?:_[0-9]+)*)", r"(32'[sS]?[bB][01xXzZ?][01xXzZ?_]*)",
            r"(\bmodule\b)", r"(\binterface\b)", r"(\bfunction\b)",
            r"(\bimport\b)", r"(\bexport\b)", r"(\blet\b)", r"(\bvar\b)",
            r"(\blocalparam\b)", r"(\balways_comb\b)", r"(\balways_ff\b)",
            r"(\bassign\b)", r"(\breturn\b)", r"(\bif\b)", r"(\belse\b)",
            r"(\bfor\b)", r"(\bin\b)", r"(\bcase\b)", r"(\bdefault\b)",
            r"(\binput\b)", r"(\boutput\b)", r"(\binout\b)", r"(\bref\b)",
            r"(\bconst\b)", r"(\benum\b)", r"(\bstruct\b)", r"(\bunion\b)",
            r"(\btype\b)", r"(\bparam\b)", r"(\binst\b)", r"(\bclocking\b)",
            r"(\bposedge\b)", r"(\bnegedge\b)", r"(\bbit\b)", r"(\blogic\b)",
            r"(\btri\b)", r"(\bu32\b)", r"(\bu64\b)", r"(\bi32\b)",
            r"(\bi64\b)", r"(\bf32\b)", r"(\bf64\b)", r"(\bstring\b)",
            r"(\bstep\b)", r"(\brepeat\b)", r"(\binitial\b)", r"(\bfinal\b)",
            r"(\bpackage\b)", r"(\bpub\b)", r"(\blocal\b)",
            r"(\b_\b)", r"(\$[a-zA-Z_][0-9a-zA-Z_$]*)",
            r"([a-zA-Z_][0-9a-zA-Z_]*)", r"(::)", r"(:)", r"(\.\.\.\=)",
            r"(\.\.\=)", r"(\.\.\.\#)", r"(\.\.\#)", r"(\.\.)",
            r"(\#)", r"(\()", r"(\))", r"(\[)", r"(\])", r"(\{)", r"(\})",
            r"(;)", r"(,)", r"(\+)", r"(-)", r"(\*\*)", r"(\*)",
            r"(/)", r"(%)", r"(&)", r"(\|)", r"(\^)", r"(~)",
            r"(=)", r"(<)", r"(>)", r"(!)", r"(@)", r"(\.)",
            r"(.)",
        ];
        let pattern = lines.join("|");
        eprintln!("Pattern length: {} chars, {} alternatives", pattern.len(), lines.len());

        let re = crate::regex::Regex::with_flags_pure_rust(&pattern, Flags::empty())
            .expect("compile failed");

        let haystack = "module test_mod {\n  let x = 42;\n  // comment\n}\n";
        let mut count = 0;
        let mut pos = 0;
        while pos < haystack.len() {
            match re.captures_at_pure_rust(haystack, pos) {
                Some(caps) => {
                    count += caps.count_matched();
                    if let Some(m) = caps.get(0) {
                        pos = if m.end > m.start { m.end } else { m.start + 1 };
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }
        eprintln!("Matched {} capture groups over haystack", count);
        assert!(count > 0);
    }

    #[test]
    fn test_real_lexer_veryl() {
        // The actual lexer-veryl pattern (88 alternations with capture groups)
        let lines = vec![
            r"(\r\n|\r|\n)",
            r"([\t\v\f ]+)",
            r"((?:(?:(?://.*(?:\r\n|\r|\n))|(?:/\*.*?\*/))\s*)+)",
            r"([0-9]+(?:_[0-9]+)*\.[0-9]+(?:_[0-9]+)*[eE][+-]?[0-9]+(?:_[0-9]+)*)",
            r"([0-9]+(?:_[0-9]+)*\.[0-9]+(?:_[0-9]+)*)",
            r"([0-9]+(?:_[0-9]+)*[eE][+-]?[0-9]+(?:_[0-9]+)*)",
            r"([0-9]+(?:_[0-9]+)*)",
            r"(32'[sS]?[bB][01xXzZ?][01xXzZ?_]*)",
            r"(\bmodule\b)",
            r"(\binterface\b)",
            r"(\bfunction\b)",
            r"(\bimport\b)",
            r"(\bexport\b)",
            r"(\blet\b)",
            r"(\bvar\b)",
            r"([a-zA-Z_][0-9a-zA-Z_]*)",
            r"(.)",
        ];
        let pattern = lines.join("|");
        let bytecode = compile_regex(&pattern, Flags::empty()).expect("compile failed");
        let text = "module test_mod";
        let mut ctx = ExecContext::new(&bytecode, text.as_bytes());
        let result = ctx.exec(0);
        assert!(matches!(result, ExecResult::Match), "Should match");
    }

    #[test]
    fn test_lexer_veryl_like_pattern() {
        // Simplified version of lexer-veryl: alternation with captures + complex sub-patterns
        let pattern = r"(\r\n|\r|\n)|([ \t]+)|(//[^\n]*)|([a-z]+)|([0-9]+)|(\+|-|\*|/)|(\(|\))|(\{|\})|(\[|\])|(;)|(,)|(\.)|(:)|(\|)";
        let bytecode = compile_regex(pattern, Flags::empty()).expect("compile failed");
        // Test matching various tokens
        for (text, expected_group) in [
            ("\n", 1),
            ("  ", 2),
            ("// comment", 3),
            ("hello", 4),
            ("123", 5),
            ("+", 6),
            ("(", 7),
        ] {
            let mut ctx = ExecContext::new(&bytecode, text.as_bytes());
            let result = ctx.exec(0);
            assert!(matches!(result, ExecResult::Match), "Failed to match: {:?}", text);
            assert!(ctx.captures[expected_group * 2].is_some(),
                "Group {} should match for {:?}", expected_group, text);
        }
    }

    #[test]
    fn test_non_capturing_group() {
        // Test (?:...) doesn't create captures
        assert!(compile_and_match("(?:abc)+", Flags::empty(), "abcabc"));
        assert!(compile_and_match("(?:a|b)c", Flags::empty(), "ac"));
        assert!(compile_and_match("(?:a|b)c", Flags::empty(), "bc"));
    }

    #[test]
    fn test_nested_groups_with_captures() {
        let bytecode = compile_regex("((?:a)(b))", Flags::empty()).expect("compile failed");
        let text = b"ab";
        let mut ctx = ExecContext::new(&bytecode, text);
        assert!(matches!(ctx.exec(0), ExecResult::Match));
        // Group 0 = whole match
        assert_eq!(ctx.captures[0], Some(0));
        assert_eq!(ctx.captures[1], Some(2));
    }

    #[test]
    fn test_lazy_quantifiers() {
        // a+? should match minimal
        let m = compile_and_find("a+?", Flags::empty(), "aaa");
        assert_eq!(m, Some((0, 1))); // Just one 'a'
    }

    #[test]
    fn test_whitespace_class() {
        assert!(compile_and_match("\\s+", Flags::empty(), "  \t"));
        assert!(!compile_and_match("^\\s+$", Flags::empty(), "abc"));
    }

    #[test]
    fn test_dot_star_lazy() {
        let m = compile_and_find("a.*?b", Flags::empty(), "aXXbYYb");
        assert_eq!(m, Some((0, 4))); // "aXXb" not "aXXbYYb"
    }

    #[test]
    fn test_bounded_repetition() {
        assert!(compile_and_match("a{3}", Flags::empty(), "aaa"));
        assert!(!compile_and_match("^a{3}$", Flags::empty(), "aa"));
        assert!(compile_and_match("a{2,4}", Flags::empty(), "aaa"));
    }
}
