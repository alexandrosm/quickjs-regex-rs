use quickjs_regex::{Regex, Flags};

#[test]
fn test_digit_count() {
    let re = Regex::new("[0-9]+").unwrap();
    assert_eq!(re.count_matches("abc123def456"), 2, "two digit runs");
    assert_eq!(re.count_matches("123"), 1, "single digit run");
    assert_eq!(re.count_matches("1 2 3"), 3, "three single digits");
}

#[test]
fn test_suffix_ing_count() {
    let re = Regex::new("[a-z]+ing").unwrap();
    assert_eq!(re.count_matches("running walking"), 2);
    assert_eq!(re.count_matches("ring"), 1);
    assert_eq!(re.count_matches("ING"), 0);
}

#[test]
fn test_bounded_repeat_count() {
    let re = Regex::new("[A-Za-z]{8,13}").unwrap();
    assert_eq!(re.count_matches("abcdefghij"), 1, "10-char word");
    assert_eq!(re.count_matches("abcdefg"), 0, "7-char word too short");
}

#[test]
fn test_capitals_count() {
    let re = Regex::new("(?:[A-Z][a-z]+\\s*){10,100}").unwrap();
    assert_eq!(re.count_matches("Hello"), 0, "need 10+");
}

#[test]
fn test_quadratic_count() {
    let re = Regex::new(".*[^A-Z]|[A-Z]").unwrap();
    let text = "A".repeat(100);
    let count = re.count_matches(&text);
    eprintln!("quadratic count: {} (expected 100)", count);
    assert_eq!(count, 100);
}

#[test]
fn test_unicode_count() {
    // This pattern has CHAR32 (non-ASCII) so Wide NFA won't compile.
    // Falls through to PikeScanner which should handle it correctly.
    // Case-sensitive
    let re_cs = Regex::new("Шерлок").unwrap();
    assert_eq!(re_cs.count_matches("Шерлок шерлок"), 1, "case-sensitive finds 1");

    // Case-insensitive: test find_at directly
    let re = Regex::with_flags("Шерлок", Flags::from_bits(Flags::IGNORE_CASE)).unwrap();
    let m1 = re.find_at("Шерлок", 0);
    eprintln!("find_at case-insensitive exact: {:?}", m1);
    let m2 = re.find_at("шерлок", 0);
    eprintln!("find_at case-insensitive lower: {:?}", m2);

    // Test with find_iter for correct counting
    let count_iter = re.find_iter("Шерлок шерлок ШЕРЛОК").count();
    eprintln!("find_iter case-insensitive: {}", count_iter);

    eprintln!("strategy: {}, is_match: {}", re.strategy_name(), re.is_match("Шерлок"));
    let bc = re.debug_bytecode();
    eprintln!("bytecode ({} bytes): first 30 opcodes:", bc.len());
    let mut pc = 8;
    for _ in 0..30 {
        if pc >= bc.len() { break; }
        eprintln!("  pc={}: opcode={}", pc, bc[pc]);
        pc += 1; // simplified, not correct sizing
    }
    let count = re.count_matches("Шерлок шерлок ШЕРЛОК");
    eprintln!("count_matches case-insensitive: {} (expected 3)", count);
    assert_eq!(count, 3);
}
