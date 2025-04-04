use crate::ast::{Delete, Insert, Select, Update};

/// A database query
#[derive(Debug, Clone, PartialEq)]
pub enum Query<'a> {
    Select(Box<Select<'a>>),
    Insert(Box<Insert<'a>>),
    Update(Box<Update<'a>>),
    Delete(Box<Delete<'a>>),
}
