use super::Function;
use crate::ast::Expression;

/// A representation of the `decode` function in PostgreSQL.
#[derive(Debug, Clone)]
pub struct ConvertFrom<'a> {
    pub(crate) expression: Expression<'a>,
    pub(crate) charset: &'static str,
}

/// Decode a given expression into a specified format.
pub fn convert_from<'a>(expression: impl Into<Expression<'a>>, charset: &'static str) -> Function<'a> {
    let fun = ConvertFrom {
        expression: expression.into(),
        charset,
    };

    fun.into()
}

impl<'a> From<ConvertFrom<'a>> for Function<'a> {
    fn from(value: ConvertFrom<'a>) -> Self {
        Self {
            r#type: super::FunctionType::ConvertFrom(value),
            alias: None,
        }
    }
}
