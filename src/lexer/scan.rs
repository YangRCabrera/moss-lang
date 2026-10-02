use std::str::Chars;

#[derive(Debug, PartialEq, Eq)]
pub enum Token<'a> {
    Number(i64),
    Identifier(&'a str),
    Assign,
    OpenParen,
    CloseParen,
    Operator(Operator),
    Whitespace,
    EOF,
    Unknown,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Add,
    Sub,
    Mult,
    Div,
    Exp,
}

pub struct Lexer<'a> {
    input: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars(),
        }
    }

    pub fn next_token(&mut self) -> Token<'a> {
        match self.input.next() {
            Some('+') => Token::Operator(Operator::Add),
            Some('-') => Token::Operator(Operator::Sub),
            Some('*') => Token::Operator(Operator::Mult),
            Some('/') => Token::Operator(Operator::Div),
            Some('^') => Token::Operator(Operator::Exp),
            Some('(') => Token::OpenParen,
            Some(')') => Token::CloseParen,
            Some(c) => {
                if c.is_ascii_digit() {
                    Token::Number(c.to_digit(10).map(|d| d as i64).unwrap())
                } else if c == ' ' {
                    Token::Whitespace
                } else {
                    Token::Unknown
                }
            }
            None => Token::EOF,
        }
    }
}
