use quickjs_regex::{Regex, Flags};

fn main() {
    let text = "I saw him on January first 1st 2nd";

    // Test cases to isolate the issue
    let cases = vec![
        // Simple {1,1} - works
        ("a{1,1}", "aaa", true),

        // Group with {1,1} - does this work?
        ("(a){1,1}", "aaa", true),

        // Alternation with {1,1}
        ("(a|b){1,1}", "aaa", true),

        // Nested groups without {1,1}
        ("((a)|(b))", "aaa", true),

        // Nested groups with {1,1}
        ("((a)|(b)){1,1}", "aaa", true),

        // More realistic nested groups without {1,1}
        ("((19\\d\\d)|(january))", text, false),

        // Nested groups with {1,1}
        ("((19\\d\\d)|(january)){1,1}", text, false),

        // Three-way nested without {1,1}
        ("((19\\d\\d)|(january)|(\\d+))", text, false),

        // Three-way nested with {1,1}
        ("((19\\d\\d)|(january)|(\\d+)){1,1}", text, false),

        // Simple digit pattern
        ("\\d+", text, false),

        // Group around digit
        ("(\\d+)", text, false),

        // Group around digit with {1,1}
        ("(\\d+){1,1}", text, false),

        // Nested group around digit
        ("((\\d+))", text, false),

        // Nested group around digit with {1,1}
        ("((\\d+)){1,1}", text, false),
    ];

    for (pattern, test_text, case_sensitive) in &cases {
        let re = if *case_sensitive {
            Regex::new(pattern).unwrap()
        } else {
            Regex::with_flags(pattern, Flags::from_bits(Flags::IGNORE_CASE)).unwrap()
        };

        let mut matches = Vec::new();
        let mut pos = 0;
        while pos < test_text.len() {
            match re.find_at(test_text, pos) {
                Some(m) => {
                    matches.push(&test_text[m.start..m.end]);
                    pos = if m.end > m.start { m.end } else { m.start + 1 };
                }
                None => break,
            }
        }
        let cs = if *case_sensitive { "CS" } else { "CI" };
        println!("{} {:50} -> {} matches: {:?}", cs, pattern, matches.len(), matches);
    }
}
