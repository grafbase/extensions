use super::Function;
use crate::ast::{FunctionType, Table};

#[derive(Debug, Clone, PartialEq)]
/// A representation of the `ROW_TO_JSON` function in the database.
/// Only for `Postgresql`
pub struct RowToJson<'a> {
    pub(crate) expr: Table<'a>,
    pub(crate) pretty_print: bool,
}

/// Return the given table in `JSON` format.
pub fn row_to_json<'a, T>(expr: T, pretty_print: bool) -> Function<'a>
where
    T: Into<Table<'a>>,
{
    let fun = RowToJson {
        expr: expr.into(),
        pretty_print,
    };

    fun.into()
}

impl<'a> From<RowToJson<'a>> for Function<'a> {
    fn from(value: RowToJson<'a>) -> Self {
        Self {
            r#type: FunctionType::RowToJson(value),
            alias: None,
        }
    }
}
