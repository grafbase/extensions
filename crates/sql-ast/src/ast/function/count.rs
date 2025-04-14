use super::Function;
use crate::ast::{Expression, FunctionType};

#[derive(Debug, Clone, PartialEq)]
/// Returns the number of rows that matches a specified criteria.
pub struct Count<'a> {
    pub(crate) exprs: Vec<Expression<'a>>,
}

/// Count of the underlying table where the given expression is not null.
pub fn count<'a, T>(expr: T) -> Function<'a>
where
    T: Into<Expression<'a>>,
{
    let fun = Count {
        exprs: vec![expr.into()],
    };

    fun.into()
}

impl<'a> From<Count<'a>> for Function<'a> {
    fn from(value: Count<'a>) -> Self {
        Self {
            typ_: FunctionType::Count(value),
            alias: None,
        }
    }
}
