use super::Function;
use crate::ast::{Expression, FunctionType};

#[derive(Debug, Clone, PartialEq)]
/// Returns the first non-null expression
pub struct Coalesce<'a> {
    pub(crate) exprs: Vec<Expression<'a>>,
}

/// Returns the first non-null argument
pub fn coalesce<'a, T, V>(exprs: V) -> Function<'a>
where
    T: Into<Expression<'a>>,
    V: Into<Vec<T>>,
{
    let fun = Coalesce {
        exprs: exprs.into().into_iter().map(|e| e.into()).collect(),
    };

    fun.into()
}

impl<'a> From<Coalesce<'a>> for Function<'a> {
    fn from(value: Coalesce<'a>) -> Self {
        Self {
            typ_: FunctionType::Coalesce(value),
            alias: None,
        }
    }
}
