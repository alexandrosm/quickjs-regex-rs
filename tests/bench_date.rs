use quickjs_regex::{Regex, Flags};

#[test]
fn test_date_wide_nfa() {
    let pattern = std::fs::read_to_string("tests/date_regex.txt").unwrap();
    let pattern = pattern.trim();
    let re = Regex::with_flags(pattern, Flags::from_bits(Flags::IGNORE_CASE)).unwrap();

    let unit = "abcdef 2023-01-15 ghijkl ";
    let medium = unit.repeat(100);
    let mut scratch = re.create_scratch();

    // Scratch path
    let t1 = std::time::Instant::now();
    let mut c1 = 0;
    let mut p1 = 0;
    while p1 < medium.len() {
        match re.find_at_scratch(&medium, p1, &mut scratch) {
            Some(m) => { c1 += 1; p1 = if m.end > m.start { m.end } else { m.start + 1 }; }
            None => break,
        }
    }
    eprintln!("scratch: {} matches in {:?} ({:.1}us/match)", c1, t1.elapsed(), t1.elapsed().as_micros() as f64 / c1 as f64);

    // Plain find_at (Pike VM exec, no scratch)
    let t2 = std::time::Instant::now();
    let mut c2 = 0;
    let mut p2 = 0;
    while p2 < medium.len() {
        match re.find_at(&medium, p2) {
            Some(m) => { c2 += 1; p2 = if m.end > m.start { m.end } else { m.start + 1 }; }
            None => break,
        }
    }
    eprintln!("find_at: {} matches in {:?} ({:.1}us/match)", c2, t2.elapsed(), t2.elapsed().as_micros() as f64 / c2 as f64);

    // find_iter (persistent PikeScanner)
    let t3 = std::time::Instant::now();
    let c3 = re.find_iter(&medium).count();
    eprintln!("find_iter: {} matches in {:?} ({:.1}us/match)", c3, t3.elapsed(), t3.elapsed().as_micros() as f64 / c3 as f64);

    // count_matches (optimized counting)
    let t4 = std::time::Instant::now();
    let c4 = re.count_matches(&medium);
    eprintln!("count_matches: {} matches in {:?} ({:.1}us/match)", c4, t4.elapsed(), t4.elapsed().as_micros() as f64 / c4 as f64);
}
