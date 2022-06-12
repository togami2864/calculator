use crate::{
    ast::{Ast, Operator},
    lexer::Lexer,
    token::Token,
};
use std::fmt;
#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken(Token, Token),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::UnexpectedToken(expected, token) => {
                write!(
                    f,
                    "Unexpected Token: expected '{}', but got '{}'",
                    expected, token
                )
            }
        }
    }
}

type ParserResult = Result<Ast, ParserError>;

#[derive(Debug, Clone)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    cur_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Self {
            lexer,
            cur_token: Token::Illegal,
            peek_token: Token::Illegal,
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    pub fn next_token(&mut self) -> Token {
        let cur = self.cur_token.clone();
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token().unwrap();
        cur
    }

    pub fn parse_expr(&mut self) -> ParserResult {
        let left = self.parse_mul()?;
        let op = match self.cur_token {
            Token::Plus => Operator::Plus,
            Token::Minus => Operator::Minus,
            _ => return Ok(left),
        };
        self.next_token();
        let right = self.parse_mul()?;
        Ok(Ast::BinOp {
            op,
            l: Box::new(left),
            r: Box::new(right),
        })
    }

    fn parse_mul(&mut self) -> ParserResult {
        let left = self.parse_primary()?;
        self.next_token();
        let op = match self.cur_token {
            Token::Asterisk => Operator::Asterisk,
            Token::Slash => Operator::Slash,
            _ => return Ok(left),
        };
        self.next_token();
        let right = self.parse_primary()?;
        Ok(Ast::BinOp {
            op,
            l: Box::new(left),
            r: Box::new(right),
        })
    }

    fn parse_primary(&mut self) -> ParserResult {
        match self.cur_token {
            Token::Num(n) => Ok(Ast::Num(n)),
            Token::LParen => {
                self.next_token();
                let expr = self.parse_expr()?;
                match self.cur_token.clone() {
                    Token::RParen => Ok(expr),
                    unexpected => Err(ParserError::UnexpectedToken(Token::RParen, unexpected)),
                }
            }
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_plus() {
        let input = "1 + 1";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        assert_eq!(
            p.parse_expr().unwrap(),
            Ast::BinOp {
                l: Box::new(Ast::Num(1)),
                op: Operator::Plus,
                r: Box::new(Ast::Num(1))
            }
        );
    }

    #[test]
    fn test_minus() {
        let input = "1 - 1";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        assert_eq!(
            p.parse_expr().unwrap(),
            Ast::BinOp {
                l: Box::new(Ast::Num(1)),
                op: Operator::Minus,
                r: Box::new(Ast::Num(1))
            }
        );
    }

    #[test]
    fn test_mul() {
        let input = "1 * 4";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        assert_eq!(
            p.parse_expr().unwrap(),
            Ast::BinOp {
                l: Box::new(Ast::Num(1)),
                op: Operator::Asterisk,
                r: Box::new(Ast::Num(4))
            }
        );
    }

    #[test]
    fn test_div() {
        let input = "4 / 2";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        assert_eq!(
            p.parse_expr().unwrap(),
            Ast::BinOp {
                l: Box::new(Ast::Num(4)),
                op: Operator::Slash,
                r: Box::new(Ast::Num(2))
            }
        );
    }
    #[test]
    fn test_paren() {
        let input = "(1 + 3) * 2";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        assert_eq!(
            p.parse_expr().unwrap(),
            Ast::BinOp {
                l: Box::new(Ast::BinOp {
                    op: Operator::Plus,
                    l: Box::new(Ast::Num(1)),
                    r: Box::new(Ast::Num(3))
                }),
                op: Operator::Asterisk,
                r: Box::new(Ast::Num(2))
            }
        );
    }
}
