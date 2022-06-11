use crate::token::Token;
use std::fmt;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: std::str::Chars<'a>,
    cur: char,
    peek: char,
}

#[derive(Debug)]
pub enum LexerError {
    InvalidInput(char),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "found invalid value: {}", self)
    }
}

impl std::error::Error for LexerError {}

type LexerResult = std::result::Result<Token, LexerError>;

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut l = Self {
            input: input.chars(),
            cur: '\u{0}',
            peek: '\u{0}',
        };
        l.readChar();
        l.readChar();
        l
    }

    fn readChar(&mut self) -> char {
        let c = self.cur;
        self.cur = self.peek;
        self.peek = self.input.next().unwrap_or('\u{0}');
        c
    }

    pub fn nextToken(&mut self) -> LexerResult {
        let ch = self.cur;
        let token = match ch {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '\u{0}' => Token::Eof,

            b => {
                if b.is_digit(10) {
                    let mut num = String::new();
                    num.push(b);
                    Token::Num(num.parse().unwrap())
                } else {
                    return Err(LexerError::InvalidInput(b));
                }
            }
        };
        self.readChar();
        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let input = "1 + 1";
        let mut l = Lexer::new(input);
        assert_eq!(l.nextToken().unwrap(), Token::Num(1));
        assert_eq!(l.nextToken().unwrap(), Token::Plus);
        assert_eq!(l.nextToken().unwrap(), Token::Num(1));
    }

    #[test]
    fn test_minus() {
        let input = "1-1";
        let mut l = Lexer::new(input);
        assert_eq!(l.nextToken().unwrap(), Token::Num(1));
        assert_eq!(l.nextToken().unwrap(), Token::Minus);
        assert_eq!(l.nextToken().unwrap(), Token::Num(1));
    }

    #[test]
    fn test_asterisk() {
        let input = "1*1";
        let mut l = Lexer::new(input);
        assert_eq!(l.nextToken().unwrap(), Token::Num(1));
        assert_eq!(l.nextToken().unwrap(), Token::Asterisk);
        assert_eq!(l.nextToken().unwrap(), Token::Num(1));
    }

    #[test]
    fn test_slash() {
        let input = "1/1";
        let mut l = Lexer::new(input);
        assert_eq!(l.nextToken().unwrap(), Token::Num(1));
        assert_eq!(l.nextToken().unwrap(), Token::Slash);
        assert_eq!(l.nextToken().unwrap(), Token::Num(1));
    }
}
