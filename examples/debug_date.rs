use quickjs_regex::{Regex, Flags};
use std::time::Instant;

fn main() {
    // Test the exact pattern from rebar benchmark
    let pattern = "Sherlock Holmes|John Watson|Irene Adler|Inspector Lestrade|Professor Moriarty";

    // Test text
    let text = "When SHERLOCK HOLMES solved the case with JOHN WATSON, they met IRENE ADLER. \
                Then Inspector Lestrade arrived with Professor Moriarty.";

    // Case sensitive
    let re_cs = Regex::new(pattern).unwrap();
    let mut count_cs = 0;
    let mut pos = 0;
    while pos < text.len() {
        match re_cs.find_at(text, pos) {
            Some(m) => {
                count_cs += 1;
                println!("CS Match: '{}' at {}..{}", &text[m.start..m.end], m.start, m.end);
                pos = if m.end > m.start { m.end } else { m.start + 1 };
            }
            None => break,
        }
    }
    println!("Case sensitive matches: {}", count_cs);

    // Case insensitive
    let re_ci = Regex::with_flags(pattern, Flags::from_bits(Flags::IGNORE_CASE)).unwrap();
    let mut count_ci = 0;
    let mut pos = 0;
    while pos < text.len() {
        match re_ci.find_at(text, pos) {
            Some(m) => {
                count_ci += 1;
                println!("CI Match: '{}' at {}..{}", &text[m.start..m.end], m.start, m.end);
                pos = if m.end > m.start { m.end } else { m.start + 1 };
            }
            None => break,
        }
    }
    println!("Case insensitive matches: {}", count_ci);

    // Performance test on larger text
    let big_text = text.repeat(1000);
    let iterations = 100;

    let start = Instant::now();
    for _ in 0..iterations {
        let mut pos = 0;
        while pos < big_text.len() {
            match re_cs.find_at(&big_text, pos) {
                Some(m) => { pos = if m.end > m.start { m.end } else { m.start + 1 }; }
                None => break,
            }
        }
    }
    println!("\nCase sensitive: {:?} for {} iterations", start.elapsed(), iterations);

    let start = Instant::now();
    for _ in 0..iterations {
        let mut pos = 0;
        while pos < big_text.len() {
            match re_ci.find_at(&big_text, pos) {
                Some(m) => { pos = if m.end > m.start { m.end } else { m.start + 1 }; }
                None => break,
            }
        }
    }
    println!("Case insensitive: {:?} for {} iterations", start.elapsed(), iterations);
}
