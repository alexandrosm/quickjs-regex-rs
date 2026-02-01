use quickjs_regex::{Regex, Flags};

fn main() {
    // Simplified date pattern similar to the rebar benchmark
    // The key issue was nested alternations with {1,1}
    let pattern = r"((19\d\d|20\d\d)|(january|february|march|april|may|june|july|august|september|october|november|december)|(\d+)){1,1}";

    let text = "Meeting on January 15 2024 and February 20 2023 at 10am";

    let re = Regex::with_flags(pattern, Flags::from_bits(Flags::IGNORE_CASE)).unwrap();

    let mut matches = Vec::new();
    let mut pos = 0;
    while pos < text.len() {
        match re.find_at(text, pos) {
            Some(m) => {
                matches.push((&text[m.start..m.end], m.start, m.end));
                pos = if m.end > m.start { m.end } else { m.start + 1 };
            }
            None => break,
        }
    }

    println!("Pattern: {}", pattern);
    println!("Text: {}", text);
    println!("Found {} matches:", matches.len());
    for (s, start, end) in &matches {
        println!("  '{}' at {}..{}", s, start, end);
    }

    // Expected: January, 15, 2024, February, 20, 2023, 10
    let expected = 7;
    if matches.len() == expected {
        println!("\nSUCCESS: Found expected {} matches", expected);
    } else {
        println!("\nFAILURE: Expected {} matches, got {}", expected, matches.len());
    }
}
