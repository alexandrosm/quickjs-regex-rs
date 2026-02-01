use quickjs_regex::{Regex, Flags};

fn main() {
    let text = "I saw him on January first 1st 2nd";

    // This specific pattern doesn't work
    let pattern = r"((19\d\d)|(january)){1,1}";

    let re = Regex::with_flags(pattern, Flags::from_bits(Flags::IGNORE_CASE)).unwrap();

    println!("Pattern: {}", pattern);
    println!("Text: {:?}", text);

    // Test find
    match re.find(text) {
        Some(m) => println!("find: '{}' at {}..{}", &text[m.start..m.end], m.start, m.end),
        None => println!("find: no match"),
    }

    // Test at various positions
    for pos in [0, 12, 13, 14, 20, 27] {
        match re.find_at(text, pos) {
            Some(m) => println!("find_at({:2}): '{}' at {}..{}", pos, &text[m.start..m.end], m.start, m.end),
            None => println!("find_at({:2}): no match", pos),
        }
    }

    // Test without {1,1}
    println!("\n--- Without {{1,1}} ---");
    let pattern2 = r"((19\d\d)|(january))";
    let re2 = Regex::with_flags(pattern2, Flags::from_bits(Flags::IGNORE_CASE)).unwrap();

    match re2.find(text) {
        Some(m) => println!("find: '{}' at {}..{}", &text[m.start..m.end], m.start, m.end),
        None => println!("find: no match"),
    }

    for pos in [0, 12, 13, 14] {
        match re2.find_at(text, pos) {
            Some(m) => println!("find_at({:2}): '{}' at {}..{}", pos, &text[m.start..m.end], m.start, m.end),
            None => println!("find_at({:2}): no match", pos),
        }
    }

    // Test with different first character
    println!("\n--- With j instead of 1/2 ---");
    let pattern3 = r"((january)|(february)){1,1}";
    let re3 = Regex::with_flags(pattern3, Flags::from_bits(Flags::IGNORE_CASE)).unwrap();

    match re3.find(text) {
        Some(m) => println!("find: '{}' at {}..{}", &text[m.start..m.end], m.start, m.end),
        None => println!("find: no match"),
    }
}
