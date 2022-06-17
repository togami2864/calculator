use crate::token::Token;
use std::error::Error;
use std::fmt::{self};

#[derive(Debug)]
pub enum CalcError {
    DivisionByZero,
    UnexpectedToken(Token, Token),
    InvalidInput(char),
    IllegalToken(Token),
}

impl Error for CalcError {}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::DivisionByZero => {
                write!(f, "DivisionByZero: This operation will panic at runtime")
            }
            CalcError::UnexpectedToken(expected, actual) => {
                write!(
                    f,
                    "UnexpectedToken: Expected {}, but got {}",
                    expected, actual
                )
            }
            CalcError::InvalidInput(c) => write!(f, "Invalid input: {}", c),
            CalcError::IllegalToken(tok) => write!(f, "Illegal Token: {}", tok),
        }
    }
}

pub type Result<T> = std::result::Result<T, CalcError>;
