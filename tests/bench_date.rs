use quickjs_regex::{Regex, Flags};

#[test]
fn test_date_wide_nfa() {
    let pattern = std::fs::read_to_string("tests/date_regex.txt").unwrap();
    let re = Regex::with_flags(pattern.trim(), Flags::from_bits(Flags::IGNORE_CASE)).unwrap();
    let mut scratch = re.create_scratch();

    // Small: dense matches
    let small = ("abcdef 2023-01-15 ghijkl ").repeat(100);
    let t1 = std::time::Instant::now();
    let mut c1 = 0; let mut p1 = 0;
    while p1 < small.len() {
        match re.find_at_scratch(&small, p1, &mut scratch) {
            Some(m) => { c1 += 1; p1 = if m.end > m.start { m.end } else { m.start + 1 }; }
            None => break,
        }
    }
    eprintln!("small: {} matches in {:?} ({:.1}us/match, {:.2}us/byte)",
        c1, t1.elapsed(), t1.elapsed().as_nanos() as f64 / c1 as f64 / 1000.0,
        t1.elapsed().as_nanos() as f64 / small.len() as f64 / 1000.0);

    // Large: sparse matches in 200KB of text
    let large = "x".repeat(100_000) + "2023-01-15 test" + &"y".repeat(100_000);
    let t2 = std::time::Instant::now();
    let mut c2 = 0; let mut p2 = 0;
    while p2 < large.len() {
        match re.find_at_scratch(&large, p2, &mut scratch) {
            Some(m) => { c2 += 1; p2 = if m.end > m.start { m.end } else { m.start + 1 }; }
            None => break,
        }
    }
    eprintln!("large: {} matches in {:?} ({:.1}us/match, {:.2}us/byte)",
        c2, t2.elapsed(), t2.elapsed().as_nanos() as f64 / c2.max(1) as f64 / 1000.0,
        t2.elapsed().as_nanos() as f64 / large.len() as f64 / 1000.0);
}
