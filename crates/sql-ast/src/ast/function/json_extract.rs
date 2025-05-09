use super::Function;
use crate::ast::{Expression, FunctionType};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub struct JsonExtract<'a> {
    pub(crate) column: Box<Expression<'a>>,
    pub(crate) path: Vec<Cow<'a, str>>,
    pub(crate) extract_as_string: bool,
}

/// Extracts a subset of a JSON blob given a path.
/// Two types of paths can be used:
/// - `String` paths, referring to JSON paths. This is supported by MySQL only.
/// - `Array` paths, supported by Postgres only.
pub fn json_extract<'a, C>(column: C, path: Vec<Cow<'a, str>>, extract_as_string: bool) -> Function<'a>
where
    C: Into<Expression<'a>>,
{
    let fun = JsonExtract {
        column: Box::new(column.into()),
        path,
        extract_as_string,
    };

    fun.into()
}

impl<'a> From<JsonExtract<'a>> for Function<'a> {
    fn from(value: JsonExtract<'a>) -> Self {
        Self {
            r#type: FunctionType::JsonExtract(value),
            alias: None,
        }
    }
}
