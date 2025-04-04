use super::{Walker, key::KeyWalker, table_column::TableColumnWalker};
use crate::{KeyColumn, KeyColumnId};

/// A column that is part of a unique constraint.
pub type KeyColumnWalker<'a> = Walker<'a, KeyColumnId>;

impl<'a> KeyColumnWalker<'a> {
    /// The constraint this column is part of.
    pub fn key(self) -> KeyWalker<'a> {
        self.walk(self.get().key_id())
    }

    /// The column in the table this column refers to.
    pub fn table_column(self) -> TableColumnWalker<'a> {
        self.walk(self.get().column_id())
    }

    fn get(self) -> &'a KeyColumn {
        &self.database_definition.key_columns[self.id.0 as usize]
    }
}
