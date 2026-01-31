use quickjs_regex::Regex;

fn main() {
    // Test 1: Bounded repeat \d{4}-\d{2}-\d{2}
    println!("=== Test 1: Bounded repeat ===");
    let pattern = r"\d{4}-\d{2}-\d{2}";
    println!("Pattern: {}", pattern);
    match Regex::new(pattern) {
        Ok(re) => {
            let text = "Events on 2024-01-15, 2024-02-20, and 2024-12-25";
            println!("Text: {}", text);
            let matches: Vec<_> = re.find_iter(text).collect();
            println!("Matches found: {}", matches.len());
            for m in &matches {
                println!("  Match: '{}' at {}..{}", m.as_str(text), m.start, m.end);
            }
            
            // Try simpler patterns
            let simple = Regex::new(r"\d\d\d\d").unwrap();
            let simple_matches: Vec<_> = simple.find_iter(text).collect();
            println!("Simple dddd matches: {}", simple_matches.len());
            
            let simple2 = Regex::new(r"\d{2}").unwrap();
            let simple2_matches: Vec<_> = simple2.find_iter(text).collect();
            println!("Simple d{{2}} matches: {}", simple2_matches.len());
        }
        Err(e) => println!("Error compiling: {}", e),
    }
    
    // Test 2: Complex capture groups
    println!("\n=== Test 2: Complex capture groups ===");
    let pattern2 = r"(\d{4})-(\d{2})-(\d{2})T(\d{2}):(\d{2}):(\d{2})";
    println!("Pattern: {}", pattern2);
    match Regex::new(pattern2) {
        Ok(re) => {
            let text = "timestamp: 2024-01-15T14:30:45";
            println!("Text: {}", text);
            if let Some(m) = re.find(text) {
                println!("Match found: '{}' at {}..{}", m.as_str(text), m.start, m.end);
            } else {
                println!("No match found");
            }
            
            // Try simpler version
            let simple = Regex::new(r"\d+-\d+-\d+T\d+:\d+:\d+").unwrap();
            if let Some(m) = simple.find(text) {
                println!("Simple pattern match: '{}' at {}..{}", m.as_str(text), m.start, m.end);
            } else {
                println!("Simple pattern no match");
            }
        }
        Err(e) => println!("Error compiling: {}", e),
    }
    
    // Test 3: Basic bounded repeat
    println!("\n=== Test 3: Basic bounded repeat ===");
    let re = Regex::new(r"a{2,4}").unwrap();
    let text = "a aa aaa aaaa aaaaa";
    let matches: Vec<_> = re.find_iter(text).collect();
    println!("Pattern: a{{2,4}} on '{}'", text);
    println!("Matches: {}", matches.len());
    for m in &matches {
        println!("  '{}' at {}..{}", m.as_str(text), m.start, m.end);
    }
}
