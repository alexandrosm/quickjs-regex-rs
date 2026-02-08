use quickjs_regex::{Regex, Flags};

#[test]
fn test_exec_perf() {
    let re = Regex::with_flags(r"\b\w+\b", Flags::from_bits(Flags::UNICODE)).unwrap();
    let hay = "hello world this is a test ".repeat(100);
    eprintln!("haystack: {} bytes", hay.len());

    // Plain find (uses find_at internally)
    let t = std::time::Instant::now();
    let m = re.find(&hay);
    eprintln!("find: {:?} in {:?}", m, t.elapsed());

    // count_matches (uses PikeScanner internally)
    let t2 = std::time::Instant::now();
    let c = re.count_matches(&hay);
    eprintln!("count_matches: {} in {:?}", c, t2.elapsed());

    // find_iter
    let t3 = std::time::Instant::now();
    let c3: usize = re.find_iter(&hay).map(|m| m.end - m.start).sum();
    eprintln!("find_iter spans: {} in {:?}", c3, t3.elapsed());
}
