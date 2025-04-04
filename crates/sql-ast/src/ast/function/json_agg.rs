use super::Function;
use crate::ast::{Expression, Ordering};

#[derive(Debug, Clone, PartialEq)]
/// A representation of the `json_agg` function in PostgreSQL.
pub struct JsonbAgg<'a> {
    pub(crate) expression: Expression<'a>,
    pub(crate) distinct: bool,
    pub(crate) order_by: Option<Ordering<'a>>,
}

/// Return the given table as JSONB collection.
pub fn jsonb_agg<'a>(
    expression: impl Into<Expression<'a>>,
    order_by: Option<Ordering<'a>>,
    distinct: bool,
) -> Function<'a> {
    let fun = JsonbAgg {
        expression: expression.into(),
        distinct,
        order_by,
    };

    fun.into()
}

impl<'a> From<JsonbAgg<'a>> for Function<'a> {
    fn from(value: JsonbAgg<'a>) -> Self {
        Self {
            typ_: super::FunctionType::JsonbAgg(value),
            alias: None,
        }
    }
}
