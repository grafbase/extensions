use crate::ast::{Expression, Function, FunctionType};

/// A represention of the `SUM` function in the database.
#[derive(Debug, Clone, PartialEq)]
pub struct Sum<'a> {
    pub(crate) expr: Box<Expression<'a>>,
}

/// Calculates the sum value of a numeric column.
pub fn sum<'a, E>(expr: E) -> Function<'a>
where
    E: Into<Expression<'a>>,
{
    let fun = Sum {
        expr: Box::new(expr.into()),
    };

    fun.into()
}

impl<'a> From<Sum<'a>> for Function<'a> {
    fn from(value: Sum<'a>) -> Self {
        Self {
            typ_: FunctionType::Sum(value),
            alias: None,
        }
    }
}
