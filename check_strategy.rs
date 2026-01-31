use quickjs_regex::Regex;

fn main() {
    // Check what strategy is being used for simple literals
    let re_holmes = Regex::new("Holmes").unwrap();
    println!("Holmes: {:?}", re_holmes);
    
    let re_the = Regex::new("the").unwrap();
    println!("the: {:?}", re_the);
    
    let re_digits = Regex::new("[0-9]+").unwrap();
    println!("[0-9]+: {:?}", re_digits);
}
