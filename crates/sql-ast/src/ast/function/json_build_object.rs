use super::{Function, FunctionType};
use crate::ast::Expression;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub struct JsonBuildObject<'a> {
    pub(crate) values: Vec<(Cow<'a, str>, Expression<'a>)>,
}

pub fn json_build_object<'a, S, E>(values: impl IntoIterator<Item = (S, E)>) -> Function<'a>
where
    S: Into<Cow<'a, str>>,
    E: Into<Expression<'a>>,
{
    let values = values
        .into_iter()
        .map(|(name, value)| (name.into(), value.into()))
        .collect();

    let function = JsonBuildObject { values };

    function.into()
}

impl<'a> From<JsonBuildObject<'a>> for Function<'a> {
    fn from(value: JsonBuildObject<'a>) -> Self {
        Self {
            typ_: FunctionType::JsonBuildObject(value),
            alias: None,
        }
    }
}
