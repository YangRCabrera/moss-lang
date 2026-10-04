use crate::lexer::{Operator, Token, tokens::get_reserved};
use std::{iter::Peekable, str::Chars};

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
        }
    }

    fn next_token(&mut self) -> Token {
        match self.input.next() {
            Some('+') => Token::Operator(Operator::Add),
            Some('-') => Token::Operator(Operator::Sub),
            Some('*') => Token::Operator(Operator::Mult),
            Some('/') => Token::Operator(Operator::Div),
            Some('^') => Token::Operator(Operator::Exp),
            Some('=') => Token::Operator(Operator::Eq),
            Some('(') => Token::OpenParen,
            Some(')') => Token::CloseParen,
            Some('{') => Token::OpenCurly,
            Some('}') => Token::CloseCurly,
            Some('[') => Token::OpenBracket,
            Some(']') => Token::CloseBraket,
            Some(c) if c.is_ascii_digit() => {
                let mut number_str = c.to_string();
                while let Some(&next_char) = self.input.peek() {
                    if next_char.is_ascii_digit() {
                        number_str.push(self.input.next().unwrap());
                    } else {
                        break;
                    }
                }
                Token::Number(number_str.parse::<i64>().unwrap())
            }
            Some(c) if c.is_ascii_alphabetic() || c == '_' => {
                let mut identifier_str = c.to_string();
                while let Some(&next_char) = self.input.peek() {
                    if next_char.is_ascii_alphabetic()
                        || next_char == '_'
                        || next_char.is_ascii_digit()
                    {
                        identifier_str.push(self.input.next().unwrap());
                    } else {
                        break;
                    }
                }
                if let Some(reserved) = get_reserved(&identifier_str) {
                    Token::Reserved(reserved)
                } else {
                    Token::Identifier(identifier_str)
                }
            }
            Some(' ') => Token::Whitespace,
            Some(c) => Token::Unknown(c),
            None => Token::EOF,
        }
    }

    pub fn generate_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            tokens.push(token.clone());

            if token == Token::EOF {
                break;
            }
        }
        tokens
    }
}
