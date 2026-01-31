#!/usr/bin/env rust-script
//! Safety analyzer for quickjs-regex-rs
//! Run with: rust-script scripts/safety_analyze.rs
//! Or: cargo install rust-script && rust-script scripts/safety_analyze.rs

use std::fs;
use std::path::Path;

#[derive(Default, Debug)]
struct FileStats {
    safe_pure: Vec<String>,      // Safe fn, no unsafe blocks
    safe_wrapper: Vec<String>,   // Safe fn, contains unsafe blocks
    unsafe_fn: Vec<String>,      // unsafe fn (not extern C)
    extern_c: Vec<String>,       // unsafe extern "C" fn
    lines: usize,
}

fn analyze_file(path: &Path) -> FileStats {
    let content = fs::read_to_string(path).unwrap_or_default();
    let lines: Vec<&str> = content.lines().collect();
    let mut stats = FileStats {
        lines: lines.len(),
        ..Default::default()
    };

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();

        // Check for function declaration
        if let Some(fn_info) = parse_fn_declaration(line) {
            let fn_name = fn_info.name.clone();
            let body = extract_fn_body(&lines, i);
            let has_unsafe_block = body.contains("unsafe {") || body.contains("unsafe{");

            match fn_info.kind {
                FnKind::Safe => {
                    if has_unsafe_block {
                        stats.safe_wrapper.push(fn_name);
                    } else {
                        stats.safe_pure.push(fn_name);
                    }
                }
                FnKind::Unsafe => {
                    stats.unsafe_fn.push(fn_name);
                }
                FnKind::ExternC => {
                    stats.extern_c.push(fn_name);
                }
            }
        }
        i += 1;
    }

    stats
}

#[derive(Debug)]
enum FnKind {
    Safe,
    Unsafe,
    ExternC,
}

struct FnInfo {
    name: String,
    kind: FnKind,
}

fn parse_fn_declaration(line: &str) -> Option<FnInfo> {
    let line = line.trim();

    // Skip if not a function declaration at start of line
    if !line.starts_with("pub ") && !line.starts_with("fn ")
        && !line.starts_with("unsafe ") && !line.starts_with("#[") {
        return None;
    }

    // Skip attributes, impl blocks, etc
    if line.starts_with("#[") || line.starts_with("impl ") || line.starts_with("type ") {
        return None;
    }

    // Determine function kind
    let kind = if line.contains("unsafe extern \"C\" fn ") || line.contains("unsafe extern \"C\" fn(") {
        FnKind::ExternC
    } else if line.contains("unsafe fn ") || line.contains("unsafe fn(") {
        FnKind::Unsafe
    } else if line.contains(" fn ") || line.starts_with("fn ") {
        FnKind::Safe
    } else {
        return None;
    };

    // Extract function name
    let fn_idx = line.find(" fn ")?;
    let after_fn = &line[fn_idx + 4..];
    let name_end = after_fn.find(|c: char| c == '(' || c == '<' || c.is_whitespace())?;
    let name = after_fn[..name_end].to_string();

    if name.is_empty() {
        return None;
    }

    Some(FnInfo { name, kind })
}

fn extract_fn_body(lines: &[&str], start: usize) -> String {
    let mut body = String::new();
    let mut brace_count = 0;
    let mut started = false;

    for i in start..lines.len().min(start + 500) {
        let line = lines[i];

        for c in line.chars() {
            if c == '{' {
                brace_count += 1;
                started = true;
            } else if c == '}' {
                brace_count -= 1;
            }
        }

        body.push_str(line);
        body.push('\n');

        if started && brace_count == 0 {
            break;
        }
    }

    body
}

fn main() {
    println!("# Safety Metrics Report");
    println!("Generated: {}", chrono_lite());
    println!();

    let files = [
        "src/regex/util.rs",
        "src/regex/engine.rs",
        "src/regex/unicode.rs",
    ];

    let mut all_stats: Vec<(&str, FileStats)> = Vec::new();

    for file in &files {
        let path = Path::new(file);
        if path.exists() {
            let stats = analyze_file(path);
            all_stats.push((file, stats));
        }
    }

    // Summary table
    println!("## Summary Table");
    println!("```");
    println!("| {:12} | {:6} | {:8} | {:6} | {:8} | {:6} |",
             "File", "Pure", "Wrappers", "Unsafe", "extern C", "Lines");
    println!("|--------------|--------|----------|--------|----------|--------|");

    let mut totals = (0usize, 0usize, 0usize, 0usize, 0usize);

    for (file, stats) in &all_stats {
        let basename = Path::new(file).file_name().unwrap().to_str().unwrap();
        println!("| {:12} | {:6} | {:8} | {:6} | {:8} | {:6} |",
                 basename,
                 stats.safe_pure.len(),
                 stats.safe_wrapper.len(),
                 stats.unsafe_fn.len(),
                 stats.extern_c.len(),
                 stats.lines);

        totals.0 += stats.safe_pure.len();
        totals.1 += stats.safe_wrapper.len();
        totals.2 += stats.unsafe_fn.len();
        totals.3 += stats.extern_c.len();
        totals.4 += stats.lines;
    }

    println!("|--------------|--------|----------|--------|----------|--------|");
    println!("| {:12} | {:6} | {:8} | {:6} | {:8} | {:6} |",
             "TOTAL", totals.0, totals.1, totals.2, totals.3, totals.4);
    println!("```");

    println!();
    println!("## Legend");
    println!("- **Pure**: Safe functions with no `unsafe` blocks");
    println!("- **Wrappers**: Safe API functions that internally use `unsafe`");
    println!("- **Unsafe**: Functions marked `unsafe fn`");
    println!("- **extern C**: Functions with C ABI (callbacks)");

    // Detailed listings
    println!();
    println!("## Details");

    for (file, stats) in &all_stats {
        let basename = Path::new(file).file_name().unwrap().to_str().unwrap();
        println!();
        println!("### {}", basename);

        if !stats.extern_c.is_empty() {
            println!();
            println!("**extern \"C\" (callbacks):** {}", stats.extern_c.join(", "));
        }

        if !stats.safe_wrapper.is_empty() {
            println!();
            println!("**Safe wrappers ({}):** {}",
                     stats.safe_wrapper.len(),
                     if stats.safe_wrapper.len() <= 10 {
                         stats.safe_wrapper.join(", ")
                     } else {
                         format!("{}... and {} more",
                                 stats.safe_wrapper[..10].join(", "),
                                 stats.safe_wrapper.len() - 10)
                     });
        }
    }

    // Metrics
    println!();
    println!("## Metrics");
    let total_fns = totals.0 + totals.1 + totals.2 + totals.3;
    let truly_safe = totals.0;
    let safe_api = totals.0 + totals.1;

    if total_fns > 0 {
        println!("- **Truly safe (no unsafe):** {} / {} ({}%)",
                 truly_safe, total_fns, truly_safe * 100 / total_fns);
        println!("- **Safe API (safe signature):** {} / {} ({}%)",
                 safe_api, total_fns, safe_api * 100 / total_fns);
        println!("- **Pure Rust (no extern C):** {} / {} ({}%)",
                 total_fns - totals.3, total_fns, (total_fns - totals.3) * 100 / total_fns);
    }
    println!("- **Total lines:** {}", totals.4);
}

fn chrono_lite() -> String {
    // Simple timestamp without external deps
    "see bash script for timestamp".to_string()
}
