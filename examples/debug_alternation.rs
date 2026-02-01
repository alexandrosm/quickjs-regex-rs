use quickjs_regex::Regex;
use std::time::Instant;

fn main() {
    // The alternation pattern from rebar benchmarks
    let pattern = "Sherlock Holmes|John Watson|Irene Adler|Inspector Lestrade|Professor Moriarty";

    println!("Pattern: {}", pattern);

    let re = Regex::new(pattern).expect("Failed to compile");
    println!("Strategy: {}", re.strategy_name());

    // Create a sample text (similar to sherlock.txt but smaller for testing)
    let text = "The Adventures of Sherlock Holmes by Arthur Conan Doyle.
Sherlock Holmes sat in his Baker Street rooms. His friend John Watson was visiting.
They were discussing a case involving Professor Moriarty.
Inspector Lestrade had asked for their help.
Irene Adler was mentioned in passing.
More text here to add some padding between matches.
John Watson took notes while Sherlock Holmes paced the room.
Professor Moriarty's network was vast.
Inspector Lestrade brought new evidence.
The game was afoot, said Sherlock Holmes.
";

    // Test find_at iteration (how rebar calls it)
    let mut count = 0;
    let mut pos = 0;
    let start = Instant::now();
    while pos < text.len() {
        match re.find_at(text, pos) {
            Some(m) => {
                count += 1;
                pos = m.end.max(pos + 1);
            }
            None => break,
        }
    }
    let elapsed = start.elapsed();

    println!("Found {} matches in {:?}", count, elapsed);

    // Now let's test with find() loop (how hybrid mode works)
    let mut count2 = 0;
    let mut pos2 = 0;
    let start2 = Instant::now();
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

    println!("Found {} matches via find() slicing in {:?}", count2, elapsed2);

    // List the matches
    println!("\nMatches found:");
    let mut pos = 0;
    while pos < text.len() {
        match re.find_at(text, pos) {
            Some(m) => {
                println!("  '{}' at {}..{}", &text[m.start..m.end], m.start, m.end);
                pos = m.end.max(pos + 1);
            }
            None => break,
        }
    }
}
