//! Benchmarks comparing quickjs-regex with regress
//!
//! Run with: cargo bench

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use quickjs_regex::Regex as QjsRegex;
use regress::Regex as RegressRegex;

// ============================================================================
// Compilation benchmarks
// ============================================================================

fn bench_compile(c: &mut Criterion) {
    let patterns = [
        ("simple", "hello"),
        ("alternation", "cat|dog|bird|fish"),
        ("quantifiers", r"a{2,4}b+c*d?"),
        ("character_class", r"[a-zA-Z0-9_]+"),
        ("groups", r"(\w+)@(\w+)\.(\w+)"),
        ("complex", r"^(?:https?://)?(?:www\.)?([a-zA-Z0-9-]+)\.([a-z]{2,})(?:/.*)?$"),
    ];

    let mut group = c.benchmark_group("compile");

    for (name, pattern) in patterns.iter() {
        group.bench_with_input(BenchmarkId::new("qjs", name), pattern, |b, p| {
            b.iter(|| QjsRegex::new(black_box(p)).unwrap())
        });

        group.bench_with_input(BenchmarkId::new("regress", name), pattern, |b, p| {
            b.iter(|| RegressRegex::new(black_box(p)).unwrap())
        });
    }

    group.finish();
}

// ============================================================================
// Matching benchmarks (is_match / test)
// ============================================================================

fn bench_is_match(c: &mut Criterion) {
    let cases = [
        ("literal_hit", "hello", "hello world"),
        ("literal_miss", "xyz", "hello world"),
        ("digits", r"\d+", "abc123def456"),
        ("email", r"\w+@\w+\.\w+", "contact user@example.com today"),
        ("url", r"https?://\w+", "Visit https://example.com for more"),
        ("word_boundary", r"\bword\b", "find the word in text"),
    ];

    let mut group = c.benchmark_group("is_match");

    for (name, pattern, input) in cases.iter() {
        let qjs_re = QjsRegex::new(pattern).unwrap();
        let regress_re = RegressRegex::new(pattern).unwrap();

        group.bench_with_input(BenchmarkId::new("qjs", name), input, |b, s| {
            b.iter(|| qjs_re.is_match(black_box(s)))
        });

        group.bench_with_input(BenchmarkId::new("regress", name), input, |b, s| {
            b.iter(|| regress_re.find(black_box(s)).is_some())
        });
    }

    group.finish();
}

// ============================================================================
// Find benchmarks
// ============================================================================

fn bench_find(c: &mut Criterion) {
    // Build dynamic strings first
    let late_match_input = "a".repeat(100) + "xyz";
    let greedy_input = "b".repeat(50) + &"a".repeat(50);

    let cases: Vec<(&str, &str, &str)> = vec![
        ("first_match", r"\d+", "abc123def456ghi789"),
        ("late_match", r"xyz", &late_match_input),
        ("alternation", "cat|dog", "the quick brown fox jumps over the lazy dog"),
        ("greedy", "a+", &greedy_input),
    ];

    let mut group = c.benchmark_group("find");

    for (name, pattern, input) in cases.iter() {
        let qjs_re = QjsRegex::new(pattern).unwrap();
        let regress_re = RegressRegex::new(pattern).unwrap();

        group.bench_with_input(BenchmarkId::new("qjs", name), input, |b, s| {
            b.iter(|| qjs_re.find(black_box(s)))
        });

        group.bench_with_input(BenchmarkId::new("regress", name), input, |b, s| {
            b.iter(|| regress_re.find(black_box(s)))
        });
    }

    group.finish();
}

// ============================================================================
// Find all matches benchmarks
// ============================================================================

fn bench_find_all(c: &mut Criterion) {
    let text_with_numbers = "Item 1: $10, Item 2: $25, Item 3: $100, Item 4: $50, Item 5: $75";
    let text_with_words = "The quick brown fox jumps over the lazy dog near the red barn";

    let mut group = c.benchmark_group("find_all");

    // Find all numbers
    let qjs_num = QjsRegex::new(r"\d+").unwrap();
    let regress_num = RegressRegex::new(r"\d+").unwrap();

    group.bench_function("qjs/numbers", |b| {
        b.iter(|| {
            qjs_num.find_iter(black_box(text_with_numbers)).count()
        })
    });

    group.bench_function("regress/numbers", |b| {
        b.iter(|| {
            regress_num.find_iter(black_box(text_with_numbers)).count()
        })
    });

    // Find all words
    let qjs_word = QjsRegex::new(r"\b\w+\b").unwrap();
    let regress_word = RegressRegex::new(r"\b\w+\b").unwrap();

    group.bench_function("qjs/words", |b| {
        b.iter(|| {
            qjs_word.find_iter(black_box(text_with_words)).count()
        })
    });

    group.bench_function("regress/words", |b| {
        b.iter(|| {
            regress_word.find_iter(black_box(text_with_words)).count()
        })
    });

    group.finish();
}

// ============================================================================
// Capture groups benchmarks
// ============================================================================

fn bench_captures(c: &mut Criterion) {
    let mut group = c.benchmark_group("captures");

    // Email parsing
    let email_pattern = r"(\w+)@(\w+)\.(\w+)";
    let email_input = "Contact us at support@example.com for help";

    let qjs_email = QjsRegex::new(email_pattern).unwrap();
    let regress_email = RegressRegex::new(email_pattern).unwrap();

    group.bench_function("qjs/email", |b| {
        b.iter(|| qjs_email.captures(black_box(email_input)))
    });

    group.bench_function("regress/email", |b| {
        b.iter(|| regress_email.find(black_box(email_input)))
    });

    // Date parsing
    let date_pattern = r"(\d{4})-(\d{2})-(\d{2})";
    let date_input = "The event is on 2024-12-25 this year";

    let qjs_date = QjsRegex::new(date_pattern).unwrap();
    let regress_date = RegressRegex::new(date_pattern).unwrap();

    group.bench_function("qjs/date", |b| {
        b.iter(|| qjs_date.captures(black_box(date_input)))
    });

    group.bench_function("regress/date", |b| {
        b.iter(|| regress_date.find(black_box(date_input)))
    });

    group.finish();
}

// ============================================================================
// Long input benchmarks
// ============================================================================

fn bench_long_input(c: &mut Criterion) {
    let mut group = c.benchmark_group("long_input");

    // Create long inputs
    let long_text = "a".repeat(10000);
    let long_with_match = "b".repeat(9990) + "aaaaaaaaaa";

    // Match at start of long input
    let qjs_re = QjsRegex::new("a+").unwrap();
    let regress_re = RegressRegex::new("a+").unwrap();

    group.bench_function("qjs/match_at_start", |b| {
        b.iter(|| qjs_re.find(black_box(&long_text)))
    });

    group.bench_function("regress/match_at_start", |b| {
        b.iter(|| regress_re.find(black_box(&long_text)))
    });

    // Match at end of long input
    group.bench_function("qjs/match_at_end", |b| {
        b.iter(|| qjs_re.find(black_box(&long_with_match)))
    });

    group.bench_function("regress/match_at_end", |b| {
        b.iter(|| regress_re.find(black_box(&long_with_match)))
    });

    // No match in long input
    let no_match_re_qjs = QjsRegex::new("xyz").unwrap();
    let no_match_re_regress = RegressRegex::new("xyz").unwrap();

    group.bench_function("qjs/no_match", |b| {
        b.iter(|| no_match_re_qjs.find(black_box(&long_text)))
    });

    group.bench_function("regress/no_match", |b| {
        b.iter(|| no_match_re_regress.find(black_box(&long_text)))
    });

    group.finish();
}

// ============================================================================
// Real-world pattern benchmarks
// ============================================================================

fn bench_realistic(c: &mut Criterion) {
    let mut group = c.benchmark_group("realistic");

    // Log line parsing
    let log_pattern = r"\[(\d{4}-\d{2}-\d{2})\s+(\d{2}:\d{2}:\d{2})\]\s+(\w+):\s+(.*)";
    let log_line = "[2024-01-15 14:30:45] ERROR: Connection timeout after 30s";

    let qjs_log = QjsRegex::new(log_pattern).unwrap();
    let regress_log = RegressRegex::new(log_pattern).unwrap();

    group.bench_function("qjs/log_parse", |b| {
        b.iter(|| qjs_log.captures(black_box(log_line)))
    });

    group.bench_function("regress/log_parse", |b| {
        b.iter(|| regress_log.find(black_box(log_line)))
    });

    // HTML tag matching
    let html_pattern = r"<(\w+)(?:\s+[^>]*)?>([^<]*)</\1>";
    let html_input = "<div class=\"container\">Hello World</div>";

    let qjs_html = QjsRegex::new(html_pattern).unwrap();
    let regress_html = RegressRegex::new(html_pattern).unwrap();

    group.bench_function("qjs/html_tag", |b| {
        b.iter(|| qjs_html.captures(black_box(html_input)))
    });

    group.bench_function("regress/html_tag", |b| {
        b.iter(|| regress_html.find(black_box(html_input)))
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_compile,
    bench_is_match,
    bench_find,
    bench_find_all,
    bench_captures,
    bench_long_input,
    bench_realistic
);

criterion_main!(benches);
