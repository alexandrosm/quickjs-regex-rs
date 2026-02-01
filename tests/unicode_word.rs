use quickjs_regex::{Regex, Flags};

/// Test that \w+ matches ASCII word characters
#[test]
fn test_ascii_only() {
    let text = "Hello world";

    let re = Regex::with_flags(r"\w+", Flags::empty()).unwrap();

    let result = re.find(text);
    assert!(result.is_some());
    let m = result.unwrap();
    assert_eq!(&text[m.start..m.end], "Hello");
}

/// Test that \w+ does NOT match Cyrillic (per ECMAScript spec)
///
/// This is expected behavior: QuickJS follows ECMAScript semantics where
/// \w means [a-zA-Z0-9_] regardless of the unicode flag. The unicode flag
/// affects surrogate pair handling, not \w semantics.
///
/// Note: This differs from rust/regex where \w with unicode is Unicode-aware.
#[test]
fn test_cyrillic_not_matched() {
    let text = "Привет мир"; // "Hello world" in Russian

    // Create regex with UNICODE flag
    let mut flags = Flags::empty();
    flags.insert(Flags::UNICODE);
    let re = Regex::with_flags(r"\w+", flags).unwrap();

    // ECMAScript: \w is ASCII-only, so no match for Cyrillic
    let result = re.find(text);
    assert!(result.is_none(), "\\w should not match Cyrillic per ECMAScript spec");
}

/// Test mixed ASCII and Cyrillic text
#[test]
fn test_mixed_ascii_cyrillic() {
    let text = "Hello Мир World";

    let mut flags = Flags::empty();
    flags.insert(Flags::UNICODE);
    let re = Regex::with_flags(r"\w+", flags).unwrap();

    let mut words: Vec<String> = Vec::new();
    let mut pos = 0;
    while pos < text.len() {
        let slice = &text[pos..];
        match re.find(slice) {
            Some(m) if m.end > m.start => {
                let word = &slice[m.start..m.end];
                words.push(word.to_string());
                pos += m.end;
            }
            Some(m) => {
                // Empty match - advance by one UTF-8 character
                let char_len = slice.chars().next().map(|c| c.len_utf8()).unwrap_or(1);
                pos += m.start + char_len;
            }
            None => break,
        }
    }

    // Only ASCII words are matched (per ECMAScript spec)
    assert!(words.contains(&"Hello".to_string()));
    assert!(words.contains(&"World".to_string()));
    assert!(!words.contains(&"Мир".to_string()), "Cyrillic should not match");
}

/// Test that UTF-8 text doesn't cause panics when scanning
#[test]
fn test_utf8_no_panic() {
    let texts = [
        "Hello",
        "Привет",
        "你好世界",
        "🎉🎊",
        "Mixed Привет 你好 🎉",
        "",
    ];

    let re = Regex::new(r"\w+").unwrap();

    for text in &texts {
        // Should not panic
        let _ = re.find(text);
        let _ = re.find_iter(text).count();
    }
}

/// Test that \d matches ASCII digits only (per ECMAScript spec)
#[test]
fn test_digit_ascii_only() {
    // ASCII digits
    let ascii_text = "Test 123 numbers";
    let re = Regex::new(r"\d+").unwrap();
    
    let result = re.find(ascii_text);
    assert!(result.is_some());
    let m = result.unwrap();
    assert_eq!(&ascii_text[m.start..m.end], "123");
    
    // Unicode digits (should NOT match per ECMAScript)
    let arabic_digits = "Test ١٢٣ numbers"; // Arabic-Indic digits
    let result = re.find(arabic_digits);
    // \d should only match ASCII 0-9, not Unicode digits
    assert!(result.is_none() || &arabic_digits[result.as_ref().unwrap().start..result.as_ref().unwrap().end] != "١٢٣",
        r"\d should not match Arabic-Indic digits per ECMAScript spec");
}

/// Test date pattern with ASCII digits
#[test]
fn test_date_pattern() {
    let text = "Date: 2024-01-15, Time: 10:30:00";
    
    // Date pattern
    let re = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();
    let result = re.find(text);
    assert!(result.is_some());
    let m = result.unwrap();
    assert_eq!(&text[m.start..m.end], "2024-01-15");
    
    // Count all digit sequences
    let re_digits = Regex::new(r"\d+").unwrap();
    let count = re_digits.find_iter(text).count();
    assert_eq!(count, 6, "Should find 6 digit sequences: 2024, 01, 15, 10, 30, 00");
}
