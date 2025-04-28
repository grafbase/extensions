mod back_relation;
mod r#enum;
mod enum_variant;
mod foreign_key;
mod foreign_key_column;
mod forward_relation;
mod key;
mod key_column;
mod relation;
mod table;
mod table_column;

use super::StringId;
use crate::DatabaseDefinition;

pub(crate) use foreign_key::ForeignKeyWalker;
pub(crate) use foreign_key_column::ForeignKeyColumnWalker;

pub use r#enum::EnumWalker;
pub use key::KeyWalker;
pub use relation::RelationWalker;
use std::ops::Range;
pub use table::TableWalker;
pub use table_column::TableColumnWalker;

/// An abstraction to iterate over an introspected PostgreSQL database.
///
/// The `Id` must be something that points to an object in the database.
#[derive(Clone, Copy)]
pub struct Walker<'a, Id> {
    pub(super) id: Id,
    pub(super) database_definition: &'a DatabaseDefinition,
}

impl<Id> PartialEq for Walker<'_, Id>
where
    Id: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<Id> Eq for Walker<'_, Id> where Id: Eq {}

impl<Id> std::hash::Hash for Walker<'_, Id>
where
    Id: std::hash::Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<'a, Id> Walker<'a, Id>
where
    Id: Copy,
{
    pub fn new(id: Id, database_definition: &'a DatabaseDefinition) -> Self {
        Self {
            id,
            database_definition,
        }
    }

    pub fn id(self) -> Id {
        self.id
    }

    fn walk<OtherId>(self, id: OtherId) -> Walker<'a, OtherId> {
        self.database_definition.walk(id)
    }

    fn get_name(self, id: StringId) -> &'a str {
        self.database_definition.interner.get(id)
    }
}

/// For a slice sorted by a key K, return the contiguous range of items matching the key.
fn range_for_key<I, K>(slice: &[I], key: K, extract: fn(&I) -> K) -> Range<usize>
where
    K: Copy + Ord + PartialOrd + PartialEq,
{
    let seed = slice.binary_search_by_key(&key, extract).unwrap_or(0);
    let mut iter = slice[..seed].iter();
    let start = match iter.rposition(|i| extract(i) != key) {
        None => 0,
        Some(other) => other + 1,
    };
    let mut iter = slice[seed..].iter();
    let end = seed + iter.position(|i| extract(i) != key).unwrap_or(slice.len() - seed);
    start..end
}
