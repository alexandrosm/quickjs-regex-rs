use quickjs_regex::Regex;

#[test]
fn test_noseyparker_prefilter() {
    // Real noseyparker-like patterns (with \b prefix)
    let pats = vec![
        r"\bage1[0-9a-z]{58}\b",
        r"\bAGE-SECRET-KEY-1[0-9A-Z]{58}\b",
        r"\bp8e-[a-z0-9-]{32}\b",
        r"\bAKIA[A-Z0-7]{16}\b",
        r"\bsk_live_[a-zA-Z0-9]{24}\b",
        r"\bghp_[a-zA-Z0-9]{36}\b",
        r"\bglpat-[a-zA-Z0-9\-]{20}\b",
        r"\bnpm_[a-zA-Z0-9]{36}\b",
    ];
    let combined = pats.iter().enumerate()
        .map(|(_, p)| format!("({})", p))
        .collect::<Vec<_>>().join("|");

    let re = Regex::new(&combined).unwrap();
    eprintln!("strategy: {}", re.strategy_name());

    // Check if prefilter works by testing count
    let text = "nothing age1aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa done";
    let c = re.count_matches(text);
    eprintln!("count on small text: {}", c);

    // Benchmark on 7MB non-matching text
    let big = "no match here at all xyz 1234 some random text ".repeat(150000);
    eprintln!("haystack: {} bytes", big.len());
    let t = std::time::Instant::now();
    for _ in 0..3 {
        let c = re.count_matches(&big);
        assert_eq!(c, 0);
    }
    eprintln!("3 count_matches scans: {:?} ({:.1}ns/byte)",
        t.elapsed(), t.elapsed().as_nanos() as f64 / big.len() as f64 / 3.0);

    // Test with embedded matches
    let with_matches = format!(
        "{}AKIAABCDEFGH01234567 {}ghp_abcdefghijklmnopqrstuvwxyz0123456789 {}",
        "prefix text ".repeat(10000),
        "middle text ".repeat(10000),
        "suffix text ".repeat(10000),
    );
    let c = re.count_matches(&with_matches);
    eprintln!("count with matches: {}", c);
    assert_eq!(c, 2);
}
