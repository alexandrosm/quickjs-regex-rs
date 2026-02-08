use quickjs_regex::Regex;

#[test]
fn test_suffix_repeated() {
    let re = Regex::new("[a-z]+ing").unwrap();
    let unit = "running walking testing jumping ";
    for n in [1, 2, 5, 10, 50, 100] {
        let text = unit.repeat(n);
        let count = re.count_matches(&text);
        let iter_count = re.find_iter(&text).count();
        eprintln!("n={}: count_matches={} find_iter={} expected={}", n, count, iter_count, n * 4);
        assert_eq!(count, n * 4, "count_matches at n={}", n);
        assert_eq!(iter_count, n * 4, "find_iter at n={}", n);
    }
}
