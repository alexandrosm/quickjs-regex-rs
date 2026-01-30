//! Benchmark comparing Pure Rust interpreter vs Original C engine
//!
//! This compares the performance of:
//! - Pure Rust interpreter (interpreter.rs)
//! - Original C engine (engine.rs lre_exec)

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use quickjs_regex::Regex;

fn bench_is_match(c: &mut Criterion) {
    let mut group = c.benchmark_group("is_match");

    let cases = [
        ("literal", "hello", "hello world"),
        ("digits", r"\d+", "abc123def"),
        ("email", r"\w+@\w+\.\w+", "contact foo@bar.com today"),
        ("word_boundary", r"\bword\b", "find the word in text"),
    ];

    for (name, pattern, input) in cases {
        let re = Regex::new(pattern).unwrap();

        // Rust interpreter (via is_match which uses find_at internally)
        group.bench_with_input(
            BenchmarkId::new("rust", name),
            &input,
            |b, i| {
                b.iter(|| re.is_match(black_box(*i)))
            },
        );

        // C engine
        group.bench_with_input(
            BenchmarkId::new("c_engine", name),
            &input,
            |b, i| {
                b.iter(|| re.find_at_c_engine(black_box(*i), 0).is_some())
            },
        );
    }

    group.finish();
}

fn bench_find(c: &mut Criterion) {
    let mut group = c.benchmark_group("find");

    let late_input = "a".repeat(100) + "xyz";

    let cases: Vec<(&str, &str, String)> = vec![
        ("first_match", r"\d+", "abc123def".to_string()),
        ("late_match", "xyz", late_input),
        ("alternation", "cat|dog", "the lazy dog".to_string()),
    ];

    for (name, pattern, input) in &cases {
        let re = Regex::new(pattern).unwrap();

        // Rust interpreter
        group.bench_with_input(
            BenchmarkId::new("rust", name),
            input.as_str(),
            |b, i| {
                b.iter(|| re.find(black_box(i)))
            },
        );

        // C engine
        group.bench_with_input(
            BenchmarkId::new("c_engine", name),
            input.as_str(),
            |b, i| {
                b.iter(|| re.find_at_c_engine(black_box(i), 0))
            },
        );
    }

    group.finish();
}

fn bench_greedy(c: &mut Criterion) {
    let mut group = c.benchmark_group("greedy");

    let inputs = [
        ("10", "a".repeat(10)),
        ("50", "a".repeat(50)),
        ("100", "a".repeat(100)),
    ];

    let re = Regex::new(r"a+").unwrap();

    for (name, input) in &inputs {
        // Rust interpreter
        group.bench_with_input(
            BenchmarkId::new("rust", name),
            input.as_str(),
            |b, i| {
                b.iter(|| re.find(black_box(i)))
            },
        );

        // C engine
        group.bench_with_input(
            BenchmarkId::new("c_engine", name),
            input.as_str(),
            |b, i| {
                b.iter(|| re.find_at_c_engine(black_box(i), 0))
            },
        );
    }

    group.finish();
}

fn bench_char_class(c: &mut Criterion) {
    let mut group = c.benchmark_group("char_class");

    let input = "The Quick Brown Fox 123";

    let patterns = [
        ("word", r"\w+"),
        ("digit", r"\d+"),
        ("range", r"[a-zA-Z]+"),
    ];

    for (name, pattern) in patterns {
        let re = Regex::new(pattern).unwrap();

        // Rust interpreter
        group.bench_with_input(
            BenchmarkId::new("rust", name),
            &input,
            |b, i| {
                b.iter(|| re.find(black_box(*i)))
            },
        );

        // C engine
        group.bench_with_input(
            BenchmarkId::new("c_engine", name),
            &input,
            |b, i| {
                b.iter(|| re.find_at_c_engine(black_box(*i), 0))
            },
        );
    }

    group.finish();
}

fn bench_backtracking(c: &mut Criterion) {
    let mut group = c.benchmark_group("backtracking");

    // Pattern that requires significant backtracking
    let re = Regex::new(r"a?a?a?a?a?aaaaa").unwrap();
    let input = "aaaaa";

    group.bench_function("rust", |b| {
        b.iter(|| re.is_match(black_box(input)))
    });

    group.bench_function("c_engine", |b| {
        b.iter(|| re.find_at_c_engine(black_box(input), 0).is_some())
    });

    group.finish();
}

fn bench_long_input(c: &mut Criterion) {
    let mut group = c.benchmark_group("long_input");

    let long_text = "b".repeat(9990) + "aaaaaaaaaa";
    let re = Regex::new(r"a+").unwrap();

    group.bench_function("rust/match_at_end", |b| {
        b.iter(|| re.find(black_box(&long_text)))
    });

    group.bench_function("c_engine/match_at_end", |b| {
        b.iter(|| re.find_at_c_engine(black_box(&long_text), 0))
    });

    // No match
    let no_match_text = "b".repeat(10000);
    let literal_re = Regex::new("xyz").unwrap();

    group.bench_function("rust/no_match", |b| {
        b.iter(|| literal_re.find(black_box(&no_match_text)))
    });

    group.bench_function("c_engine/no_match", |b| {
        b.iter(|| literal_re.find_at_c_engine(black_box(&no_match_text), 0))
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_is_match,
    bench_find,
    bench_greedy,
    bench_char_class,
    bench_backtracking,
    bench_long_input,
);

criterion_main!(benches);
