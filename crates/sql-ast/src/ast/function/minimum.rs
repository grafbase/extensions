use super::Function;
use crate::ast::{Column, FunctionType};

/// A represention of the `MIN` function in the database.
#[derive(Debug, Clone, PartialEq)]
pub struct Minimum<'a> {
    pub(crate) column: Column<'a>,
}

/// Calculates the minimum value of a numeric column.
pub fn min<'a, C>(col: C) -> Function<'a>
where
    C: Into<Column<'a>>,
{
    let fun = Minimum { column: col.into() };
    fun.into()
}

impl<'a> From<Minimum<'a>> for Function<'a> {
    fn from(value: Minimum<'a>) -> Self {
        Self {
            typ_: FunctionType::Minimum(value),
            alias: None,
        }
    }
}
