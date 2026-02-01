use quickjs_regex::{Regex, Flags};

fn main() {
    // First portion of the actual date pattern
    let pattern = r"((19\d\d01[0-3]\d[0-5]\d[0-5]\d[0-5]\d|20\d\d01[0-3]\d[0-5]\d[0-5]\d[0-5]\d|19\d\d|20\d\d|january|february|march|april|may|june|july|august|september|october|november|december|\d+){1,1})";

    println!("Testing with portion of real date regex");
    println!("Pattern length: {}", pattern.len());

    let text = "Meeting on January 15, 2024 and February 20, 2023";

    let re = match Regex::with_flags(pattern, Flags::from_bits(Flags::IGNORE_CASE)) {
        Ok(r) => r,
        Err(e) => {
            println!("Error compiling: {:?}", e);
            return;
        }
    };

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

    println!("Text: {}", text);
    println!("Found {} matches:", matches.len());
    for (s, start, end) in &matches {
        println!("  '{}' at {}..{}", s, start, end);
    }
}
