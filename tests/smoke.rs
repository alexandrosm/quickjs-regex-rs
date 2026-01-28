// Smoke tests for quickjs-regex
// These tests verify that the library loads without static initialization crashes.

use quickjs_regex::Regex;

#[test]
fn smoke_no_crash_on_load() {
    // Verify the library loads without crashing and can compile patterns
    let re = Regex::new("test").expect("should compile simple pattern");
    assert!(re.is_match("this is a test"));
}
