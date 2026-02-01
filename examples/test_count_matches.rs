use quickjs_regex::Regex;
use std::time::Instant;

fn main() {
    // Read the actual haystack from rebar
    let text = match std::fs::read_to_string("/root/rebar/benchmarks/haystacks/opensubtitles/en-sampled.txt") {
        Ok(t) => t,
        Err(_) => {
            eprintln!("Could not read haystack, using generated text");
            generate_text()
        }
    };

    let pattern = "Sherlock Holmes|John Watson|Irene Adler|Inspector Lestrade|Professor Moriarty";
    println!("Pattern: {}", pattern);
    println!("Haystack size: {} bytes", text.len());

    let re = Regex::new(pattern).expect("Failed to compile");
    println!("Strategy: {}", re.strategy_name());

    // Warmup
    for _ in 0..5 {
        let _ = re.count_matches(&text);
    }

    // Test count_matches
    let start = Instant::now();
    let mut total_count = 0;
    for _ in 0..1000 {
        total_count = re.count_matches(&text);
    }
    let elapsed = start.elapsed();
    println!("count_matches: {} matches, {} iterations in {:?}", total_count, 1000, elapsed);
    println!("  per-iteration: {:?}", elapsed / 1000);

    // Test find_iter().count()
    let start = Instant::now();
    let mut total_count = 0;
    for _ in 0..1000 {
        total_count = re.find_iter(&text).count();
    }
    let elapsed = start.elapsed();
    println!("find_iter().count(): {} matches, {} iterations in {:?}", total_count, 1000, elapsed);
    println!("  per-iteration: {:?}", elapsed / 1000);

    // Test find_at loop
    let start = Instant::now();
    let mut total_count = 0;
    for _ in 0..1000 {
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
        total_count = count;
    }
    let elapsed = start.elapsed();
    println!("find_at loop: {} matches, {} iterations in {:?}", total_count, 1000, elapsed);
    println!("  per-iteration: {:?}", elapsed / 1000);
}

fn generate_text() -> String {
    let base = "The quick brown fox jumps over the lazy dog. ";
    let mut text = String::with_capacity(900_000);
    while text.len() < 850_000 {
        text.push_str(base);
    }
    text.insert_str(100_000, " Sherlock Holmes ");
    text.insert_str(300_000, " John Watson ");
    text.insert_str(500_000, " Irene Adler ");
    text.insert_str(700_000, " Professor Moriarty ");
    text
}
