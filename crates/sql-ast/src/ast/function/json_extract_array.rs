use crate::ast::{Expression, Function, FunctionType};

#[derive(Debug, Clone, PartialEq)]
pub struct JsonExtractLastArrayElem<'a> {
    pub(crate) expr: Box<Expression<'a>>,
}

/// This is an internal function used to help construct the JsonArrayEndsInto Comparable
pub(crate) fn json_extract_last_array_elem<'a, E>(expr: E) -> Function<'a>
where
    E: Into<Expression<'a>>,
{
    let fun = JsonExtractLastArrayElem {
        expr: Box::new(expr.into()),
    };

    fun.into()
}

impl<'a> From<JsonExtractLastArrayElem<'a>> for Function<'a> {
    fn from(value: JsonExtractLastArrayElem<'a>) -> Self {
        Self {
            typ_: FunctionType::JsonExtractLastArrayElem(value),
            alias: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct JsonExtractFirstArrayElem<'a> {
    pub(crate) expr: Box<Expression<'a>>,
}

/// This is an internal function used to help construct the JsonArrayBeginsWith Comparable
pub(crate) fn json_extract_first_array_elem<'a, E>(expr: E) -> Function<'a>
where
    E: Into<Expression<'a>>,
{
    let fun = JsonExtractFirstArrayElem {
        expr: Box::new(expr.into()),
    };

    fun.into()
}

impl<'a> From<JsonExtractFirstArrayElem<'a>> for Function<'a> {
    fn from(value: JsonExtractFirstArrayElem<'a>) -> Self {
        Self {
            typ_: FunctionType::JsonExtractFirstArrayElem(value),
            alias: None,
        }
    }
}
