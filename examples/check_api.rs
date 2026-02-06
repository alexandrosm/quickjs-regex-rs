use regex_syntax::Parser;

fn main() {
    let hir = Parser::new().parse("a+").unwrap();
    println!("{:#?}", hir);

    let hir2 = Parser::new().parse("a{2,5}").unwrap();
    println!("{:#?}", hir2);
}
