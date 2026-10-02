use moss_lang::lexer::{Lexer, Token};

fn main() {
    let src = "1 + 2";
    println!("{}", src);

    let mut lexer = Lexer::new(src);

    loop {
        let token = lexer.next_token();
        println!("{:?}", token);

        if token == Token::EOF {
            break;
        }
    }
}
