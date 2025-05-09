use crate::ast::{Expression, Function, FunctionType};

/// Represents an `UNNEST` function call.
#[derive(Debug, Clone, PartialEq)]
pub struct Unnest<'a> {
    pub(crate) expression: Box<Expression<'a>>,
    pub(crate) with_ordinality: bool,
}

/// Creates an `UNNEST` function call.
///
/// `UNNEST` expands an array or map into a relation. Arrays are unnested into a
/// single column, and maps are unnested into two columns (key, value).
/// Used mainly in PostgreSQL, but some other databases might support standard
/// compliant `UNNEST` or similar functions.
pub fn unnest<'a, E>(expression: E, with_ordinality: bool) -> Function<'a>
where
    E: Into<Expression<'a>>,
{
    let fun = Unnest {
        expression: Box::new(expression.into()),
        with_ordinality,
    };

    fun.into()
}

impl<'a> From<Unnest<'a>> for Function<'a> {
    fn from(value: Unnest<'a>) -> Self {
        Self {
            r#type: FunctionType::Unnest(value),
            alias: None,
        }
    }
}
