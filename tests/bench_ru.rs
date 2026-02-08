use quickjs_regex::{Regex, Flags};

#[test]
fn test_sherlock_ru_count() {
    let hay = match std::fs::read_to_string("/tmp/ru.txt")
        .or_else(|_| std::fs::read_to_string("/root/rebar/benchmarks/haystacks/opensubtitles/ru-sampled.txt")) {
        Ok(h) => h,
        Err(_) => {
            // Not on Fly.io, use synthetic Russian text
            let unit = "Привет мир Шерлок Холмс тест ";
            unit.repeat(1000)
        }
    };
    eprintln!("haystack: {} bytes", hay.len());

    let re = Regex::with_flags("Шерлок Холмс", Flags::from_bits(Flags::IGNORE_CASE | Flags::UNICODE)).unwrap();

    for i in 0..5 {
        let start = std::time::Instant::now();
        let c = re.count_matches(&hay);
        eprintln!("iter {}: count={} in {:?}", i, c, start.elapsed());
    }
}
