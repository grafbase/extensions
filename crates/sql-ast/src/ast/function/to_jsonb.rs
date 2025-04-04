use super::Function;
use crate::ast::Table;

#[derive(Debug, Clone, PartialEq)]
/// A representation of the `to_jsonb` function in PostgreSQL.
pub struct ToJsonb<'a> {
    pub(crate) table: Table<'a>,
}

/// Return the given table in JSONB.
pub fn to_jsonb<'a>(table: impl Into<Table<'a>>) -> Function<'a> {
    let fun = ToJsonb { table: table.into() };

    fun.into()
}

impl<'a> From<ToJsonb<'a>> for Function<'a> {
    fn from(value: ToJsonb<'a>) -> Self {
        Self {
            typ_: super::FunctionType::ToJsonb(value),
            alias: None,
        }
    }
}
