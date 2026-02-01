use std::time::Instant;

fn main() {
    // Read the actual sherlock.txt from rebar
    let text = match std::fs::read_to_string("/root/rebar/benchmarks/haystacks/sherlock.txt") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Could not read sherlock.txt: {}", e);
            eprintln!("Trying local path...");
            std::fs::read_to_string("benchmarks/haystacks/sherlock.txt")
                .expect("Could not read sherlock.txt from any path")
        }
    };

    let pattern = "Sherlock Holmes|John Watson|Irene Adler|Inspector Lestrade|Professor Moriarty";
    println!("Pattern: {}", pattern);
    println!("Haystack size: {} bytes", text.len());

    // Test quickjs-regex
    let re = quickjs_regex::Regex::new(pattern).expect("Failed to compile");
    println!("Strategy: {}", re.strategy_name());

    // Warmup
    for _ in 0..3 {
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
        let _ = count;
    }

    // Measure quickjs find_at
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
    println!("quickjs find_at: {} matches in {:?}", count, elapsed);

    // Measure quickjs find() with slicing
    let start = Instant::now();
    let mut count2 = 0;
    let mut pos = 0;
    while pos < text.len() {
        match re.find(&text[pos..]) {
            Some(m) => {
                count2 += 1;
                pos += m.end.max(1);
            }
            None => break,
        }
    }
    let elapsed = start.elapsed();
    println!("quickjs find() slice: {} matches in {:?}", count2, elapsed);

    // Measure rust/regex
    let rust_re = regex::Regex::new(pattern).unwrap();

    let start = Instant::now();
    let count3 = rust_re.find_iter(&text).count();
    let elapsed = start.elapsed();
    println!("rust/regex find_iter: {} matches in {:?}", count3, elapsed);

    // Measure rust/regex with manual loop (like rebar)
    let start = Instant::now();
    let mut count4 = 0;
    let mut pos = 0;
    while let Some(m) = rust_re.find_at(&text, pos) {
        count4 += 1;
        pos = m.end().max(pos + 1);
    }
    let elapsed = start.elapsed();
    println!("rust/regex find_at loop: {} matches in {:?}", count4, elapsed);
}
