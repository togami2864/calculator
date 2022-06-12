use crate::ast::{Ast, Operator};

#[derive(Debug, Clone)]
pub enum InterpreterError {
    DivisionByZero,
}

type InterpreterResult = Result<u64, InterpreterError>;

#[derive(Debug, Default)]
pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn eval(&mut self, expr: Ast) -> InterpreterResult {
        match expr {
            Ast::Num(n) => Ok(n as u64),
            Ast::BinOp { op, l, r } => {
                let l = self.eval(*l)?;
                let r = self.eval(*r)?;
                Ok(self.eval_binop(l, r, op)?)
            }
        }
    }

    fn eval_binop(&mut self, l: u64, r: u64, op: Operator) -> InterpreterResult {
        let res = match op {
            Operator::Plus => l + r,
            Operator::Minus => l - r,
            Operator::Asterisk => l * r,
            Operator::Slash => {
                if r == 0 {
                    return Err(InterpreterError::DivisionByZero);
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
        let input = "5 * (1 + 1)";
        let l = Lexer::new(input);
        let mut p = Parser::new(l);
        let ast = p.parse_expr().unwrap();
        let mut i = Interpreter::new();
        assert_eq!(i.eval(ast).unwrap(), 10);
    }
}