use super::Function;
use crate::ast::{Expression, FunctionType};

#[derive(Debug, Clone, PartialEq)]
/// An aggregate function that concatenates strings from a group into a single
/// string with various options.
pub struct AggregateToString<'a> {
    pub(crate) value: Box<Expression<'a>>,
}

/// Aggregates the given field into a string.
pub fn aggregate_to_string<'a, T>(expr: T) -> Function<'a>
where
    T: Into<Expression<'a>>,
{
    let fun = AggregateToString {
        value: Box::new(expr.into()),
    };

    fun.into()
}

impl<'a> From<AggregateToString<'a>> for Function<'a> {
    fn from(value: AggregateToString<'a>) -> Self {
        Self {
            typ_: FunctionType::AggregateToString(value),
            alias: None,
        }
    }
}
