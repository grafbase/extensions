use super::{Function, FunctionType};
use crate::ast::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct JsonBuildArray<'a> {
    pub(crate) expressions: Vec<Expression<'a>>,
}

pub fn json_build_array<'a, E>(values: impl IntoIterator<Item = E>) -> Function<'a>
where
    E: Into<Expression<'a>>,
{
    let expressions = values.into_iter().map(Into::into).collect();
    let function = JsonBuildArray { expressions };

    function.into()
}

impl<'a> From<JsonBuildArray<'a>> for Function<'a> {
    fn from(value: JsonBuildArray<'a>) -> Self {
        Self {
            r#type: FunctionType::JsonBuildArray(value),
            alias: None,
        }
    }
}
