use quickjs_regex::Regex;

fn main() {
    // The exact pattern from rebar benchmarks
    let pattern = "Sherlock Holmes|John Watson|Irene Adler|Inspector Lestrade|Professor Moriarty";

    println!("Pattern: {}", pattern);

    let re = Regex::new(pattern).expect("Failed to compile");
    let strategy = re.strategy_name();

    println!("Selected strategy: {}", strategy);

    // Check if it starts with "AlternationLiterals"
    if strategy.starts_with("AlternationLiterals") {
        println!("SUCCESS: Aho-Corasick is being used");
    } else {
        println!("WARNING: Aho-Corasick is NOT being used!");
    }
}
