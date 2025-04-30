use item::{Item, Path};

pub mod expr;

pub mod ty;

pub mod item;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct File {
    pub file_doc: Vec<String>,
    pub items: Vec<Item>,
}
