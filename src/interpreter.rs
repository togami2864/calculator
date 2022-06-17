use crate::ast::{Ast, Operator, UnaryOperator};
use crate::error::{CalcError, Result};

#[derive(Debug, Default)]
pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn eval(&mut self, expr: Ast) -> Result<i64> {
        match expr {
            Ast::Num(n) => Ok(n as i64),
            Ast::Unary { op, r } => {
                let r = self.eval(*r)?;
                match op {
                    UnaryOperator::Minus => Ok(0 - r),
                    UnaryOperator::Plus => Ok(r),
                }
            }
            Ast::BinOp { op, l, r } => {
                let l = self.eval(*l)?;
                let r = self.eval(*r)?;
                Ok(self.eval_binop(l, r, op)?)
            }
        }
    }

    fn eval_binop(&mut self, l: i64, r: i64, op: Operator) -> Result<i64> {
        let res = match op {
            Operator::Plus => l + r,
            Operator::Minus => l - r,
            Operator::Asterisk => l * r,
            Operator::Slash => {
                if r == 0 {
                    return Err(CalcError::DivisionByZero);
                }
                l / r
            }
        };
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, parser::Parser};

    use super::*;
    #[test]
    fn test_add() {
        let input = "1 + 1";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let ast = p.parse_expr().unwrap();
        let mut i = Interpreter::new();
        assert_eq!(i.eval(ast).unwrap(), 2);
    }

    #[test]
    fn test_sub() {
        let input = "1 - 1";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let ast = p.parse_expr().unwrap();
        let mut i = Interpreter::new();
        assert_eq!(i.eval(ast).unwrap(), 0);
    }

    #[test]
    fn test_mul() {
        let input = "1 * 2";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let ast = p.parse_expr().unwrap();
        let mut i = Interpreter::new();
        assert_eq!(i.eval(ast).unwrap(), 2);
    }

    #[test]
    fn test_div() {
        let input = "5 / 5";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let ast = p.parse_expr().unwrap();
        let mut i = Interpreter::new();
        assert_eq!(i.eval(ast).unwrap(), 1);
    }

    #[test]
    fn test_paren() {
        let input = "5 * (1 + 1) * 5";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let ast = p.parse_expr().unwrap();
        let mut i = Interpreter::new();
        assert_eq!(i.eval(ast).unwrap(), 50);
    }

    #[test]
    fn test_minus() {
        let input = "1 - 5";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let ast = p.parse_expr().unwrap();
        let mut i = Interpreter::new();
        assert_eq!(i.eval(ast).unwrap(), -4);
    }

    #[test]
    fn test_unary() {
        let input = "1 - -5";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let ast = p.parse_expr().unwrap();
        let mut i = Interpreter::new();
        assert_eq!(i.eval(ast).unwrap(), 6);
    }
}
