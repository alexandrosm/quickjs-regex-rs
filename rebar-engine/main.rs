use std::io::Write;

use {
    anyhow::Context,
    lexopt::{Arg, ValueExt},
    quickjs_regex::{Flags, Regex, Scratch},
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
            }
            _ => return Err(arg.unexpected().into()),
        }
    }
    if version {
        writeln!(std::io::stdout(), "{}", env!("CARGO_PKG_VERSION"))?;
        return Ok(());
    }
    let b = klv::Benchmark::read(std::io::stdin())
        .context("failed to read KLV data from <stdin>")?;
    let samples = match b.model.as_str() {
        "compile" => model_compile(&b, mode)?,
        "count" => model_count(&b, &compile_with_mode(&b, mode)?, mode)?,
        "count-spans" => model_count_spans(&b, &compile_with_mode(&b, mode)?, mode)?,
        "count-captures" => model_count_captures(&b, &compile_with_mode(&b, mode)?, mode)?,
        "grep" => model_grep(&b, &compile_with_mode(&b, mode)?, mode)?,
        "grep-captures" => model_grep_captures(&b, &compile_with_mode(&b, mode)?, mode)?,
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

fn find_all(re: &Regex, haystack: &str, mode: Mode) -> usize {
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
    count
}

fn model_compile(b: &klv::Benchmark, mode: Mode) -> anyhow::Result<Vec<timer::Sample>> {
    let haystack = b.haystack_str()?;
    timer::run_and_count(
        b,
        |re: Regex| Ok(find_all(&re, haystack, mode)),
        || compile_with_mode(b, mode),
    )
}

fn model_count(
    b: &klv::Benchmark,
    re: &Regex,
    mode: Mode,
) -> anyhow::Result<Vec<timer::Sample>> {
    let haystack = b.haystack_str()?;
    // Use count_matches for PureRust mode - it uses native Aho-Corasick iteration
    // which is faster than repeated find_at calls (maintains automaton state)
    match mode {
        Mode::PureRust => timer::run(b, || Ok(re.count_matches(haystack))),
        _ => timer::run(b, || Ok(find_all(re, haystack, mode))),
    }
}

fn model_count_spans(
    b: &klv::Benchmark,
    re: &Regex,
    mode: Mode,
) -> anyhow::Result<Vec<timer::Sample>> {
    let haystack = b.haystack_str()?;
    let mut scratch = re.create_scratch();
    // DEBUG: time a single pass to see if Wide NFA path is working
    if matches!(mode, Mode::PureRust) {
        let start_time = std::time::Instant::now();
        let m = re.find_at_scratch(haystack, 0, &mut scratch);
        eprintln!("[DEBUG] first find_at_scratch: {:?} in {:?}, haystack={}",
            m, start_time.elapsed(), haystack.len());
    }
    timer::run(b, || {
        let mut sum = 0;
        let mut pos = 0;
        while pos < haystack.len() {
            let m = match mode {
                Mode::Hybrid => re.find(&haystack[pos..]),
                Mode::PureRust => re.find_at_scratch(haystack, pos, &mut scratch),
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
    let mut scratch = re.create_scratch();
    timer::run(b, || {
        let mut count = 0;
        let mut pos = 0;
        while pos < haystack.len() {
            let caps = match mode {
                Mode::Hybrid => re.captures(&haystack[pos..]),
                Mode::PureRust => re.captures_at_scratch(haystack, pos, &mut scratch),
                Mode::CEngine => re.captures_at(haystack, pos),
            };
            match caps {
                Some(caps) => {
                    // Count only groups that actually matched (non-None)
                    count += caps.count_matched();
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
    let mut scratch = re.create_scratch();
    timer::run(b, || {
        let mut count = 0;
        for line in haystack.lines() {
            let found = match mode {
                Mode::Hybrid => re.find(line).is_some(),
                Mode::PureRust => re.find_at_scratch(line, 0, &mut scratch).is_some(),
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
    let mut scratch = re.create_scratch();
    timer::run(b, || {
        let mut count = 0;
        for line in haystack.lines() {
            let mut pos = 0;
            while pos < line.len() {
                let caps = match mode {
                    Mode::Hybrid => re.captures(&line[pos..]),
                    Mode::PureRust => re.captures_at_scratch(line, pos, &mut scratch),
                    Mode::CEngine => re.captures_at(line, pos),
                };
                match caps {
                    Some(caps) => {
                        count += caps.count_matched();
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
    compile_with_mode(b, Mode::Hybrid)
}

fn compile_with_mode(b: &klv::Benchmark, mode: Mode) -> anyhow::Result<Regex> {
    let pattern = b.regex.one()?;
    let mut flags = Flags::empty();
    if b.regex.case_insensitive {
        flags.insert(Flags::IGNORE_CASE);
    }
    if b.regex.unicode {
        flags.insert(Flags::UNICODE);
    }
    match mode {
        Mode::PureRust => Ok(Regex::with_flags_pure_rust(&pattern, flags)?),
        _ => Ok(Regex::with_flags(&pattern, flags)?),
    }
}
