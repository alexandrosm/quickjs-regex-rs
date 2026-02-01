use std::io::Write;

use {
    anyhow::Context,
    lexopt::{Arg, ValueExt},
    quickjs_regex::{Flags, Regex},
};

#[derive(Clone, Copy, Debug)]
enum Mode {
    Hybrid,
    PureRust,
    CEngine,
}

fn main() -> anyhow::Result<()> {
    let mut p = lexopt::Parser::from_env();
    let (mut quiet, mut version) = (false, false);
    let mut mode = Mode::Hybrid;
    while let Some(arg) = p.next()? {
        match arg {
            Arg::Short('h') | Arg::Long("help") => {
                anyhow::bail!("main [--version | --quiet | --mode hybrid|pure-rust|c-engine]")
            }
            Arg::Short('q') | Arg::Long("quiet") => {
                quiet = true;
            }
            Arg::Long("version") => {
                version = true;
            }
            Arg::Long("mode") => {
                let val: String = p.value()?.parse()?;
                mode = match val.as_str() {
                    "hybrid" => Mode::Hybrid,
                    "pure-rust" => Mode::PureRust,
                    "c-engine" => Mode::CEngine,
                    _ => anyhow::bail!("unknown mode: {}", val),
                };
                eprintln!("DEBUG: mode set to {:?}", mode);
            }
            _ => return Err(arg.unexpected().into()),
        }
    }
    eprintln!("DEBUG: final mode is {:?}", mode);
    if version {
        writeln!(std::io::stdout(), "{}", env!("CARGO_PKG_VERSION"))?;
        return Ok(());
    }
    let b = klv::Benchmark::read(std::io::stdin())
        .context("failed to read KLV data from <stdin>")?;
    let samples = match b.model.as_str() {
        "compile" => model_compile(&b, mode)?,
        "count" => model_count(&b, &compile(&b)?, mode)?,
        "count-spans" => model_count_spans(&b, &compile(&b)?, mode)?,
        "count-captures" => model_count_captures(&b, &compile(&b)?, mode)?,
        "grep" => model_grep(&b, &compile(&b)?, mode)?,
        "grep-captures" => model_grep_captures(&b, &compile(&b)?, mode)?,
        _ => anyhow::bail!("unrecognized benchmark model '{}'", b.model),
    };
    if !quiet {
        let mut stdout = std::io::stdout().lock();
        for s in samples.iter() {
            writeln!(stdout, "{},{}", s.duration.as_nanos(), s.count)?;
        }
    }
    Ok(())
}

// Track whether we've logged timing info
static LOGGED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn find_all(re: &Regex, haystack: &str, mode: Mode) -> usize {
    let start_time = std::time::Instant::now();
    let mut count = 0;
    let mut pos = 0;
    while pos < haystack.len() {
        let m = match mode {
            Mode::Hybrid => re.find(&haystack[pos..]),
            Mode::PureRust => re.find_at(haystack, pos),
            Mode::CEngine => re.find_at_c_engine(haystack, pos),
        };
        match m {
            Some(m) => {
                count += 1;
                let (start, end) = match mode {
                    Mode::Hybrid => (pos + m.start, pos + m.end),
                    _ => (m.start, m.end),
                };
                pos = if end > start { end } else { start + 1 };
            }
            None => break,
        }
    }
    // Log timing for first run only
    if !LOGGED.swap(true, std::sync::atomic::Ordering::Relaxed) {
        let msg = format!("DEBUG find_all: mode={:?} count={} elapsed={:?} haystack_len={} strategy={}",
            mode, count, start_time.elapsed(), haystack.len(), re.strategy_name());
        // Write to file since stderr might be captured
        let _ = std::fs::write("/tmp/quickjs_debug.txt", &msg);
        eprintln!("{}", msg);
    }
    count
}

fn model_compile(b: &klv::Benchmark, mode: Mode) -> anyhow::Result<Vec<timer::Sample>> {
    let haystack = b.haystack_str()?;
    timer::run_and_count(
        b,
        |re: Regex| Ok(find_all(&re, haystack, mode)),
        || compile(b),
    )
}

fn model_count(
    b: &klv::Benchmark,
    re: &Regex,
    mode: Mode,
) -> anyhow::Result<Vec<timer::Sample>> {
    let haystack = b.haystack_str()?;
    timer::run(b, || Ok(find_all(re, haystack, mode)))
}

fn model_count_spans(
    b: &klv::Benchmark,
    re: &Regex,
    mode: Mode,
) -> anyhow::Result<Vec<timer::Sample>> {
    let haystack = b.haystack_str()?;
    timer::run(b, || {
        let mut sum = 0;
        let mut pos = 0;
        while pos < haystack.len() {
            let m = match mode {
                Mode::Hybrid => re.find(&haystack[pos..]),
                Mode::PureRust => re.find_at(haystack, pos),
                Mode::CEngine => re.find_at_c_engine(haystack, pos),
            };
            match m {
                Some(m) => {
                    let (start, end) = match mode {
                        Mode::Hybrid => (pos + m.start, pos + m.end),
                        _ => (m.start, m.end),
                    };
                    sum += end - start;
                    pos = if end > start { end } else { start + 1 };
                }
                None => break,
            }
        }
        Ok(sum)
    })
}

fn model_count_captures(
    b: &klv::Benchmark,
    re: &Regex,
    mode: Mode,
) -> anyhow::Result<Vec<timer::Sample>> {
    let haystack = b.haystack_str()?;
    timer::run(b, || {
        let mut count = 0;
        let mut pos = 0;
        while pos < haystack.len() {
            // Note: captures always uses C engine (no pure Rust implementation yet)
            let caps = match mode {
                Mode::Hybrid => re.captures(&haystack[pos..]),
                Mode::PureRust | Mode::CEngine => re.captures_at(haystack, pos),
            };
            match caps {
                Some(caps) => {
                    count += caps.len();
                    if let Some(m) = caps.get(0) {
                        let end = match mode {
                            Mode::Hybrid => pos + m.end,
                            _ => m.end,
                        };
                        let start = match mode {
                            Mode::Hybrid => pos + m.start,
                            _ => m.start,
                        };
                        pos = if end > start { end } else { start + 1 };
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }
        Ok(count)
    })
}

fn model_grep(
    b: &klv::Benchmark,
    re: &Regex,
    mode: Mode,
) -> anyhow::Result<Vec<timer::Sample>> {
    let haystack = b.haystack_str()?;
    timer::run(b, || {
        let mut count = 0;
        for line in haystack.lines() {
            let found = match mode {
                Mode::Hybrid => re.find(line).is_some(),
                Mode::PureRust => re.find_at(line, 0).is_some(),
                Mode::CEngine => re.find_at_c_engine(line, 0).is_some(),
            };
            if found {
                count += 1;
            }
        }
        Ok(count)
    })
}

fn model_grep_captures(
    b: &klv::Benchmark,
    re: &Regex,
    mode: Mode,
) -> anyhow::Result<Vec<timer::Sample>> {
    let haystack = b.haystack_str()?;
    timer::run(b, || {
        let mut count = 0;
        for line in haystack.lines() {
            let mut pos = 0;
            while pos < line.len() {
                // Note: captures always uses C engine (no pure Rust implementation yet)
                let caps = match mode {
                    Mode::Hybrid => re.captures(&line[pos..]),
                    Mode::PureRust | Mode::CEngine => re.captures_at(line, pos),
                };
                match caps {
                    Some(caps) => {
                        count += caps.len();
                        if let Some(m) = caps.get(0) {
                            let end = match mode {
                                Mode::Hybrid => pos + m.end,
                                _ => m.end,
                            };
                            let start = match mode {
                                Mode::Hybrid => pos + m.start,
                                _ => m.start,
                            };
                            pos = if end > start { end } else { start + 1 };
                        } else {
                            break;
                        }
                    }
                    None => break,
                }
            }
        }
        Ok(count)
    })
}

fn compile(b: &klv::Benchmark) -> anyhow::Result<Regex> {
    let pattern = b.regex.one()?;
    let mut flags = Flags::empty();
    if b.regex.case_insensitive {
        flags.insert(Flags::IGNORE_CASE);
    }
    if b.regex.unicode {
        flags.insert(Flags::UNICODE);
    }
    let re = Regex::with_flags(&pattern, flags)?;
    // Debug: print strategy to stderr
    eprintln!("DEBUG: pattern='{}' strategy={}", pattern, re.strategy_name());
    Ok(re)
}
