// Smoke tests for quickjs-regex
// These tests verify that the library loads without static initialization crashes.

use quickjs_regex::Regex;

#[test]
fn smoke_no_crash_on_load() {
    // Verify the library loads without crashing and can compile patterns
    let re = Regex::new("test").expect("should compile simple pattern");
    assert!(re.is_match("this is a test"));
}

#[test]
fn test_inline_flags_case_insensitive() {
    // Test (?i) inline flag for case-insensitive matching
    let re = Regex::new("(?i)hello").expect("should compile with inline flag");
    assert!(re.is_match("HELLO"), "should match uppercase");
    assert!(re.is_match("hello"), "should match lowercase");
    assert!(re.is_match("HeLLo"), "should match mixed case");
}

#[test]
fn test_inline_flags_multiple() {
    // Test multiple inline flags (?im)
    let re = Regex::new("(?im)^hello").expect("should compile with multiple inline flags");
    assert!(re.is_match("world\nHELLO"), "should match with multiline + case insensitive");
}

#[test]
fn test_inline_flags_at_start() {
    // Inline flags at start should be extracted, not cause syntax error
    let re = Regex::new("(?i)test(?:foo|bar)").expect("should compile complex pattern with inline flag");
    assert!(re.is_match("TESTFOO"));
    assert!(re.is_match("testbar"));
}
