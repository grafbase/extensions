use super::{EncodeFormat, Function};
use crate::ast::Expression;

/// A representation of the `decode` function in PostgreSQL.
#[derive(Debug, Clone)]
pub struct Decode<'a> {
    pub(crate) expression: Expression<'a>,
    pub(crate) format: EncodeFormat,
}

/// Decode a given expression into a specified format.
pub fn decode<'a>(expression: impl Into<Expression<'a>>, format: EncodeFormat) -> Function<'a> {
    let fun = Decode {
        expression: expression.into(),
        format,
    };

    fun.into()
}

impl<'a> From<Decode<'a>> for Function<'a> {
    fn from(value: Decode<'a>) -> Self {
        Self {
            r#type: super::FunctionType::Decode(value),
            alias: None,
        }
    }
}
