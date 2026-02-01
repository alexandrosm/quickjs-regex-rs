use quickjs_regex::{Regex, Flags};

fn main() {
    // Test the pattern that failed before
    let cases = vec![
        // This failed earlier
        (r"((19\d\d|20\d\d)|(january|february|march)|(\d+)){1,1}", "I saw him on January first 1st 2nd"),
        // Simpler version
        (r"(19\d\d|20\d\d|january|february|march|\d+){1,1}", "I saw him on January first 1st 2nd"),
        // Without {1,1}
        (r"(19\d\d|20\d\d|january|february|march|\d+)", "I saw him on January first 1st 2nd"),
        // With case sensitivity
        (r"(January|19\d\d|\d+)", "I saw him on January first 1st 2nd"),
    ];

    for (pattern, text) in &cases {
        let re = Regex::with_flags(pattern, Flags::from_bits(Flags::IGNORE_CASE)).unwrap();

        let mut matches = Vec::new();
        let mut pos = 0;
        while pos < text.len() {
            match re.find_at(text, pos) {
                Some(m) => {
                    matches.push(&text[m.start..m.end]);
                    pos = if m.end > m.start { m.end } else { m.start + 1 };
                }
                None => break,
            }
        }
        println!("{:60} -> {} matches: {:?}", pattern, matches.len(), matches);
    }
}
