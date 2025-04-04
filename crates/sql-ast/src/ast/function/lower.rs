use super::Function;
use crate::ast::{Expression, FunctionType};

/// A represention of the `LOWER` function in the database.
#[derive(Debug, Clone, PartialEq)]
pub struct Lower<'a> {
    pub(crate) expression: Box<Expression<'a>>,
}

/// Converts the result of the expression into lowercase string.
pub fn lower<'a, E>(expression: E) -> Function<'a>
where
    E: Into<Expression<'a>>,
{
    let fun = Lower {
        expression: Box::new(expression.into()),
    };

    fun.into()
}

impl<'a> From<Lower<'a>> for Function<'a> {
    fn from(value: Lower<'a>) -> Self {
        Self {
            typ_: FunctionType::Lower(value),
            alias: None,
        }
    }
}
