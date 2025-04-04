use super::Function;
use crate::ast::{Expression, FunctionType};

#[derive(Debug, Clone, PartialEq)]
pub struct Cast<'a> {
    pub(crate) expr: Expression<'a>,
    pub(crate) target_type: &'static str,
}

/// Count of the underlying table where the given expression is not null.
pub fn cast<'a, T>(expr: T, target_type: &'static str) -> Function<'a>
where
    T: Into<Expression<'a>>,
{
    let fun = Cast {
        expr: expr.into(),
        target_type,
    };

    fun.into()
}

impl<'a> From<Cast<'a>> for Function<'a> {
    fn from(value: Cast<'a>) -> Self {
        Self {
            typ_: FunctionType::Cast(value),
            alias: None,
        }
    }
}
