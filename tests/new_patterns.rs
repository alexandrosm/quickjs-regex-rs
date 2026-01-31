use quickjs_regex::Regex;

#[test]
fn test_capital_word_pattern() {
    let re = Regex::new("[A-Z][a-z]+").unwrap();
    let text = "Hello World Test";
    let matches: Vec<_> = re.find_iter(text).collect();
    assert_eq!(matches.len(), 3);
    assert_eq!(&text[matches[0].start..matches[0].end], "Hello");
    assert_eq!(&text[matches[1].start..matches[1].end], "World");
    assert_eq!(&text[matches[2].start..matches[2].end], "Test");
}

#[test]
fn test_lower_suffix_pattern() {
    let re = Regex::new("[a-z]+ing").unwrap();
    let text = "walking and talking and running";
    let matches: Vec<_> = re.find_iter(text).collect();
    assert_eq!(matches.len(), 3);
    assert_eq!(&text[matches[0].start..matches[0].end], "walking");
    assert_eq!(&text[matches[1].start..matches[1].end], "talking");
    assert_eq!(&text[matches[2].start..matches[2].end], "running");
}

#[test]
fn test_lower_suffix_singing() {
    // Critical test: "singing" should match as "singing", not "sing"
    // The pattern [a-z]+ing is greedy and should match the longest lowercase run ending in "ing"
    let re = Regex::new("[a-z]+ing").unwrap();
    let text = "singing loudly";
    let matches: Vec<_> = re.find_iter(text).collect();
    assert_eq!(matches.len(), 1, "Expected 1 match, got {}", matches.len());
    assert_eq!(&text[matches[0].start..matches[0].end], "singing");
}

#[test]
fn test_lower_suffix_no_match() {
    // "King" should not match (starts with uppercase)
    let re = Regex::new("[a-z]+ing").unwrap();
    let text = "King of the hill";
    let matches: Vec<_> = re.find_iter(text).collect();
    assert_eq!(matches.len(), 0, "Expected 0 matches, got {}", matches.len());
}
