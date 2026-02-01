use quickjs_regex::{Regex, Flags};

fn main() {
    // Test a subset of the date pattern from rebar
    // The real pattern has ((...)|(...)|(...)...){1,1}

    // Test nested alternation with groups
    let pattern = r"((19\d\d|20\d\d)|(january|february|march)|(\d+)){1,1}";

    let re = Regex::with_flags(pattern, Flags::from_bits(Flags::IGNORE_CASE)).unwrap();

    let text = "I saw him on January first 1st 2nd";

    println!("Pattern: {}", pattern);
    println!("Text: {}", text);

    // Count all matches
    let mut count = 0;
    let mut pos = 0;
    while pos < text.len() {
        match re.find_at(text, pos) {
            Some(m) => {
                count += 1;
                println!("  Match {}: '{}' at {}..{}", count, &text[m.start..m.end], m.start, m.end);
                pos = if m.end > m.start { m.end } else { m.start + 1 };
            }
            None => break,
        }
    }
    println!("Total matches: {}", count);

    // Test {1,1} interpretation
    let pattern2 = r"a{1,1}";
    let re2 = Regex::new(pattern2).unwrap();
    let text2 = "aaa";
    let mut count2 = 0;
    let mut pos = 0;
    while pos < text2.len() {
        match re2.find_at(text2, pos) {
            Some(m) => {
                count2 += 1;
                println!("  a{{1,1}} Match: '{}' at {}..{}", &text2[m.start..m.end], m.start, m.end);
                pos = if m.end > m.start { m.end } else { m.start + 1 };
            }
            None => break,
        }
    }
    println!("a{{1,1}} on 'aaa': {} matches", count2);
}
