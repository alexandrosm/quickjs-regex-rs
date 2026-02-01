use quickjs_regex::{Regex, Flags};

fn main() {
    // Simplified version of the date pattern structure:
    // ((alternations at depth 1){1,1})
    let pattern = r"((19\d\d|20\d\d|jan|feb|mar|\d+){1,1})";

    println!("Testing pattern structure similar to date regex");
    println!("Pattern: {}", pattern);

    let text = "Meeting in January 2024 on the 15th";

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

    println!("Text: {}", text);
    println!("Found {} matches:", matches.len());
    for (s, start, end) in &matches {
        println!("  '{}' at {}..{}", s, start, end);
    }

    // Also test using find() in a loop like the benchmark does
    println!("\nUsing find() on slices (like benchmark Hybrid mode):");
    let mut count = 0;
    let mut pos = 0;
    while pos < text.len() {
        match re.find(&text[pos..]) {
            Some(m) => {
                count += 1;
                let abs_start = pos + m.start;
                let abs_end = pos + m.end;
                println!("  '{}' at {}..{}", &text[abs_start..abs_end], abs_start, abs_end);
                pos = if abs_end > abs_start { abs_end } else { abs_start + 1 };
            }
            None => break,
        }
    }
    println!("Total: {} matches", count);
}
