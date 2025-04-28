use super::Function;
use crate::ast::{Expression, FunctionType};

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayPosition<'a> {
    pub(crate) array: Expression<'a>,
    pub(crate) column: Expression<'a>,
}

/// Return the given table in `JSON` format.
pub fn array_position<'a, T, E>(array: T, column: E) -> Function<'a>
where
    T: Into<Expression<'a>>,
    E: Into<Expression<'a>>,
{
    let fun = ArrayPosition {
        array: array.into(),
        column: column.into(),
    };

    fun.into()
}

impl<'a> From<ArrayPosition<'a>> for Function<'a> {
    fn from(value: ArrayPosition<'a>) -> Self {
        Self {
            typ_: FunctionType::ArrayPosition(value),
            alias: None,
        }
    }
}
