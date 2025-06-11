use super::Function;
use crate::ast::Expression;

/// The encode format.
#[derive(Debug, Clone, Copy)]
pub enum EncodeFormat {
    Base64,
    Escape,
    Hex,
}

/// A representation of the `encode` function in PostgreSQL.
#[derive(Debug, Clone)]
pub struct Encode<'a> {
    pub(crate) expression: Expression<'a>,
    pub(crate) format: EncodeFormat,
}

/// Encode a given expression into a specified format.
pub fn encode<'a>(expression: impl Into<Expression<'a>>, format: EncodeFormat) -> Function<'a> {
    let fun = Encode {
        expression: expression.into(),
        format,
    };

    fun.into()
}

impl<'a> From<Encode<'a>> for Function<'a> {
    fn from(value: Encode<'a>) -> Self {
        Self {
            r#type: super::FunctionType::Encode(value),
            alias: None,
        }
    }
}
