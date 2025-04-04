use crate::ast::{Expression, Function, FunctionType};

/// Represents an `UNNEST` function call.
#[derive(Debug, Clone, PartialEq)]
pub struct Unnest<'a> {
    pub(crate) expression: Box<Expression<'a>>,
}

/// Creates an `UNNEST` function call.
///
/// `UNNEST` expands an array or map into a relation. Arrays are unnested into a
/// single column, and maps are unnested into two columns (key, value).
/// Used mainly in PostgreSQL, but some other databases might support standard
/// compliant `UNNEST` or similar functions.
pub fn unnest<'a, E>(expression: E) -> Function<'a>
where
    E: Into<Expression<'a>>,
{
    let fun = Unnest {
        expression: Box::new(expression.into()),
    };

    fun.into()
}

impl<'a> From<Unnest<'a>> for Function<'a> {
    fn from(value: Unnest<'a>) -> Self {
        Self {
            typ_: FunctionType::Unnest(value),
            alias: None,
        }
    }
}
