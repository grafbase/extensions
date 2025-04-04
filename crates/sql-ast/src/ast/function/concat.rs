use super::Function;
use crate::ast::{Expression, FunctionType};

/// A representation of the `Concat` function in the database.
#[derive(Debug, Clone, PartialEq)]
pub struct Concat<'a> {
    pub(crate) exprs: Vec<Expression<'a>>,
}

/// Concat several expressions.
pub fn concat<'a, T>(exprs: Vec<T>) -> Function<'a>
where
    T: Into<Expression<'a>>,
{
    let fun = Concat {
        exprs: exprs.into_iter().map(Into::into).collect(),
    };

    fun.into()
}

impl<'a> From<Concat<'a>> for Function<'a> {
    fn from(value: Concat<'a>) -> Self {
        Self {
            typ_: FunctionType::Concat(value),
            alias: None,
        }
    }
}
