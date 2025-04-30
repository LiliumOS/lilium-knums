use lilium_sys::uuid::Uuid;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    ShiftLeft,
    ShiftRight,
    BitAnd,
    BitOr,
    BitXor,
    Div,
}

impl core::fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOp::Add => f.write_str("+"),
            BinaryOp::Sub => f.write_str("-"),
            BinaryOp::Mul => f.write_str("*"),
            BinaryOp::ShiftLeft => f.write_str("<<"),
            BinaryOp::ShiftRight => f.write_str(">>"),
            BinaryOp::BitAnd => f.write_str("&"),
            BinaryOp::BitOr => f.write_str("|"),
            BinaryOp::BitXor => f.write_str("^"),
            BinaryOp::Div => f.write_str("/"),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum UnaryOp {
    Plus,
    Neg,
    Not,
}

impl core::fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOp::Plus => f.write_str("+"),
            UnaryOp::Neg => f.write_str("-"),
            UnaryOp::Not => f.write_str("!"),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Expression {
    Id(String),
    Integer(String),
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    UuidLit(Uuid),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct UnaryExpr(pub UnaryOp, pub Box<Expression>);

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct BinaryExpr(pub BinaryOp, pub Box<Expression>, pub Box<Expression>);
