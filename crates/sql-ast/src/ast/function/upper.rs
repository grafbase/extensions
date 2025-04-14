use crate::ast::{Expression, Function, FunctionType};

/// A represention of the `UPPER` function in the database.
#[derive(Debug, Clone, PartialEq)]
pub struct Upper<'a> {
    pub(crate) expression: Box<Expression<'a>>,
}

/// Converts the result of the expression into uppercase string.
pub fn upper<'a, E>(expression: E) -> Function<'a>
where
    E: Into<Expression<'a>>,
{
    let fun = Upper {
        expression: Box::new(expression.into()),
    };

    fun.into()
}

impl<'a> From<Upper<'a>> for Function<'a> {
    fn from(value: Upper<'a>) -> Self {
        Self {
            typ_: FunctionType::Upper(value),
            alias: None,
        }
    }
}
