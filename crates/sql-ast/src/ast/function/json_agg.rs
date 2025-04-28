use super::Function;
use crate::ast::{Expression, Ordering};

#[derive(Debug, Clone, PartialEq)]
/// A representation of the `json_agg` function in PostgreSQL.
pub struct JsonAgg<'a> {
    pub(crate) expression: Expression<'a>,
    pub(crate) distinct: bool,
    pub(crate) order_by: Option<Ordering<'a>>,
}

/// Return the given table as JSONB collection.
pub fn json_agg<'a>(
    expression: impl Into<Expression<'a>>,
    order_by: Option<Ordering<'a>>,
    distinct: bool,
) -> Function<'a> {
    let fun = JsonAgg {
        expression: expression.into(),
        distinct,
        order_by,
    };

    fun.into()
}

impl<'a> From<JsonAgg<'a>> for Function<'a> {
    fn from(value: JsonAgg<'a>) -> Self {
        Self {
            typ_: super::FunctionType::JsonAgg(value),
            alias: None,
        }
    }
}
