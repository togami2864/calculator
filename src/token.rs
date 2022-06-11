use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Num(u64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen,
    Eof,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Plus => f.write_str("+"),
            Token::Minus => f.write_str("-"),
            Token::Asterisk => f.write_str("*"),
            Token::Slash => f.write_str("/"),
            Token::LParen => f.write_str("("),
            Token::RParen => f.write_str(")"),
            Token::Eof => f.write_str("EOF"),
            Token::Num(n) => write!(f, "{}", n),
        }
    }
}
