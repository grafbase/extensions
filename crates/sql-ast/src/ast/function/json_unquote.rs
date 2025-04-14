use super::Function;
use crate::ast::{Expression, FunctionType};

#[derive(Debug, Clone, PartialEq)]
pub struct JsonUnquote<'a> {
    pub(crate) expr: Box<Expression<'a>>,
}

/// Converts a JSON expression into string and unquotes it.
pub fn json_unquote<'a, E>(expr: E) -> Function<'a>
where
    E: Into<Expression<'a>>,
{
    let fun = JsonUnquote {
        expr: Box::new(expr.into()),
    };

    fun.into()
}

impl<'a> From<JsonUnquote<'a>> for Function<'a> {
    fn from(value: JsonUnquote<'a>) -> Self {
        Self {
            typ_: FunctionType::JsonUnquote(value),
            alias: None,
        }
    }
}
