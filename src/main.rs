use moss_lang::lexer::Lexer;

fn main() {
    let src = "x = 1 + 21 * 4";
    println!("{}\n", src);

    let mut lexer = Lexer::new(src);
    let tokens = lexer.generate_tokens();
    println!("{:#?}", tokens)
}
