use crate::ast::{Expression, Function, FunctionType};

#[derive(Debug, Clone)]
pub struct JsonExtractArrayElem<'a> {
    pub(crate) expr: Box<Expression<'a>>,
    pub(crate) index: usize,
}

/// This is an internal function used to help construct the JsonArrayEndsInto Comparable
pub fn json_extract_array_elem<'a, E>(expr: E, index: usize) -> Function<'a>
where
    E: Into<Expression<'a>>,
{
    let fun = JsonExtractArrayElem {
        expr: Box::new(expr.into()),
        index,
    };

    fun.into()
}

impl<'a> From<JsonExtractArrayElem<'a>> for Function<'a> {
    fn from(value: JsonExtractArrayElem<'a>) -> Self {
        Self {
            r#type: FunctionType::JsonExtractArrayElem(value),
            alias: None,
        }
    }
}
