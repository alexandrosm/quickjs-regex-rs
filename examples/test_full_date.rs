use quickjs_regex::{Regex, Flags};

fn main() {
    // The actual date pattern from rebar benchmarks
    let pattern = r#"((19\d\d01[0-3]\d[0-5]\d[0-5]\d[0-5]\d|20\d\d01[0-3]\d[0-5]\d[0-5]\d[0-5]\d|19\d\d02[0-3]\d[0-5]\d[0-5]\d[0-5]\d|20\d\d02[0-3]\d[0-5]\d[0-5]\d[0-5]\d|19\d\d03[0-3]\d[0-5]\d[0-5]\d[0-5]\d|20\d\d03[0-3]\d[0-5]\d[0-5]\d[0-5]\d|19\d\d04[0-3]\d[0-5]\d[0-5]\d[0-5]\d|20\d\d04[0-3]\d[0-5]\d[0-5]\d[0-5]\d|19\d\d05[0-3]\d[0-5]\d[0-5]\d[0-5]\d|20\d\d05[0-3]\d[0-5]\d[0-5]\d[0-5]\d|19\d\d06[0-3]\d[0-5]\d[0-5]\d[0-5]\d|20\d\d06[0-3]\d[0-5]\d[0-5]\d[0-5]\d|19\d\d07[0-3]\d[0-5]\d[0-5]\d[0-5]\d|20\d\d07[0-3]\d[0-5]\d[0-5]\d[0-5]\d|19\d\d08[0-3]\d[0-5]\d[0-5]\d[0-5]\d|20\d\d08[0-3]\d[0-5]\d[0-5]\d[0-5]\d|19\d\d09[0-3]\d[0-5]\d[0-5]\d[0-5]\d|20\d\d09[0-3]\d[0-5]\d[0-5]\d[0-5]\d|19\d\d10[0-3]\d[0-5]\d[0-5]\d[0-5]\d|20\d\d10[0-3]\d[0-5]\d[0-5]\d[0-5]\d|19\d\d11[0-3]\d[0-5]\d[0-5]\d[0-5]\d|20\d\d11[0-3]\d[0-5]\d[0-5]\d[0-5]\d|19\d\d12[0-3]\d[0-5]\d[0-5]\d[0-5]\d|20\d\d12[0-3]\d[0-5]\d[0-5]\d[0-5]\d|19\d\d01[0123]\d|20\d\d01[0123]\d|19\d\d02[0123]\d|20\d\d02[0123]\d|19\d\d03[0123]\d|20\d\d03[0123]\d|19\d\d04[0123]\d|20\d\d04[0123]\d|19\d\d05[0123]\d|20\d\d05[0123]\d|19\d\d06[0123]\d|20\d\d06[0123]\d|19\d\d07[0123]\d|20\d\d07[0123]\d|19\d\d08[0123]\d|20\d\d08[0123]\d|19\d\d09[0123]\d|20\d\d09[0123]\d|19\d\d10[0123]\d|20\d\d10[0123]\d|19\d\d11[0123]\d|20\d\d11[0123]\d|19\d\d12[0123]\d|20\d\d12[0123]\d|19\d\d01|20\d\d01|19\d\d02|20\d\d02|19\d\d03|20\d\d03|19\d\d04|20\d\d04|19\d\d05|20\d\d05|19\d\d06|20\d\d06|19\d\d07|20\d\d07|19\d\d08|20\d\d08|19\d\d09|20\d\d09|19\d\d10|20\d\d10|19\d\d11|20\d\d11|19\d\d12|20\d\d12|(-?(:[1-9][0-9]*)?[0-9]{4})-(1[0-2]|0[1-9])-(3[01]|0[1-9]|[12][0-9])T(2[0-3]|[01][0-9]):([0-5][0-9]):([0-5][0-9])(?:[.,]+([0-9]+))?((?:Z|[+-](?:2[0-3]|[01][0-9]):[0-5][0-9]))?)|((((\d{1,2}):(\d{1,2})(:(\d{1,2}))?([.,](\d{1,6}))?\s*(a.m.|am|p.m.|pm)?\s*(ACDT|ACST|ACT|pacific|eastern|mountain|central)?)|((\d{1,2})\s*(a.m.|am|p.m.|pm)\s*(ACDT|ACST|ACT|pacific|eastern|mountain|central)*))|(19\d\d|20\d\d)|(first|second|third|fourth|fifth|sixth|seventh|eighth|nineth|tenth)|(\d+)(st|th|rd|nd)?|(monday|tuesday|wednesday|thursday|friday|saturday|sunday|mon|tue|wed|thu|fri|sat|sun)|(january|february|march|april|may|june|july|august|september|october|november|december|jan[.\s]|feb[.\s]|mar[.\s]|apr[.\s]|may[.\s]|jun[.\s]|jul[.\s]|aug[.\s]|sep[^A-Za-z]|sept[.\s]|oct[.\s]|nov[.\s]|dec[.\s])|([/:\-,.\s_+@]+)|(next|last)|(due|by|on|during|standard|daylight|savings|time|date|dated|of|to|through|between|until|at|day)){1,1})"#;

    println!("Testing with (simplified) full date regex");
    println!("Pattern length: {}", pattern.len());

    let text = "Meeting on January 15, 2024 at 10:30 am EST";

    let re = match Regex::with_flags(pattern, Flags::from_bits(Flags::IGNORE_CASE)) {
        Ok(r) => r,
        Err(e) => {
            println!("Error compiling: {:?}", e);
            return;
        }
    };

    // Test find_at
    println!("\nUsing find_at:");
    let mut matches = Vec::new();
    let mut pos = 0;
    while pos < text.len() {
        match re.find_at(text, pos) {
            Some(m) => {
                matches.push((&text[m.start..m.end], m.start, m.end));
                pos = if m.end > m.start { m.end } else { m.start + 1 };
            }
            None => break,
        }
    }

    println!("Text: {}", text);
    println!("Found {} matches:", matches.len());
    for (s, start, end) in &matches {
        println!("  '{}' at {}..{}", s, start, end);
    }

    // Test find() on slices
    println!("\nUsing find() on slices (like benchmark Hybrid mode):");
    let mut count = 0;
    let mut pos = 0;
    while pos < text.len() {
        match re.find(&text[pos..]) {
            Some(m) => {
                count += 1;
                let abs_start = pos + m.start;
                let abs_end = pos + m.end;
                println!("  '{}' at {}..{}", &text[abs_start..abs_end], abs_start, abs_end);
                pos = if abs_end > abs_start { abs_end } else { abs_start + 1 };
            }
            None => break,
        }
    }
    println!("Total: {} matches", count);
}
