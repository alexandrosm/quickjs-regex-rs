use quickjs_regex::{Regex, Flags};

fn main() {
    let text = "I saw him on January first 1st 2nd";

    // Test case-insensitive nested groups in detail
    let cases = vec![
        // Two alternatives in outer group - without nested groups
        ("(january|february)", true),

        // Two alternatives in nested groups
        ("((january)|(february))", true),

        // Add third alternative - this might break
        ("((january)|(february)|(march))", true),

        // Two alternatives with {1,1}
        ("((january)|(february)){1,1}", true),

        // Simple nested - works
        ("((january))", true),

        // Simple nested with other pattern - this breaks?
        ("((january)|(\\d+))", true),

        // Just digit pattern nested
        ("((\\d+))", true),

        // Year pattern nested
        ("((19\\d\\d))", true),

        // Year OR month - this breaks
        ("((19\\d\\d)|(january))", true),

        // Case sensitive version - should work
        ("((19\\d\\d)|(January))", false),
    ];

    for (pattern, case_insensitive) in &cases {
        let re = if *case_insensitive {
            Regex::with_flags(pattern, Flags::from_bits(Flags::IGNORE_CASE)).unwrap()
        } else {
            Regex::new(pattern).unwrap()
        };

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
        let mode = if *case_insensitive { "CI" } else { "CS" };
        println!("{} {:50} -> {} matches: {:?}", mode, pattern, matches.len(), matches);
    }
}
