use std::borrow::Cow;

use super::Column;

#[derive(Debug, Clone, PartialEq)]
pub struct Alias<'a> {
    pub(crate) name: Cow<'a, str>,
    pub(crate) columns: Vec<Column<'a>>,
}

impl<'a> Alias<'a> {
    pub fn new(name: impl Into<Cow<'a, str>>) -> Self {
        Alias {
            name: name.into(),
            columns: Vec::new(),
        }
    }

    pub fn push_column(&mut self, column: impl Into<Column<'a>>) {
        self.columns.push(column.into());
    }
}

impl<'a, T> From<T> for Alias<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(alias: T) -> Self {
        Self {
            name: alias.into(),
            columns: Vec::new(),
        }
    }
}

/// An object that can be aliased.
pub trait Aliasable<'a> {
    type Target;

    /// Alias table for usage elsewhere in the query.
    fn alias<T>(self, alias: T) -> Self::Target
    where
        T: Into<Alias<'a>>;
}
