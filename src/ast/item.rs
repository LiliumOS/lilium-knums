use super::{
    expr::Expression,
    ty::{FnSignature, Type},
};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Item {
    pub doc: Vec<String>,
    pub body: ItemBody,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum ItemBody {
    Directive(String),
    Fn(ItemFn),
    Const(ItemConst),
    Use(ItemUse),
    Type(ItemTypeAlias),
    Struct(ItemStructy),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ItemFn {
    pub name: String,
    pub params: FnSignature,
    pub sysno: Expression,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ItemConst {
    pub name: String,
    pub ty: Type,
    pub value: Expression,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ItemUse {
    pub inline: bool,
    pub path: Path,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Path {
    pub has_leading: bool,
    pub components: Vec<String>,
}

impl core::fmt::Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sep = if self.has_leading { "::" } else { "" };

        for comp in &self.components {
            f.write_str(sep)?;
            sep = "::";
            f.write_str(comp)?;
        }
        Ok(())
    }
}

impl core::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sep = if self.has_leading { "::" } else { "" };

        for comp in &self.components {
            f.write_str(sep)?;
            sep = "::";
            f.write_str(comp)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ItemTypeAlias {
    pub name: String,
    pub def: Type,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct ItemStructy {
    pub kind: StructKind,
    pub name: String,
    pub generics: Vec<String>,
    pub properties: Vec<StructProperties>,
    pub body: StructBody,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum StructKind {
    Struct,
    Union,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum StructProperties {
    Align(Expression),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum StructBody {
    Opaque(Option<Type>),
    Fields(StructBodyFields),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct StructBodyFields {
    pub fields: Vec<StructField>,
    pub padding: Option<Padding>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Padding {
    Pad(Type),
    PadTo(Expression),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct StructField {
    pub name: String,
    pub ty: Type,
}
