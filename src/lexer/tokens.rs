#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Number(i64),
    Identifier(String),
    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,
    OpenBracket,
    CloseBraket,
    Operator(Operator),
    Whitespace,
    Reserved(Reserved),
    EOF,
    Unknown(char),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mult,
    Div,
    Exp,
    Eq,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Reserved {
    Fn,
}

pub fn get_reserved(src: &str) -> Option<Reserved> {
    match src {
        "fn" => Some(Reserved::Fn),
        _ => None,
    }
}
