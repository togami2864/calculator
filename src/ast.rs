/// expr = mul ("+" mul | "-" mul)*
/// mul = unary ("*" unary | "/" unary)*
/// unary = ("+"|"-")? primary
/// primary = num | "(" expr ")"

#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    BinOp {
        op: Operator,
        l: Box<Ast>,
        r: Box<Ast>,
    },
    Unary {
        op: UnaryOperator,
        r: Box<Ast>,
    },
    Integer(i64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Asterisk,
    Slash,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Plus,
    Minus,
}
