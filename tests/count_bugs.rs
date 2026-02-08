use quickjs_regex::{Regex, Flags};

#[test]
fn test_digit_count() {
    let re = Regex::new("[0-9]+").unwrap();
    // Simple test
    assert_eq!(re.count_matches("abc123def456"), 2, "two digit runs");
    assert_eq!(re.count_matches("123"), 1, "single digit run");
    assert_eq!(re.count_matches("1 2 3"), 3, "three single digits");
}

#[test]
fn test_suffix_ing_count() {
    let re = Regex::new("[a-z]+ing").unwrap();
    assert_eq!(re.count_matches("running walking"), 2);
    assert_eq!(re.count_matches("ring"), 1);
    assert_eq!(re.count_matches("ING"), 0); // case sensitive
}

#[test]
fn test_bounded_repeat_count() {
    let re = Regex::new("[A-Za-z]{8,13}").unwrap();
    assert_eq!(re.count_matches("abcdefghij"), 1, "10-char word");
    assert_eq!(re.count_matches("abcdefg"), 0, "7-char word too short");
    assert_eq!(re.count_matches("abcdefghijklmnop"), 1, "16-char word, one match");
}

#[test]
fn test_capitals_count() {
    // (?:[A-Z][a-z]+\s*){10,100}
    let re = Regex::new("(?:[A-Z][a-z]+\\s*){10,100}").unwrap();
    assert_eq!(re.count_matches("Hello"), 0, "single capital word, need 10+");
    let text = "Hello World This Is A Test Of Ten Capital Words Here";
    let count = re.count_matches(text);
    eprintln!("capitals count on test text: {}", count);
    // Should match the entire sequence of 10+ capitalized words
}

#[test]
fn test_unicode_count() {
    // sherlock-casei-ru: case-insensitive Russian literal
    let re = Regex::with_flags("Шерлок", Flags::from_bits(Flags::IGNORE_CASE)).unwrap();
    assert_eq!(re.count_matches("Шерлок шерлок ШЕРЛОК"), 3, "case-insensitive Russian");
}
