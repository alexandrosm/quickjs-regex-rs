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
