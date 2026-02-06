//! Test262-style compliance tests for the QuickJS regex engine.
//!
//! These tests are derived from the ECMAScript Test262 test suite
//! (https://github.com/tc39/test262) and verify JavaScript regex semantics.

use quickjs_regex::{Regex, Flags};

/// Helper to test that a pattern matches input and returns expected captures
fn test_exec(pattern: &str, input: &str, expected: Option<(&str, usize, &[&str])>) {
    let re = Regex::new(pattern).expect(&format!("Failed to compile: {}", pattern));

    match (re.captures(input), expected) {
        (Some(caps), Some((full_match, index, groups))) => {
            let m = caps.get(0).expect("Should have group 0");
            assert_eq!(m.as_str(input), full_match,
                "Full match mismatch for /{}/", pattern);
            assert_eq!(m.start, index,
                "Index mismatch for /{}/", pattern);

            for (i, &expected_group) in groups.iter().enumerate() {
                let actual = caps.get_str(i);
                assert_eq!(actual, Some(expected_group),
                    "Group {} mismatch for /{}/", i, pattern);
            }
        }
        (None, None) => {
            // Expected no match, got no match - good
        }
        (Some(caps), None) => {
            panic!("Expected no match for /{}/ on {:?}, but got {:?}",
                pattern, input, caps.get_str(0));
        }
        (None, Some(_)) => {
            panic!("Expected match for /{}/ on {:?}, but got none", pattern, input);
        }
    }
}

/// Helper to test that a pattern matches (is_match)
fn test_match(pattern: &str, input: &str, should_match: bool) {
    let re = Regex::new(pattern).expect(&format!("Failed to compile: {}", pattern));
    assert_eq!(re.is_match(input), should_match,
        "is_match mismatch for /{}/ on {:?}", pattern, input);
}

/// Helper for case-insensitive tests
fn test_exec_flags(pattern: &str, flags: Flags, input: &str, expected: Option<(&str, usize)>) {
    let re = Regex::with_flags(pattern, flags)
        .expect(&format!("Failed to compile: /{}/{:?}", pattern, flags));

    match (re.find(input), expected) {
        (Some(m), Some((full_match, index))) => {
            assert_eq!(m.as_str(input), full_match,
                "Match mismatch for /{}/{:?}", pattern, flags);
            assert_eq!(m.start, index,
                "Index mismatch for /{}/{:?}", pattern, flags);
        }
        (None, None) => {}
        (Some(m), None) => {
            panic!("Expected no match for /{}/{:?}, got {:?}", pattern, flags, m.as_str(input));
        }
        (None, Some(_)) => {
            panic!("Expected match for /{}/{:?}, got none", pattern, flags);
        }
    }
}

// ============================================================================
// S15.10.6.2 - RegExp.prototype.exec
// ============================================================================

mod exec_basic {
    use super::*;

    #[test]
    fn alternation_t1() {
        // /1|12/.exec("123") => ["1"] at index 0
        test_exec("1|12", "123", Some(("1", 0, &["1"])));
    }

    #[test]
    fn alternation_t2() {
        // /2|12/.exec("1.012") => ["12"] at index 3
        // String "1.012": '1'=0, '.'=1, '0'=2, '1'=3, '2'=4
        // "12" matches at index 3-4
        test_exec("2|12", "1.012", Some(("12", 3, &["12"])));
    }

    #[test]
    fn character_class_t5() {
        // /t[a-b|q-s]/.exec("true") => ["tr"]
        test_exec("t[a-bq-s]", "true", Some(("tr", 0, &["tr"])));
    }

    #[test]
    fn quantifier_greedy() {
        // /a[a-z]{2,4}/.exec("abcdefghi") => ["abcde"]
        test_exec("a[a-z]{2,4}", "abcdefghi", Some(("abcde", 0, &["abcde"])));
    }

    #[test]
    fn quantifier_lazy() {
        // /a[a-z]{2,4}?/.exec("abcdefghi") => ["abc"]
        test_exec("a[a-z]{2,4}?", "abcdefghi", Some(("abc", 0, &["abc"])));
    }

    #[test]
    fn capture_groups() {
        // /(aa|aabaac|ba|b|c)*/.exec("aabaac") => ["aaba", "ba"]
        test_exec("(aa|aabaac|ba|b|c)*", "aabaac", Some(("aaba", 0, &["aaba", "ba"])));
    }

    #[test]
    fn nested_groups() {
        // /(z)((a+)?(b+)?(c))*/.exec("zaacbbbcac")
        // => ["zaacbbbcac", "z", "ac", "a", undefined, "c"]
        let re = Regex::new("(z)((a+)?(b+)?(c))*").unwrap();
        let caps = re.captures("zaacbbbcac").unwrap();
        assert_eq!(caps.get_str(0), Some("zaacbbbcac"));
        assert_eq!(caps.get_str(1), Some("z"));
        assert_eq!(caps.get_str(2), Some("ac"));
        assert_eq!(caps.get_str(3), Some("a"));
        // Group 4 (b+)? didn't participate in final iteration
        assert!(caps.get(4).is_none() || caps.get_str(4) == Some(""));
        assert_eq!(caps.get_str(5), Some("c"));
    }
}

// ============================================================================
// S15.10.6.3 - RegExp.prototype.test
// ============================================================================

mod test_basic {
    use super::*;

    #[test]
    fn test_returns_bool() {
        test_match("1|12", "123", true);
        test_match("xyz", "123", false);
    }

    #[test]
    fn test_empty_match() {
        test_match("", "anything", true);
    }
}

// ============================================================================
// Character Classes
// ============================================================================

mod character_classes {
    use super::*;

    #[test]
    fn digit_class() {
        test_match(r"\d", "a1b", true);
        test_match(r"\d", "abc", false);
        test_exec(r"\d+", "abc123def", Some(("123", 3, &["123"])));
    }

    #[test]
    fn non_digit_class() {
        test_match(r"\D", "123", false);
        test_match(r"\D", "a23", true);
    }

    #[test]
    fn word_class() {
        test_match(r"\w", "a", true);
        test_match(r"\w", "1", true);
        test_match(r"\w", "_", true);
        test_match(r"\w", " ", false);
        test_match(r"\w", "-", false);
    }

    #[test]
    fn non_word_class() {
        test_match(r"\W", " ", true);
        test_match(r"\W", "-", true);
        test_match(r"\W", "a", false);
    }

    #[test]
    fn whitespace_class() {
        test_match(r"\s", " ", true);
        test_match(r"\s", "\t", true);
        test_match(r"\s", "\n", true);
        test_match(r"\s", "a", false);
    }

    #[test]
    fn non_whitespace_class() {
        test_match(r"\S", "a", true);
        test_match(r"\S", " ", false);
    }

    #[test]
    fn ranges() {
        test_match("[a-z]", "m", true);
        test_match("[a-z]", "M", false);
        test_match("[A-Z]", "M", true);
        test_match("[0-9]", "5", true);
        test_match("[a-zA-Z0-9]", "x", true);
        test_match("[a-zA-Z0-9]", "X", true);
        test_match("[a-zA-Z0-9]", "5", true);
        test_match("[a-zA-Z0-9]", "-", false);
    }

    #[test]
    fn negated_class() {
        test_match("[^abc]", "d", true);
        test_match("[^abc]", "a", false);
        test_match("[^0-9]", "x", true);
        test_match("[^0-9]", "5", false);
    }
}

// ============================================================================
// Anchors
// ============================================================================

mod anchors {
    use super::*;

    #[test]
    fn start_anchor() {
        test_match("^hello", "hello world", true);
        test_match("^hello", "say hello", false);
    }

    #[test]
    fn end_anchor() {
        test_match("world$", "hello world", true);
        test_match("world$", "world hello", false);
    }

    #[test]
    fn both_anchors() {
        test_match("^exact$", "exact", true);
        test_match("^exact$", "not exact", false);
        test_match("^exact$", "exact not", false);
    }

    #[test]
    fn word_boundary() {
        test_match(r"\bword\b", "a word here", true);
        test_match(r"\bword\b", "awordhere", false);
        test_match(r"\bword\b", "wording", false);
        test_match(r"\bword\b", "sword", false);
    }

    #[test]
    fn non_word_boundary() {
        test_match(r"\Bword", "sword", true);
        test_match(r"\Bword", "word", false);
    }
}

// ============================================================================
// Quantifiers
// ============================================================================

mod quantifiers {
    use super::*;

    #[test]
    fn zero_or_more() {
        test_exec("a*", "aaa", Some(("aaa", 0, &["aaa"])));
        test_exec("a*", "bbb", Some(("", 0, &[""])));
    }

    #[test]
    fn one_or_more() {
        test_exec("a+", "aaa", Some(("aaa", 0, &["aaa"])));
        test_match("a+", "bbb", false);
    }

    #[test]
    fn optional() {
        test_exec("colou?r", "color", Some(("color", 0, &["color"])));
        test_exec("colou?r", "colour", Some(("colour", 0, &["colour"])));
    }

    #[test]
    fn exact_count() {
        test_exec("a{3}", "aaaaa", Some(("aaa", 0, &["aaa"])));
        test_match("a{3}", "aa", false);
    }

    #[test]
    fn range_count() {
        test_exec("a{2,4}", "aaaaa", Some(("aaaa", 0, &["aaaa"])));
        test_exec("a{2,4}", "aa", Some(("aa", 0, &["aa"])));
        test_match("a{2,4}", "a", false);
    }

    #[test]
    fn lazy_quantifiers() {
        test_exec("a+?", "aaa", Some(("a", 0, &["a"])));
        test_exec("a*?", "aaa", Some(("", 0, &[""])));
        test_exec("a{2,4}?", "aaaaa", Some(("aa", 0, &["aa"])));
    }
}

// ============================================================================
// Groups and Backreferences
// ============================================================================

mod groups {
    use super::*;

    #[test]
    fn capturing_groups() {
        let re = Regex::new("(\\w+)@(\\w+)\\.(\\w+)").unwrap();
        let caps = re.captures("user@example.com").unwrap();
        assert_eq!(caps.get_str(0), Some("user@example.com"));
        assert_eq!(caps.get_str(1), Some("user"));
        assert_eq!(caps.get_str(2), Some("example"));
        assert_eq!(caps.get_str(3), Some("com"));
    }

    #[test]
    fn non_capturing_groups() {
        let re = Regex::new("(?:ab)+").unwrap();
        assert_eq!(re.capture_count(), 1); // Only group 0
        let caps = re.captures("ababab").unwrap();
        assert_eq!(caps.get_str(0), Some("ababab"));
    }

    #[test]
    fn backreference() {
        test_match(r"(\w+)\s+\1", "hello hello", true);
        test_match(r"(\w+)\s+\1", "hello world", false);

        let re = Regex::new(r"<(\w+)>.*</\1>").unwrap();
        assert!(re.is_match("<div>content</div>"));
        assert!(!re.is_match("<div>content</span>"));
    }

    #[test]
    fn nested_groups() {
        let re = Regex::new("((a)(b))").unwrap();
        let caps = re.captures("ab").unwrap();
        assert_eq!(caps.get_str(0), Some("ab"));
        assert_eq!(caps.get_str(1), Some("ab"));
        assert_eq!(caps.get_str(2), Some("a"));
        assert_eq!(caps.get_str(3), Some("b"));
    }
}

// ============================================================================
// Alternation
// ============================================================================

mod alternation {
    use super::*;

    #[test]
    fn simple_alternation() {
        test_match("cat|dog", "I have a cat", true);
        test_match("cat|dog", "I have a dog", true);
        test_match("cat|dog", "I have a bird", false);
    }

    #[test]
    fn alternation_order() {
        // First alternative wins
        test_exec("a|ab", "ab", Some(("a", 0, &["a"])));
        test_exec("ab|a", "ab", Some(("ab", 0, &["ab"])));
    }

    #[test]
    fn alternation_with_groups() {
        let re = Regex::new("(cat|dog) food").unwrap();
        let caps = re.captures("cat food").unwrap();
        assert_eq!(caps.get_str(1), Some("cat"));
    }
}

// ============================================================================
// Lookahead and Lookbehind
// ============================================================================

mod lookaround {
    use super::*;

    #[test]
    fn positive_lookahead() {
        test_exec(r"foo(?=bar)", "foobar", Some(("foo", 0, &["foo"])));
        test_match(r"foo(?=bar)", "foobaz", false);
    }

    #[test]
    fn negative_lookahead() {
        test_exec(r"foo(?!bar)", "foobaz", Some(("foo", 0, &["foo"])));
        test_match(r"foo(?!bar)", "foobar", false);
    }

    #[test]
    fn positive_lookbehind() {
        test_exec(r"(?<=foo)bar", "foobar", Some(("bar", 3, &["bar"])));
        test_match(r"(?<=foo)bar", "bazbar", false);
    }

    #[test]
    fn negative_lookbehind() {
        test_exec(r"(?<!foo)bar", "bazbar", Some(("bar", 3, &["bar"])));
        test_match(r"(?<!foo)bar", "foobar", false);
    }
}

// ============================================================================
// Flags
// ============================================================================

mod flags {
    use super::*;

    #[test]
    fn case_insensitive() {
        let flags = Flags::from_bits(Flags::IGNORE_CASE);
        test_exec_flags("hello", flags, "HELLO", Some(("HELLO", 0)));
        test_exec_flags("hello", flags, "HeLLo", Some(("HeLLo", 0)));
    }

    #[test]
    fn multiline() {
        let flags = Flags::from_bits(Flags::MULTILINE);
        let re = Regex::with_flags("^line", flags).unwrap();
        assert!(re.is_match("first\nline two"));
    }

    #[test]
    fn dotall() {
        // Without 's' flag, dot doesn't match newline
        let re_no_s = Regex::new("a.b").unwrap();
        assert!(!re_no_s.is_match("a\nb"));

        // With 's' flag, dot matches newline
        let flags = Flags::from_bits(Flags::DOT_ALL);
        let re_s = Regex::with_flags("a.b", flags).unwrap();
        assert!(re_s.is_match("a\nb"));
    }
}

// ============================================================================
// Edge Cases
// ============================================================================

mod edge_cases {
    use super::*;

    #[test]
    fn empty_pattern() {
        let re = Regex::new("").unwrap();
        assert!(re.is_match("anything"));
        let m = re.find("test").unwrap();
        assert_eq!(m.start, 0);
        assert_eq!(m.end, 0);
    }

    #[test]
    fn empty_input() {
        test_match("^$", "", true);
        test_match(".", "", false);
    }

    #[test]
    fn special_chars_escaped() {
        test_match(r"\.", ".", true);
        test_match(r"\.", "a", false);
        test_match(r"\*", "*", true);
        test_match(r"\+", "+", true);
        test_match(r"\?", "?", true);
        test_match(r"\[", "[", true);
        test_match(r"\]", "]", true);
        test_match(r"\(", "(", true);
        test_match(r"\)", ")", true);
        test_match(r"\{", "{", true);
        test_match(r"\}", "}", true);
        test_match(r"\\", "\\", true);
        test_match(r"\^", "^", true);
        test_match(r"\$", "$", true);
        test_match(r"\|", "|", true);
    }

    #[test]
    fn unicode_basic() {
        test_match(".", "\u{1F600}", true); // Emoji
        test_match("...", "\u{1F600}\u{1F601}\u{1F602}", true);
    }

    #[test]
    fn long_input() {
        let long_input = "a".repeat(10000);
        let re = Regex::new("a+").unwrap();
        let m = re.find(&long_input).unwrap();
        assert_eq!(m.end - m.start, 10000);
    }

    #[test]
    fn catastrophic_backtracking_protection() {
        // This pattern would cause exponential backtracking without a limit.
        // The interpreter has a backtrack step limit that prevents hanging.
        let re = Regex::new("(a+)+b").unwrap();
        let input = "a".repeat(25) + "c";
        assert!(!re.is_match(&input));
    }
}

// ============================================================================
// find_iter tests
// ============================================================================

mod find_iter {
    use super::*;

    #[test]
    fn find_all_matches() {
        let re = Regex::new(r"\d+").unwrap();
        let matches: Vec<_> = re.find_iter("a1b23c456").collect();
        assert_eq!(matches.len(), 3);
        assert_eq!(matches[0].as_str("a1b23c456"), "1");
        assert_eq!(matches[1].as_str("a1b23c456"), "23");
        assert_eq!(matches[2].as_str("a1b23c456"), "456");
    }

    #[test]
    fn overlapping_potential() {
        let re = Regex::new("aa").unwrap();
        let matches: Vec<_> = re.find_iter("aaaa").collect();
        // Non-overlapping: "aa" at 0, "aa" at 2
        assert_eq!(matches.len(), 2);
    }
}
