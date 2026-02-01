use quickjs_regex::{Regex, Flags};

fn main() {
    let text = "I saw him on January first 1st 2nd";

    // Test patterns with strategy debug
    let patterns = vec![
        // Works
        ("(january|february)", true),
        // Works
        ("((january)|(february))", true),
        // Fails
        ("((january))", true),
        // Fails
        ("((january)|(february)){1,1}", true),
        // Works
        ("(\\d+)", true),
        // Works
        ("((\\d+))", true),
        // Fails
        ("((19\\d\\d))", true),
    ];

    for (pattern, case_insensitive) in &patterns {
        println!("\n=== Pattern: {} ===", pattern);

        let re = if *case_insensitive {
            Regex::with_flags(pattern, Flags::from_bits(Flags::IGNORE_CASE)).unwrap()
        } else {
            Regex::new(pattern).unwrap()
        };

        // Test find
        match re.find(text) {
            Some(m) => println!("find: '{}' at {}..{}", &text[m.start..m.end], m.start, m.end),
            None => println!("find: no match"),
        }

        // Test find_at starting from 0
        match re.find_at(text, 0) {
            Some(m) => println!("find_at(0): '{}' at {}..{}", &text[m.start..m.end], m.start, m.end),
            None => println!("find_at(0): no match"),
        }

        // Test find_at starting from 13 (right at "January")
        match re.find_at(text, 13) {
            Some(m) => println!("find_at(13): '{}' at {}..{}", &text[m.start..m.end], m.start, m.end),
            None => println!("find_at(13): no match"),
        }
    }
}
