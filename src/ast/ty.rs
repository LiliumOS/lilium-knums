use std::num::NonZero;

use super::expr::Expression;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct FnSignature {
    pub params: Vec<FnParam>,
    pub ret_ty: Box<Type>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct FnParam {
    pub name: Option<String>,
    pub ty: Type,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Type {
    Integer(IntType),
    Char,
    Void,
    Never,
    Array(Box<Type>, Expression),
    Fn(FnSignature),
    Pointer(PointerKind, Box<Type>),
    Named(String),
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct IntType {
    pub signed: bool,
    pub width: IntWidth,
}

impl core::fmt::Debug for IntType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.signed {
            f.write_str("i")?;
        } else {
            f.write_str("u")?;
        }

        match self.width {
            IntWidth::Bits(n) => n.fmt(f),
            IntWidth::Long => f.write_str("long"),
        }
    }
}

impl core::fmt::Display for IntType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.signed {
            f.write_str("i")?;
        } else {
            f.write_str("u")?;
        }

        match self.width {
            IntWidth::Bits(n) => n.fmt(f),
            IntWidth::Long => f.write_str("long"),
        }
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum IntWidth {
    Bits(NonZero<u8>),
    Long,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum PointerKind {
    Const,
    Mut,
    Handle,
    Shared,
}
