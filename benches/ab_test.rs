//! A/B test for CHAR opcode implementations
//!
//! Tests the interpreter with simple vs optimized char matching

use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Simple version - no ASCII fast path
mod simple_char {
    #[inline(always)]
    pub fn next_char(input: &[u8], pos: usize) -> Option<(u32, usize)> {
        if pos >= input.len() {
            return None;
        }
        let b0 = input[pos];
        if b0 < 0x80 {
            return Some((b0 as u32, pos + 1));
        }
        // Multi-byte (simplified)
        if b0 < 0xE0 && pos + 1 < input.len() {
            let b1 = input[pos + 1];
            let cp = ((b0 as u32 & 0x1F) << 6) | (b1 as u32 & 0x3F);
            return Some((cp, pos + 2));
        }
        Some((b0 as u32, pos + 1))
    }

    #[inline(never)]
    pub fn match_chars_simple(input: &[u8], pattern: &[u16]) -> bool {
        let mut pos = 0;
        for &expected in pattern {
            if let Some((c, new_pos)) = next_char(input, pos) {
                if c == expected as u32 {
                    pos = new_pos;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

// Optimized version with ASCII fast path
mod optimized_char {
    #[inline(always)]
    pub fn next_char(input: &[u8], pos: usize, input_len: usize) -> Option<(u32, usize)> {
        if pos >= input_len {
            return None;
        }
        let b0 = unsafe { *input.get_unchecked(pos) };
        if b0 < 0x80 {
            return Some((b0 as u32, pos + 1));
        }
        // Multi-byte
        if b0 < 0xE0 && pos + 1 < input_len {
            let b1 = unsafe { *input.get_unchecked(pos + 1) };
            let cp = ((b0 as u32 & 0x1F) << 6) | (b1 as u32 & 0x3F);
            return Some((cp, pos + 2));
        }
        Some((b0 as u32, pos + 1))
    }

    #[inline(never)]
    pub fn match_chars_optimized(input: &[u8], pattern: &[u16]) -> bool {
        let input_len = input.len();
        let mut pos = 0;

        for &expected in pattern {
            // ASCII fast path
            if expected < 128 && pos < input_len {
                let b = unsafe { *input.get_unchecked(pos) };
                if b == expected as u8 {
                    pos += 1;
                    continue;
                }
                if b < 0x80 {
                    return false; // ASCII mismatch
                }
            }

            // Full path
            if let Some((c, new_pos)) = next_char(input, pos, input_len) {
                if c == expected as u32 {
                    pos = new_pos;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    // Simplified optimized - just bounds check elimination
    #[inline(never)]
    pub fn match_chars_opt_simple(input: &[u8], pattern: &[u16]) -> bool {
        let input_len = input.len();
        let mut pos = 0;

        for &expected in pattern {
            if let Some((c, new_pos)) = next_char(input, pos, input_len) {
                if c == expected as u32 {
                    pos = new_pos;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

fn bench_char_matching(c: &mut Criterion) {
    let input = b"hello world";
    let pattern: Vec<u16> = "hello".chars().map(|c| c as u16).collect();

    let mut group = c.benchmark_group("char_match");

    group.bench_function("simple", |b| {
        b.iter(|| simple_char::match_chars_simple(black_box(input), black_box(&pattern)))
    });

    group.bench_function("optimized_full", |b| {
        b.iter(|| optimized_char::match_chars_optimized(black_box(input), black_box(&pattern)))
    });

    group.bench_function("optimized_simple", |b| {
        b.iter(|| optimized_char::match_chars_opt_simple(black_box(input), black_box(&pattern)))
    });

    group.finish();
}

fn bench_short_pattern(c: &mut Criterion) {
    let input = b"hi";
    let pattern: Vec<u16> = "hi".chars().map(|c| c as u16).collect();

    let mut group = c.benchmark_group("short_pattern");

    group.bench_function("simple", |b| {
        b.iter(|| simple_char::match_chars_simple(black_box(input), black_box(&pattern)))
    });

    group.bench_function("optimized_full", |b| {
        b.iter(|| optimized_char::match_chars_optimized(black_box(input), black_box(&pattern)))
    });

    group.bench_function("optimized_simple", |b| {
        b.iter(|| optimized_char::match_chars_opt_simple(black_box(input), black_box(&pattern)))
    });

    group.finish();
}

criterion_group!(
    ab_tests,
    bench_char_matching,
    bench_short_pattern,
);

criterion_main!(ab_tests);
