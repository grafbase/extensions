use super::Function;
use crate::ast::{Column, FunctionType};

/// A represention of the `MAX` function in the database.
#[derive(Debug, Clone, PartialEq)]
pub struct Maximum<'a> {
    pub(crate) column: Column<'a>,
}

/// Calculates the maximum value of a numeric column.
pub fn max<'a, C>(col: C) -> Function<'a>
where
    C: Into<Column<'a>>,
{
    let fun = Maximum { column: col.into() };
    fun.into()
}

impl<'a> From<Maximum<'a>> for Function<'a> {
    fn from(value: Maximum<'a>) -> Self {
        Self {
            typ_: FunctionType::Maximum(value),
            alias: None,
        }
    }
}
