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

    println!("Testing Aho-Corasick configurations with {} patterns", patterns.len());

    // Create ~900KB of text
    let base_text = "The quick brown fox jumps over the lazy dog. This is some filler text.
More text here that doesn't contain any of the search terms.
Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
";

    let mut text = String::with_capacity(900_000);
    while text.len() < 850_000 {
        text.push_str(base_text);
    }
    text.insert_str(100_000, " Sherlock Holmes was here. ");
    text.insert_str(200_000, " John Watson arrived. ");
    text.insert_str(300_000, " Professor Moriarty lurked. ");
    text.insert_str(500_000, " Inspector Lestrade investigated. ");
    text.insert_str(700_000, " Irene Adler vanished. ");
    text.insert_str(800_000, " Sherlock Holmes deduced. ");

    let text_bytes = text.as_bytes();
    println!("Haystack size: {} bytes", text_bytes.len());

    // Test 1: Default Aho-Corasick
    {
        let ac = AhoCorasick::new(patterns).unwrap();
        let start = Instant::now();
        let count = ac.find_iter(text_bytes).count();
        let elapsed = start.elapsed();
        println!("Default AC: {} matches in {:?}", count, elapsed);
    }

    // Test 2: LeftmostFirst (what we want for find)
    {
        let ac = AhoCorasickBuilder::new()
            .match_kind(MatchKind::LeftmostFirst)
            .build(patterns)
            .unwrap();
        let start = Instant::now();
        let count = ac.find_iter(text_bytes).count();
        let elapsed = start.elapsed();
        println!("LeftmostFirst AC: {} matches in {:?}", count, elapsed);
    }

    // Test 3: Check DFA usage
    {
        // The default should use DFA on x86_64, let's verify
        let ac = AhoCorasickBuilder::new()
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
        println!("AC with manual iteration: {} matches in {:?}", count, elapsed);
    }

    // Test 4: rust/regex for comparison
    {
        let pattern = "Sherlock Holmes|John Watson|Irene Adler|Inspector Lestrade|Professor Moriarty";
        let re = regex::Regex::new(pattern).unwrap();
        let start = Instant::now();
        let count = re.find_iter(&text).count();
        let elapsed = start.elapsed();
        println!("rust/regex: {} matches in {:?}", count, elapsed);
    }

    // Test 5: Direct memchr for single pattern
    {
        use memchr::memmem;
        let finder = memmem::Finder::new(b"Sherlock Holmes");
        let start = Instant::now();
        let count = finder.find_iter(text_bytes).count();
        let elapsed = start.elapsed();
        println!("memchr/memmem (single): {} matches in {:?}", count, elapsed);
    }

    // Test 6: Multiple memmem searches (manual multi-pattern)
    {
        use memchr::memmem;
        let finders: Vec<_> = patterns.iter()
            .map(|p| memmem::Finder::new(p.as_bytes()))
            .collect();

        let start = Instant::now();
        let mut matches: Vec<usize> = Vec::new();
        for finder in &finders {
            for pos in finder.find_iter(text_bytes) {
                matches.push(pos);
            }
        }
        matches.sort();
        let count = matches.len();
        let elapsed = start.elapsed();
        println!("Multiple memmem (unsorted): {} matches in {:?}", count, elapsed);
    }
}
