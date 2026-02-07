use quickjs_regex::{Regex, Flags};

#[test]
fn test_date_wide_nfa() {
    let pattern = std::fs::read_to_string("tests/date_regex.txt").unwrap();
    let re = Regex::with_flags(pattern.trim(), Flags::from_bits(Flags::IGNORE_CASE)).unwrap();
    let mut scratch = re.create_scratch();

    // Try rebar haystack (Fly.io) or synthetic (local)
    let hay = std::fs::read_to_string("/root/rebar/benchmarks/haystacks/rust-src-tools-3b0d4813.txt")
        .ok()
        .map(|s| s.lines().skip(189999).take(10000).collect::<Vec<_>>().join("\n"))
        .unwrap_or_else(|| ("abcdef 2023-01-15 ghijkl ").repeat(100));
    eprintln!("haystack: {} bytes", hay.len());

    // Measure 5 iterations like rebar would
    for iter in 0..5 {
        let t = std::time::Instant::now();
        let mut count = 0usize;
        let mut sum = 0usize;
        let mut pos = 0;
        while pos < hay.len() {
            match re.find_at_scratch(&hay, pos, &mut scratch) {
                Some(m) => { count += 1; sum += m.end - m.start; pos = if m.end > m.start { m.end } else { m.start + 1 }; }
                None => break,
            }
        }
        eprintln!("iter {}: {} matches, sum={}, in {:?} ({:.2}us/byte)",
            iter, count, sum, t.elapsed(),
            t.elapsed().as_nanos() as f64 / hay.len() as f64 / 1000.0);
    }
}
