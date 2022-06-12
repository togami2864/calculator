/// expr = mul ("+" mul | "-" mul)*
/// mul = primary ("*" primary | "/" primary)*
/// primary = num | "(" expr ")"

#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    BinOp {
        op: Operator,
        l: Box<Ast>,
        r: Box<Ast>,
    },
    Num(i64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Asterisk,
    Slash,
}
