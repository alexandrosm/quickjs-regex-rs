use quickjs_regex::Regex;
use std::error::Error;
use std::fs;
use std::time::Instant;

fn parse_patterns_from_nosey_full_test() -> Vec<String> {
    let src = include_str!("../tests/nosey_full.rs");
    let mut in_patterns = false;
    let mut out = Vec::new();

    for line in src.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("const PATTERNS") {
            in_patterns = true;
            continue;
        }
        if !in_patterns {
            continue;
        }
        if trimmed.starts_with("];") {
            break;
        }
        if let Some(pat) = parse_raw_pattern_line(trimmed) {
            out.push(pat);
        }
    }

    out
}

fn parse_raw_pattern_line(line: &str) -> Option<String> {
    // Parse lines like: r#"..."#,
    if !line.starts_with("r#\"") {
        return None;
    }
    let rest = &line[3..];
    let end = rest.find("\"#")?;
    Some(rest[..end].to_string())
}

fn parse_patterns_from_file(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let patterns = text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| line.to_string())
        .collect::<Vec<_>>();
    Ok(patterns)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut pattern_file: Option<String> = None;
    let mut haystack_file: Option<String> = None;
    let mut per_pattern = false;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--patterns" => {
                let val = args.next().ok_or("missing value for --patterns")?;
                pattern_file = Some(val);
            }
            "--haystack" => {
                let val = args.next().ok_or("missing value for --haystack")?;
                haystack_file = Some(val);
            }
            "--per-pattern" => {
                per_pattern = true;
            }
            "--help" | "-h" => {
                println!("Usage:");
                println!("  cargo run --release --example check_noseyparker_lits");
                println!("  cargo run --release --example check_noseyparker_lits -- --patterns <file>");
                println!("  cargo run --release --example check_noseyparker_lits -- --patterns <file> --haystack <file>");
                println!("  cargo run --release --example check_noseyparker_lits -- --patterns <file> --haystack <file> --per-pattern");
                return Ok(());
            }
            _ => return Err(format!("unknown argument: {arg}").into()),
        }
    }

    let patterns = if let Some(path) = pattern_file.as_deref() {
        parse_patterns_from_file(path)?
    } else {
        parse_patterns_from_nosey_full_test()
    };

    if patterns.is_empty() {
        return Err("no patterns found".into());
    }

    let combined = patterns.join("|");
    let re = Regex::new(&combined)?;

    println!("patterns={}", patterns.len());
    println!("combined_len={}", combined.len());
    println!("debug={}", re.debug_info());

    if let Some(cov) = re.decomposition_coverage() {
        let uncovered = cov.uncovered_patterns.len();
        println!(
            "decomposition: sub_patterns={} ac_literals={} covered={} uncovered={}",
            cov.total_sub_patterns,
            cov.ac_literal_count,
            cov.covered_sub_patterns,
            uncovered
        );

        if uncovered > 0 {
            println!("uncovered sub-patterns:");
            for (i, pat) in cov.uncovered_patterns.iter().enumerate() {
                println!("  {}: {}", i, pat);
            }
        }
    } else {
        println!("decomposition: disabled (no sub-pattern decomposition)");
    }

    if let Some(path) = haystack_file.as_deref() {
        let hay_bytes = fs::read(path)?;
        let hay = String::from_utf8_lossy(&hay_bytes);

        if per_pattern {
            let mut nonzero = Vec::new();
            for (i, pat) in patterns.iter().enumerate() {
                match Regex::new(pat) {
                    Ok(sub) => {
                        let c = sub.count_matches(&hay);
                        if c > 0 {
                            nonzero.push((i, c, pat.clone()));
                        }
                    }
                    Err(err) => {
                        println!("pattern_compile_error index={} err={}", i, err);
                    }
                }
            }
            println!("per_pattern_nonzero={}", nonzero.len());
            for (i, c, pat) in nonzero.iter().take(20) {
                println!("  idx={} count={} pat={}", i, c, pat);
            }
        }

        let start = Instant::now();
        let count = re.count_matches(&hay);
        let elapsed = start.elapsed();
        let ns_per_byte = elapsed.as_nanos() as f64 / hay_bytes.len().max(1) as f64;

        println!("haystack_bytes={}", hay_bytes.len());
        println!("count={}", count);
        println!("elapsed={:?}", elapsed);
        println!("ns_per_byte={:.1}", ns_per_byte);
    }

    Ok(())
}
