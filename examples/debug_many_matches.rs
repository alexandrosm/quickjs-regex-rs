use aho_corasick::{AhoCorasick, AhoCorasickBuilder, MatchKind};
use std::time::Instant;

fn main() {
    let patterns = &[
        "Sherlock Holmes",
        "John Watson",
        "Irene Adler",
        "Inspector Lestrade",
        "Professor Moriarty",
    ];

    println!("Testing with many matches (simulating sherlock.txt)");

    // Create ~600KB of text with ~100 matches scattered throughout
    let base_text = "The quick brown fox jumps over the lazy dog. This is filler.
More text that doesn't match anything we're looking for here.
Lorem ipsum dolor sit amet, consectetur adipiscing elit.
";

    let mut text = String::with_capacity(600_000);
    let mut match_count = 0;

    // Insert matches every ~5KB or so to get ~100 matches in 600KB
    while text.len() < 550_000 {
        // Add some filler
        for _ in 0..20 {
            text.push_str(base_text);
        }
        // Add a match
        let match_text = match match_count % 5 {
            0 => " Sherlock Holmes appeared. ",
            1 => " John Watson noted. ",
            2 => " Professor Moriarty lurked. ",
            3 => " Inspector Lestrade arrived. ",
            _ => " Irene Adler vanished. ",
        };
        text.push_str(match_text);
        match_count += 1;
    }

    let text_bytes = text.as_bytes();
    println!("Haystack size: {} bytes", text_bytes.len());
    println!("Expected matches: ~{}", match_count);

    // Test 1: Aho-Corasick find_iter (optimal)
    {
        let ac = AhoCorasickBuilder::new()
            .match_kind(MatchKind::LeftmostFirst)
            .build(patterns)
            .unwrap();

        let start = Instant::now();
        let count = ac.find_iter(text_bytes).count();
        let elapsed = start.elapsed();
        println!("AC find_iter: {} matches in {:?}", count, elapsed);
    }

    // Test 2: Aho-Corasick find() in a loop (simulating find_at)
    {
        let ac = AhoCorasickBuilder::new()
            .match_kind(MatchKind::LeftmostFirst)
            .build(patterns)
            .unwrap();

        let start = Instant::now();
        let mut count = 0;
        let mut pos = 0;
        while let Some(mat) = ac.find(&text_bytes[pos..]) {
            count += 1;
            pos += mat.end().max(1);
        }
        let elapsed = start.elapsed();
        println!("AC find() loop: {} matches in {:?}", count, elapsed);
    }

    // Test 3: Our quickjs-regex find_at pattern
    {
        let re = quickjs_regex::Regex::new(
            "Sherlock Holmes|John Watson|Irene Adler|Inspector Lestrade|Professor Moriarty"
        ).unwrap();

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
    }

    // Test 4: rust/regex for comparison
    {
        let re = regex::Regex::new(
            "Sherlock Holmes|John Watson|Irene Adler|Inspector Lestrade|Professor Moriarty"
        ).unwrap();

        let start = Instant::now();
        let count = re.find_iter(&text).count();
        let elapsed = start.elapsed();
        println!("rust/regex: {} matches in {:?}", count, elapsed);
    }
}
