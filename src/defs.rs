use indexmap::IndexMap;

use crate::ast::expr::Expression;

pub struct Defs {
    pub consts: IndexMap<String, Expression>,
}
