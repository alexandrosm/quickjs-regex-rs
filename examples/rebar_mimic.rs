use quickjs_regex::Regex;
use std::time::{Duration, Instant};

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

    // Mimic the timer::run function behavior
    // Run for ~4.5 seconds like rebar does
    let max_time = Duration::from_millis(4500);
    let mut total_iters = 0;
    let mut total_time = Duration::ZERO;
    let mut samples = Vec::new();

    let benchmark_start = Instant::now();

    while benchmark_start.elapsed() < max_time {
        // Time a single iteration
        let iter_start = Instant::now();
        let count = find_all(&re, &text);
        let iter_elapsed = iter_start.elapsed();

        total_iters += 1;
        total_time += iter_elapsed;
        samples.push((iter_elapsed, count));
    }

    // Calculate statistics
    let mut times: Vec<_> = samples.iter().map(|(t, _)| *t).collect();
    times.sort();
    let median = times[times.len() / 2];
    let mean = total_time / total_iters;
    let min = times[0];
    let max = times[times.len() - 1];

    println!("\nResults ({} iterations):", total_iters);
    println!("  Median: {:?}", median);
    println!("  Mean:   {:?}", mean);
    println!("  Min:    {:?}", min);
    println!("  Max:    {:?}", max);
    println!("  Count:  {}", samples[0].1);
}

fn find_all(re: &Regex, haystack: &str) -> usize {
    let mut count = 0;
    let mut pos = 0;
    while pos < haystack.len() {
        match re.find_at(haystack, pos) {
            Some(m) => {
                count += 1;
                pos = if m.end > m.start { m.end } else { m.start + 1 };
            }
            None => break,
        }
    }
    count
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
