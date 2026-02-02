//! Test that runs the actual Russian benchmark haystack
use quickjs_regex::{Regex, Flags};

#[test]
#[ignore] // Only run manually
fn test_russian_haystack() {
    let text = std::fs::read_to_string("/root/rebar/benchmarks/haystacks/opensubtitles/ru-sampled.txt")
        .expect("failed to read file");

    // Get first 2500 lines
    let haystack: String = text.lines().take(2500).collect::<Vec<_>>().join("\n");

    let mut flags = Flags::empty();
    flags.insert(Flags::UNICODE);
    let re = Regex::with_flags(r"\b\w+\b", flags).unwrap();

    // Debug: print bytecode flags
    let bc = re.debug_bytecode();
    eprintln!("Bytecode flags: 0x{:02x}{:02x}", bc[1], bc[0]);

    let mut sum = 0usize;
    let mut count = 0usize;
    let mut pos = 0;
    let mut samples = Vec::new();

    while pos < haystack.len() {
        if let Some(m) = re.find_at(&haystack, pos) {
            sum += m.end - m.start;
            count += 1;
            if samples.len() < 20 {
                samples.push((&haystack[m.start..m.end]).to_string());
            }
            pos = if m.end > m.start { m.end } else { m.start + 1 };
        } else {
            break;
        }
    }

    eprintln!("Total matches: {}", count);
    eprintln!("Total span sum: {}", sum);
    eprintln!("Expected span sum: 107391");
    eprintln!("Sample matches: {:?}", samples);
    // Get first ~200 chars safely (respecting UTF-8 boundaries)
    let first_chars: String = haystack.chars().take(200).collect();
    eprintln!("First 200 chars: {:?}", first_chars);

    // Should match approximately 107391 bytes of text
    assert!(sum > 100000, "Expected sum > 100000, got {}", sum);
}
