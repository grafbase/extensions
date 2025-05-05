use super::{TableColumnWalker, TableWalker, Walker, key_column::KeyColumnWalker};
use crate::{Key, KeyColumnId, KeyId, KeyType, StringId};

/// Defines a unique constraint in a table.
pub type KeyWalker<'a> = Walker<'a, KeyId>;

impl<'a> KeyWalker<'a> {
    /// The table of this constraint.
    pub fn table(self) -> TableWalker<'a> {
        self.walk(self.get().table_id())
    }

    /// The columns defining the unique value.
    pub fn columns(self) -> impl ExactSizeIterator<Item = KeyColumnWalker<'a>> + 'a {
        let range = super::range_for_key(&self.database_definition.key_columns, self.id, |column| column.key_id());

        range.map(move |id| self.walk(KeyColumnId(id as u32)))
    }

    /// True, if all the given columns are part of the constraint.
    pub fn has_all_the_columns(self, mut columns: impl ExactSizeIterator<Item = TableColumnWalker<'a>>) -> bool {
        columns.all(|left| self.columns().any(|right| left == right.table_column()))
    }

    /// True, if the constraint is the primary key of the table.
    pub fn is_primary(self) -> bool {
        matches!(self.get().r#type(), KeyType::Primary)
    }

    fn get(self) -> &'a Key<StringId> {
        &self.database_definition.keys[self.id.0 as usize]
    }
}
