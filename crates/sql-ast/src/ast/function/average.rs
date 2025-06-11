use super::Function;
use crate::ast::{Column, FunctionType};

/// A representation of the `AVG` function in the database.
#[derive(Debug, Clone)]
pub struct Average<'a> {
    pub(crate) column: Column<'a>,
}

/// Calculates the average value of a numeric column.
pub fn avg<'a, C>(col: C) -> Function<'a>
where
    C: Into<Column<'a>>,
{
    let fun = Average { column: col.into() };
    fun.into()
}

impl<'a> From<Average<'a>> for Function<'a> {
    fn from(value: Average<'a>) -> Self {
        Self {
            r#type: FunctionType::Average(value),
            alias: None,
        }
    }
}
