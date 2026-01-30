//! Interleaved benchmark for fair comparison across engines
//!
//! Each test case runs all 4 engines consecutively to minimize
//! the impact of thermal throttling and system load variations.
//!
//! Run with: cargo bench --bench interleaved_comparison

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use quickjs_regex::Regex as QjsRegex;
use regress::Regex as RegressRegex;

// Helper macro to run all 4 engines for a single test case
macro_rules! bench_all_engines {
    ($group:expr, $name:expr, $qjs:expr, $regress:expr, $input:expr) => {
        // Run engines in interleaved order within the same benchmark group
        // Criterion will interleave samples across these benchmarks
        $group.bench_with_input(
            BenchmarkId::new("1_hybrid", $name),
            &$input,
            |b, i| b.iter(|| $qjs.find(black_box(*i))),
        );
        $group.bench_with_input(
            BenchmarkId::new("2_pure_rust", $name),
            &$input,
            |b, i| b.iter(|| $qjs.find_at(black_box(*i), 0)),
        );
        $group.bench_with_input(
            BenchmarkId::new("3_c_engine", $name),
            &$input,
            |b, i| b.iter(|| $qjs.find_at_c_engine(black_box(*i), 0)),
        );
        $group.bench_with_input(
            BenchmarkId::new("4_regress", $name),
            &$input,
            |b, i| b.iter(|| $regress.find(black_box(*i))),
        );
    };
}

fn bench_basic_interleaved(c: &mut Criterion) {
    let mut group = c.benchmark_group("basic_interleaved");

    // Test 1: literal
    {
        let qjs = QjsRegex::new("hello").unwrap();
        let regress = RegressRegex::new("hello").unwrap();
        let input = "hello world";
        bench_all_engines!(group, "literal", qjs, regress, input);
    }

    // Test 2: digits
    {
        let qjs = QjsRegex::new(r"\d+").unwrap();
        let regress = RegressRegex::new(r"\d+").unwrap();
        let input = "abc123def";
        bench_all_engines!(group, "digits", qjs, regress, input);
    }

    // Test 3: word
    {
        let qjs = QjsRegex::new(r"\w+").unwrap();
        let regress = RegressRegex::new(r"\w+").unwrap();
        let input = "hello world";
        bench_all_engines!(group, "word", qjs, regress, input);
    }

    // Test 4: alternation
    {
        let qjs = QjsRegex::new("cat|dog").unwrap();
        let regress = RegressRegex::new("cat|dog").unwrap();
        let input = "the lazy dog";
        bench_all_engines!(group, "alternation", qjs, regress, input);
    }

    group.finish();
}

fn bench_late_match_interleaved(c: &mut Criterion) {
    let mut group = c.benchmark_group("late_match_interleaved");

    let qjs = QjsRegex::new("needle").unwrap();
    let regress = RegressRegex::new("needle").unwrap();

    // 100 chars
    {
        let input: String = "x".repeat(100) + "needle";
        let input_ref = input.as_str();
        group.bench_function("1_hybrid/100", |b| b.iter(|| qjs.find(black_box(input_ref))));
        group.bench_function("2_pure_rust/100", |b| b.iter(|| qjs.find_at(black_box(input_ref), 0)));
        group.bench_function("3_c_engine/100", |b| b.iter(|| qjs.find_at_c_engine(black_box(input_ref), 0)));
        group.bench_function("4_regress/100", |b| b.iter(|| regress.find(black_box(input_ref))));
    }

    // 1000 chars
    {
        let input: String = "x".repeat(1000) + "needle";
        let input_ref = input.as_str();
        group.bench_function("1_hybrid/1000", |b| b.iter(|| qjs.find(black_box(input_ref))));
        group.bench_function("2_pure_rust/1000", |b| b.iter(|| qjs.find_at(black_box(input_ref), 0)));
        group.bench_function("3_c_engine/1000", |b| b.iter(|| qjs.find_at_c_engine(black_box(input_ref), 0)));
        group.bench_function("4_regress/1000", |b| b.iter(|| regress.find(black_box(input_ref))));
    }

    // 10000 chars
    {
        let input: String = "x".repeat(10000) + "needle";
        let input_ref = input.as_str();
        group.bench_function("1_hybrid/10000", |b| b.iter(|| qjs.find(black_box(input_ref))));
        group.bench_function("2_pure_rust/10000", |b| b.iter(|| qjs.find_at(black_box(input_ref), 0)));
        group.bench_function("3_c_engine/10000", |b| b.iter(|| qjs.find_at_c_engine(black_box(input_ref), 0)));
        group.bench_function("4_regress/10000", |b| b.iter(|| regress.find(black_box(input_ref))));
    }

    group.finish();
}

fn bench_no_match_interleaved(c: &mut Criterion) {
    let mut group = c.benchmark_group("no_match_interleaved");

    let qjs = QjsRegex::new("needle").unwrap();
    let regress = RegressRegex::new("needle").unwrap();

    // 100 chars
    {
        let input: String = "x".repeat(100);
        let input_ref = input.as_str();
        group.bench_function("1_hybrid/100", |b| b.iter(|| qjs.find(black_box(input_ref))));
        group.bench_function("2_pure_rust/100", |b| b.iter(|| qjs.find_at(black_box(input_ref), 0)));
        group.bench_function("3_c_engine/100", |b| b.iter(|| qjs.find_at_c_engine(black_box(input_ref), 0)));
        group.bench_function("4_regress/100", |b| b.iter(|| regress.find(black_box(input_ref))));
    }

    // 1000 chars
    {
        let input: String = "x".repeat(1000);
        let input_ref = input.as_str();
        group.bench_function("1_hybrid/1000", |b| b.iter(|| qjs.find(black_box(input_ref))));
        group.bench_function("2_pure_rust/1000", |b| b.iter(|| qjs.find_at(black_box(input_ref), 0)));
        group.bench_function("3_c_engine/1000", |b| b.iter(|| qjs.find_at_c_engine(black_box(input_ref), 0)));
        group.bench_function("4_regress/1000", |b| b.iter(|| regress.find(black_box(input_ref))));
    }

    // 10000 chars
    {
        let input: String = "x".repeat(10000);
        let input_ref = input.as_str();
        group.bench_function("1_hybrid/10000", |b| b.iter(|| qjs.find(black_box(input_ref))));
        group.bench_function("2_pure_rust/10000", |b| b.iter(|| qjs.find_at(black_box(input_ref), 0)));
        group.bench_function("3_c_engine/10000", |b| b.iter(|| qjs.find_at_c_engine(black_box(input_ref), 0)));
        group.bench_function("4_regress/10000", |b| b.iter(|| regress.find(black_box(input_ref))));
    }

    group.finish();
}

fn bench_greedy_interleaved(c: &mut Criterion) {
    let mut group = c.benchmark_group("greedy_interleaved");

    let qjs = QjsRegex::new(r"a+").unwrap();
    let regress = RegressRegex::new(r"a+").unwrap();

    for size in [10, 50, 100] {
        let input: String = "a".repeat(size);
        let input_ref = input.as_str();
        let label = format!("{}", size);

        group.bench_function(format!("1_hybrid/{}", label), |b| b.iter(|| qjs.find(black_box(input_ref))));
        group.bench_function(format!("2_pure_rust/{}", label), |b| b.iter(|| qjs.find_at(black_box(input_ref), 0)));
        group.bench_function(format!("3_c_engine/{}", label), |b| b.iter(|| qjs.find_at_c_engine(black_box(input_ref), 0)));
        group.bench_function(format!("4_regress/{}", label), |b| b.iter(|| regress.find(black_box(input_ref))));
    }

    group.finish();
}

fn bench_complex_interleaved(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex_interleaved");

    // Email
    {
        let qjs = QjsRegex::new(r"\w+@\w+\.\w+").unwrap();
        let regress = RegressRegex::new(r"\w+@\w+\.\w+").unwrap();
        let input = "contact foo@bar.com today";
        bench_all_engines!(group, "email", qjs, regress, input);
    }

    // URL
    {
        let qjs = QjsRegex::new(r"https?://[\w./]+").unwrap();
        let regress = RegressRegex::new(r"https?://[\w./]+").unwrap();
        let input = "visit https://example.com/path today";
        bench_all_engines!(group, "url", qjs, regress, input);
    }

    // Word boundary
    {
        let qjs = QjsRegex::new(r"\bword\b").unwrap();
        let regress = RegressRegex::new(r"\bword\b").unwrap();
        let input = "find the word in this text";
        bench_all_engines!(group, "word_boundary", qjs, regress, input);
    }

    // Lookahead
    {
        let qjs = QjsRegex::new(r"foo(?=bar)").unwrap();
        let regress = RegressRegex::new(r"foo(?=bar)").unwrap();
        let input = "foobar foobaz";
        bench_all_engines!(group, "lookahead", qjs, regress, input);
    }

    group.finish();
}

fn bench_backtracking_interleaved(c: &mut Criterion) {
    let mut group = c.benchmark_group("backtracking_interleaved");

    // bt_5: a?a?a?a?a?aaaaa on "aaaaa"
    {
        let qjs = QjsRegex::new(r"a?a?a?a?a?aaaaa").unwrap();
        let regress = RegressRegex::new(r"a?a?a?a?a?aaaaa").unwrap();
        let input = "aaaaa";
        bench_all_engines!(group, "bt_5", qjs, regress, input);
    }

    // bt_10
    {
        let qjs = QjsRegex::new(r"a?a?a?a?a?a?a?a?a?a?aaaaaaaaaa").unwrap();
        let regress = RegressRegex::new(r"a?a?a?a?a?a?a?a?a?a?aaaaaaaaaa").unwrap();
        let input = "aaaaaaaaaa";
        bench_all_engines!(group, "bt_10", qjs, regress, input);
    }

    group.finish();
}

fn bench_realistic_interleaved(c: &mut Criterion) {
    let mut group = c.benchmark_group("realistic_interleaved");

    // Log parsing
    {
        let qjs = QjsRegex::new(r"\[(\d{4}-\d{2}-\d{2})\s+(\d{2}:\d{2}:\d{2})\]\s+(\w+):").unwrap();
        let regress = RegressRegex::new(r"\[(\d{4}-\d{2}-\d{2})\s+(\d{2}:\d{2}:\d{2})\]\s+(\w+):").unwrap();
        let input = "[2024-01-15 14:30:45] ERROR: Connection timeout";
        bench_all_engines!(group, "log_parse", qjs, regress, input);
    }

    // JSON key
    {
        let qjs = QjsRegex::new(r#""(\w+)"\s*:\s*"([^"]*)""#).unwrap();
        let regress = RegressRegex::new(r#""(\w+)"\s*:\s*"([^"]*)""#).unwrap();
        let input = r#"{"name": "John", "age": "30"}"#;
        bench_all_engines!(group, "json_key", qjs, regress, input);
    }

    // IP address
    {
        let qjs = QjsRegex::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}").unwrap();
        let regress = RegressRegex::new(r"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}").unwrap();
        let input = "Server IP is 192.168.1.100 on port 8080";
        bench_all_engines!(group, "ip_address", qjs, regress, input);
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_basic_interleaved,
    bench_late_match_interleaved,
    bench_no_match_interleaved,
    bench_greedy_interleaved,
    bench_complex_interleaved,
    bench_backtracking_interleaved,
    bench_realistic_interleaved,
);

criterion_main!(benches);
