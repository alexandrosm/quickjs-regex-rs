use quickjs_regex::Regex;

#[test]
fn test_bounded_repeat_basic() {
    let re = Regex::new(r"[A-Za-z]{3,5}").unwrap();
    assert_eq!(re.count_matches("abcde"), 1, "5-char word");
    assert_eq!(re.count_matches("ab"), 0, "2-char word too short");
    assert_eq!(re.count_matches("abc"), 1, "3-char word");
    assert_eq!(re.count_matches("abcdefgh"), 2, "8-char: 'abcde' + 'fgh'");
}

#[test]
fn test_bounded_repeat_zero_min() {
    let re = Regex::new(r"a[\s\S]{0,3}b").unwrap();
    assert_eq!(re.count_matches("ab"), 1, "0 chars between");
    assert_eq!(re.count_matches("axb"), 1, "1 char between");
    assert_eq!(re.count_matches("axxxb"), 1, "3 chars between");
    assert_eq!(re.count_matches("axxxxb"), 0, "4 chars between — too many");
}

#[test]
fn test_find_at_vs_find_iter_consistency() {
    // The key test: do find_at loop and find_iter give the same results?
    for pat in &[r"[a-z]+ing", r"[A-Za-z]{3,5}", r"\b\w+\b"] {
        let re = Regex::new(pat).unwrap();
        let text = "testing running jumping abcde xyz something";

        let mut fa_count = 0;
        let mut fa_sum = 0;
        let mut pos = 0;
        while pos < text.len() {
            match re.find_at(text, pos) {
                Some(m) => {
                    fa_count += 1;
                    fa_sum += m.end - m.start;
                    pos = if m.end > m.start { m.end } else { m.start + 1 };
                }
                None => break,
            }
        }

        let fi_count = re.find_iter(text).count();
        let fi_sum: usize = re.find_iter(text).map(|m| m.end - m.start).sum();
        let cm_count = re.count_matches(text);

        eprintln!("pat={}: find_at={}/{} find_iter={}/{} count_matches={}",
            pat, fa_count, fa_sum, fi_count, fi_sum, cm_count);

        assert_eq!(fa_count, fi_count, "find_at vs find_iter count for {}", pat);
        assert_eq!(fa_sum, fi_sum, "find_at vs find_iter spans for {}", pat);
        assert_eq!(fa_count, cm_count, "find_at vs count_matches for {}", pat);
    }
}
