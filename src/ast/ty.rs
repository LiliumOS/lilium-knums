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
    Byte,
    Array(ArrayType),
    Fn(FnSignature),
    Pointer(PointerType),
    Named(NamedType),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ArrayType(pub Box<Type>, pub Expression);

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct PointerType(pub PointerKind, pub Box<Type>);

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct NamedType(pub String, pub Option<NameSuffix>);

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum NameSuffix {
    Generics(Vec<Type>),
    ParamReplace(Box<Type>),
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

        self.width.fmt(f)
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum IntWidth {
    Bits(NonZero<u8>),
    Long,
}

impl core::fmt::Display for IntWidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntWidth::Bits(n) => n.fmt(f),
            IntWidth::Long => f.write_str("long"),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum PointerKind {
    Const,
    Mut,
    Handle,
    Shared,
}
