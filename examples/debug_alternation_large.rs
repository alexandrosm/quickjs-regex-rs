use quickjs_regex::Regex;
use std::time::Instant;

fn main() {
    // The alternation pattern from rebar benchmarks
    let pattern = "Sherlock Holmes|John Watson|Irene Adler|Inspector Lestrade|Professor Moriarty";

    println!("Pattern: {}", pattern);

    let re = Regex::new(pattern).expect("Failed to compile");
    println!("Strategy: {}", re.strategy_name());

    // Create a ~900KB haystack similar to rebar's sherlock.txt
    // The actual sherlock.txt has about 6 mentions of "Sherlock Holmes", etc.
    let base_text = "The quick brown fox jumps over the lazy dog. This is some filler text to make the haystack larger.
More text here that doesn't contain any of the search terms.
Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
";

    // Create ~900KB of text
    let mut text = String::with_capacity(900_000);
    while text.len() < 850_000 {
        text.push_str(base_text);
    }
    // Add some actual matches scattered throughout
    text.insert_str(100_000, " Sherlock Holmes was here. ");
    text.insert_str(200_000, " John Watson arrived. ");
    text.insert_str(300_000, " Professor Moriarty lurked. ");
    text.insert_str(500_000, " Inspector Lestrade investigated. ");
    text.insert_str(700_000, " Irene Adler vanished. ");
    text.insert_str(800_000, " Sherlock Holmes deduced. ");

    println!("Haystack size: {} bytes", text.len());

    // Test find_at iteration (how rebar calls it)
    let start = Instant::now();
    let mut count = 0;
    let mut pos = 0;
    while pos < text.len() {
        match re.find_at(&text, pos) {
            Some(m) => {
                count += 1;
                pos = m.end.max(pos + 1);
            }
            None => break,
        }
    }
    let elapsed = start.elapsed();
    println!("find_at: Found {} matches in {:?}", count, elapsed);

    // Test find() with slicing
    let start2 = Instant::now();
    let mut count2 = 0;
    let mut pos2 = 0;
    while pos2 < text.len() {
        match re.find(&text[pos2..]) {
            Some(m) => {
                count2 += 1;
                pos2 += m.end.max(1);
            }
            None => break,
        }
    }
    let elapsed2 = start2.elapsed();
    println!("find(): Found {} matches in {:?}", count2, elapsed2);

    // For reference, test rust/regex
    let rust_re = regex::Regex::new(pattern).unwrap();
    let start3 = Instant::now();
    let count3 = rust_re.find_iter(&text).count();
    let elapsed3 = start3.elapsed();
    println!("rust/regex: Found {} matches in {:?}", count3, elapsed3);
}
