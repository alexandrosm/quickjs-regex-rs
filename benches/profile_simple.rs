//! Focused micro-benchmark for profiling simple pattern matching
//!
//! Run with: cargo bench --bench profile_simple

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use quickjs_regex::Regex;

fn profile_literal_hit(c: &mut Criterion) {
    let re = Regex::new("hello").unwrap();
    let input = "hello world";

    c.bench_function("literal_hit", |b| {
        b.iter(|| re.is_match(black_box(input)))
    });
}

fn profile_literal_miss(c: &mut Criterion) {
    let re = Regex::new("xyz").unwrap();
    let input = "hello world";

    c.bench_function("literal_miss", |b| {
        b.iter(|| re.is_match(black_box(input)))
    });
}

fn profile_digit_match(c: &mut Criterion) {
    let re = Regex::new(r"\d+").unwrap();
    let input = "abc123def";

    c.bench_function("digit_match", |b| {
        b.iter(|| re.is_match(black_box(input)))
    });
}

fn profile_char_class(c: &mut Criterion) {
    let re = Regex::new(r"[a-z]+").unwrap();
    let input = "hello";

    c.bench_function("char_class", |b| {
        b.iter(|| re.is_match(black_box(input)))
    });
}

criterion_group!(
    profiling,
    profile_literal_hit,
    profile_literal_miss,
    profile_digit_match,
    profile_char_class,
);

criterion_main!(profiling);
