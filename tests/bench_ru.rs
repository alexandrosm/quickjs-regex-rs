use quickjs_regex::{Regex, Flags};

#[test]
fn test_sherlock_ru_count() {
    let hay = match std::fs::read_to_string("/root/rebar/benchmarks/haystacks/opensubtitles/ru-sampled.txt") {
        Ok(h) => h,
        Err(_) => "Привет мир Шерлок Холмс тест ".repeat(1000),
    };
    let re = Regex::with_flags("Шерлок Холмс", Flags::from_bits(Flags::IGNORE_CASE | Flags::UNICODE)).unwrap();
    for i in 0..3 {
        let start = std::time::Instant::now();
        let c = re.count_matches(&hay);
        eprintln!("count_matches iter {}: count={} in {:?}", i, c, start.elapsed());
    }
}

#[test]
fn test_words_ru_spans() {
    let hay = match std::fs::read_to_string("/root/rebar/benchmarks/haystacks/opensubtitles/ru-sampled.txt") {
        Ok(h) => {
            // Take first 2500 lines like rebar does
            h.lines().take(2500).collect::<Vec<_>>().join("\n")
        }
        Err(_) => "Привет мир тест слово текст ".repeat(100),
    };
    eprintln!("haystack: {} bytes", hay.len());
    let re = Regex::with_flags("\\b\\w+\\b", Flags::from_bits(Flags::UNICODE)).unwrap();
    let mut scratch = re.create_scratch();

    for i in 0..3 {
        let start = std::time::Instant::now();
        let mut count = 0usize;
        let mut sum = 0usize;
        let mut pos = 0;
        while pos < hay.len() {
            match re.find_at_scratch(&hay, pos, &mut scratch) {
                Some(m) => {
                    count += 1;
                    sum += m.end - m.start;
                    pos = if m.end > m.start { m.end } else { m.start + 1 };
                }
                None => break,
            }
        }
        eprintln!("find_at_scratch iter {}: {} matches, sum={}, in {:?}", i, count, sum, start.elapsed());
    }
}
