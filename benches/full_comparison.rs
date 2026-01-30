//! Comprehensive benchmark comparing all engine configurations:
//! - Hybrid: qjs-regex with search optimizations + Rust interpreter (default find())
//! - Pure Rust: qjs-regex Rust interpreter only (find_at directly)
//! - C Engine: qjs-regex original C engine (find_at_c_engine)
//! - Regress: Pure Rust JS-compatible regex crate
//!
//! Run with: cargo bench --bench full_comparison

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use quickjs_regex::Regex as QjsRegex;
use regress::Regex as RegressRegex;

/// Benchmark helper that runs all 4 engines on the same pattern/input
fn bench_four_engines(
    group: &mut criterion::BenchmarkGroup<criterion::measurement::WallTime>,
    name: &str,
    pattern: &str,
    input: &str,
) {
    let qjs_re = QjsRegex::new(pattern).unwrap();
    let regress_re = RegressRegex::new(pattern).unwrap();

    // 1. Hybrid (optimizations + Rust interpreter)
    group.bench_with_input(
        BenchmarkId::new("hybrid", name),
        &input,
        |b, i| b.iter(|| qjs_re.find(black_box(*i))),
    );

    // 2. Pure Rust interpreter (no search optimizations)
    group.bench_with_input(
        BenchmarkId::new("pure_rust", name),
        &input,
        |b, i| b.iter(|| qjs_re.find_at(black_box(*i), 0)),
    );

    // 3. C Engine (original c2rust transpiled engine)
    group.bench_with_input(
        BenchmarkId::new("c_engine", name),
        &input,
        |b, i| b.iter(|| qjs_re.find_at_c_engine(black_box(*i), 0)),
    );

    // 4. Regress (pure Rust JS regex crate)
    group.bench_with_input(
        BenchmarkId::new("regress", name),
        &input,
        |b, i| b.iter(|| regress_re.find(black_box(*i))),
    );
}

// ============================================================================
// Basic matching benchmarks
// ============================================================================

fn bench_basic(c: &mut Criterion) {
    let mut group = c.benchmark_group("basic");

    let cases = [
        ("literal", "hello", "hello world"),
        ("digits", r"\d+", "abc123def"),
        ("word", r"\w+", "hello world"),
        ("alternation", "cat|dog", "the lazy dog"),
    ];

    for (name, pattern, input) in cases {
        bench_four_engines(&mut group, name, pattern, input);
    }

    group.finish();
}

// ============================================================================
// Character class benchmarks
// ============================================================================

fn bench_char_class(c: &mut Criterion) {
    let mut group = c.benchmark_group("char_class");

    let input = "The Quick Brown Fox 123 Jumped!";

    let patterns = [
        ("word_class", r"\w+"),
        ("digit_class", r"\d+"),
        ("range", r"[a-zA-Z]+"),
        ("whitespace", r"\s+"),
    ];

    for (name, pattern) in patterns {
        bench_four_engines(&mut group, name, pattern, input);
    }

    group.finish();
}

// ============================================================================
// Quantifier benchmarks
// ============================================================================

fn bench_quantifiers(c: &mut Criterion) {
    let mut group = c.benchmark_group("quantifiers");

    // Greedy quantifiers on varying input sizes
    let inputs = [
        ("greedy_10", "a".repeat(10)),
        ("greedy_50", "a".repeat(50)),
        ("greedy_100", "a".repeat(100)),
    ];

    let qjs_re = QjsRegex::new(r"a+").unwrap();
    let regress_re = RegressRegex::new(r"a+").unwrap();

    for (name, input) in &inputs {
        group.bench_with_input(
            BenchmarkId::new("hybrid", name),
            input.as_str(),
            |b, i| b.iter(|| qjs_re.find(black_box(i))),
        );
        group.bench_with_input(
            BenchmarkId::new("pure_rust", name),
            input.as_str(),
            |b, i| b.iter(|| qjs_re.find_at(black_box(i), 0)),
        );
        group.bench_with_input(
            BenchmarkId::new("c_engine", name),
            input.as_str(),
            |b, i| b.iter(|| qjs_re.find_at_c_engine(black_box(i), 0)),
        );
        group.bench_with_input(
            BenchmarkId::new("regress", name),
            input.as_str(),
            |b, i| b.iter(|| regress_re.find(black_box(i))),
        );
    }

    group.finish();
}

// ============================================================================
// Late match benchmarks (match near end of input)
// ============================================================================

fn bench_late_match(c: &mut Criterion) {
    let mut group = c.benchmark_group("late_match");

    let inputs = [
        ("late_100", "x".repeat(100) + "needle"),
        ("late_1000", "x".repeat(1000) + "needle"),
        ("late_10000", "x".repeat(10000) + "needle"),
    ];

    let qjs_re = QjsRegex::new("needle").unwrap();
    let regress_re = RegressRegex::new("needle").unwrap();

    for (name, input) in &inputs {
        group.bench_with_input(
            BenchmarkId::new("hybrid", name),
            input.as_str(),
            |b, i| b.iter(|| qjs_re.find(black_box(i))),
        );
        group.bench_with_input(
            BenchmarkId::new("pure_rust", name),
            input.as_str(),
            |b, i| b.iter(|| qjs_re.find_at(black_box(i), 0)),
        );
        group.bench_with_input(
            BenchmarkId::new("c_engine", name),
            input.as_str(),
            |b, i| b.iter(|| qjs_re.find_at_c_engine(black_box(i), 0)),
        );
        group.bench_with_input(
            BenchmarkId::new("regress", name),
            input.as_str(),
            |b, i| b.iter(|| regress_re.find(black_box(i))),
        );
    }

    group.finish();
}

// ============================================================================
// No match benchmarks (worst case - scan entire input)
// ============================================================================

fn bench_no_match(c: &mut Criterion) {
    let mut group = c.benchmark_group("no_match");

    let inputs = [
        ("no_match_100", "x".repeat(100)),
        ("no_match_1000", "x".repeat(1000)),
        ("no_match_10000", "x".repeat(10000)),
    ];

    let qjs_re = QjsRegex::new("needle").unwrap();
    let regress_re = RegressRegex::new("needle").unwrap();

    for (name, input) in &inputs {
        group.bench_with_input(
            BenchmarkId::new("hybrid", name),
            input.as_str(),
            |b, i| b.iter(|| qjs_re.find(black_box(i))),
        );
        group.bench_with_input(
            BenchmarkId::new("pure_rust", name),
            input.as_str(),
            |b, i| b.iter(|| qjs_re.find_at(black_box(i), 0)),
        );
        group.bench_with_input(
            BenchmarkId::new("c_engine", name),
            input.as_str(),
            |b, i| b.iter(|| qjs_re.find_at_c_engine(black_box(i), 0)),
        );
        group.bench_with_input(
            BenchmarkId::new("regress", name),
            input.as_str(),
            |b, i| b.iter(|| regress_re.find(black_box(i))),
        );
    }

    group.finish();
}

// ============================================================================
// Backtracking benchmarks
// ============================================================================

fn bench_backtracking(c: &mut Criterion) {
    let mut group = c.benchmark_group("backtracking");

    // Classic backtracking pattern: a?^n a^n on input a^n
    let cases = [
        ("bt_5", r"a?a?a?a?a?aaaaa", "aaaaa"),
        ("bt_10", r"a?a?a?a?a?a?a?a?a?a?aaaaaaaaaa", "aaaaaaaaaa"),
    ];

    for (name, pattern, input) in cases {
        bench_four_engines(&mut group, name, pattern, input);
    }

    group.finish();
}

// ============================================================================
// Complex pattern benchmarks
// ============================================================================

fn bench_complex(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex");

    let cases = [
        ("email", r"\w+@\w+\.\w+", "contact foo@bar.com today"),
        ("url", r"https?://[\w./]+", "visit https://example.com/path today"),
        ("word_boundary", r"\bword\b", "find the word in this text"),
        ("lookahead_like", r"foo(?=bar)", "foobar foobaz"),
    ];

    for (name, pattern, input) in cases {
        bench_four_engines(&mut group, name, pattern, input);
    }

    group.finish();
}

// ============================================================================
// Real-world pattern benchmarks
// ============================================================================

fn bench_realistic(c: &mut Criterion) {
    let mut group = c.benchmark_group("realistic");

    // Log line parsing
    let log_pattern = r"\[(\d{4}-\d{2}-\d{2})\s+(\d{2}:\d{2}:\d{2})\]\s+(\w+):";
    let log_input = "[2024-01-15 14:30:45] ERROR: Connection timeout";
    bench_four_engines(&mut group, "log_parse", log_pattern, log_input);

    // JSON key extraction (simplified)
    let json_pattern = r#""(\w+)"\s*:\s*"([^"]*)""#;
    let json_input = r#"{"name": "John", "age": "30", "city": "NYC"}"#;
    bench_four_engines(&mut group, "json_key", json_pattern, json_input);

    // IP address
    let ip_pattern = r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}";
    let ip_input = "Server IP is 192.168.1.100 on port 8080";
    bench_four_engines(&mut group, "ip_address", ip_pattern, ip_input);

    group.finish();
}

// ============================================================================
// Find all matches benchmarks
// ============================================================================

fn bench_find_iter(c: &mut Criterion) {
    let mut group = c.benchmark_group("find_iter");

    let text = "Item 1: $10, Item 2: $25, Item 3: $100, Item 4: $50, Item 5: $75, Item 6: $200";

    let qjs_re = QjsRegex::new(r"\d+").unwrap();
    let regress_re = RegressRegex::new(r"\d+").unwrap();

    group.bench_function("hybrid/numbers", |b| {
        b.iter(|| qjs_re.find_iter(black_box(text)).count())
    });

    // For pure_rust and c_engine, we need to manually iterate
    group.bench_function("pure_rust/numbers", |b| {
        b.iter(|| {
            let mut count = 0;
            let mut pos = 0;
            let input = black_box(text);
            while pos < input.len() {
                if let Some(m) = qjs_re.find_at(input, pos) {
                    count += 1;
                    pos = if m.end > pos { m.end } else { pos + 1 };
                } else {
                    break;
                }
            }
            count
        })
    });

    group.bench_function("c_engine/numbers", |b| {
        b.iter(|| {
            let mut count = 0;
            let mut pos = 0;
            let input = black_box(text);
            while pos < input.len() {
                if let Some(m) = qjs_re.find_at_c_engine(input, pos) {
                    count += 1;
                    pos = if m.end > pos { m.end } else { pos + 1 };
                } else {
                    break;
                }
            }
            count
        })
    });

    group.bench_function("regress/numbers", |b| {
        b.iter(|| regress_re.find_iter(black_box(text)).count())
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_basic,
    bench_char_class,
    bench_quantifiers,
    bench_late_match,
    bench_no_match,
    bench_backtracking,
    bench_complex,
    bench_realistic,
    bench_find_iter,
);

criterion_main!(benches);
