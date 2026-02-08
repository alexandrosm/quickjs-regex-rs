use quickjs_regex::Regex;

#[test]
fn test_count_vs_find_iter() {
    let re = Regex::new(r"\b\w+\b").unwrap();
    let text = "hello world test";

    let count = re.count_matches(text);
    eprintln!("count_matches: {}", count);

    let iter_count = re.find_iter(text).count();
    eprintln!("find_iter: {}", iter_count);

    // find_at loop
    let mut pos = 0;
    let mut find_at_count = 0;
    while pos < text.len() {
        match re.find_at(text, pos) {
            Some(m) => {
                find_at_count += 1;
                eprintln!("find_at: ({}, {}) = {:?}", m.start, m.end, &text[m.start..m.end]);
                pos = if m.end > m.start { m.end } else { m.start + 1 };
            }
            None => break,
        }
    }
    eprintln!("find_at loop: {}", find_at_count);

    assert_eq!(count, 3);
    assert_eq!(iter_count, 3);
    assert_eq!(find_at_count, 3);
}
