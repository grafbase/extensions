use std::borrow::Cow;

use super::Query;

#[derive(Debug, PartialEq, Clone)]
pub struct CommonTableExpression<'a> {
    pub(crate) name: Cow<'a, str>,
    pub(crate) query: Query<'a>,
}

impl<'a> CommonTableExpression<'a> {
    pub fn new(name: impl Into<Cow<'a, str>>, query: impl Into<Query<'a>>) -> Self {
        Self {
            name: name.into(),
            query: query.into(),
        }
    }
}
