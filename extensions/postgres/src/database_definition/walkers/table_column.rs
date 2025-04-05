use super::{TableWalker, Walker};
use crate::database_definition::{ColumnType, DatabaseType, TableColumn, TableColumnId, names::StringId};

/// Definition of a column located in a table.
pub type TableColumnWalker<'a> = Walker<'a, TableColumnId>;

impl<'a> TableColumnWalker<'a> {
    /// The table this column is located.
    pub fn table(self) -> TableWalker<'a> {
        self.walk(self.get().table_id())
    }

    /// The name of the column in the database.
    pub fn database_name(self) -> &'a str {
        self.get_name(self.get().database_name())
    }

    /// The name of the column in the GraphQL APIs.
    pub fn client_name(self) -> &'a str {
        self.get_name(self.get().client_name())
    }

    /// The type of the column in the database.
    pub fn database_type(self) -> DatabaseType<'a> {
        match self.get().database_type() {
            ColumnType::Scalar(scalar) => DatabaseType::Scalar(scalar),
            ColumnType::Enum(r#enum) => DatabaseType::Enum(self.walk(r#enum.id)),
        }
    }

    /// True, if the column is an array.
    pub fn is_array(self) -> bool {
        self.get().is_array()
    }

    fn get(self) -> &'a TableColumn<StringId> {
        &self.database_definition.table_columns[self.id.0 as usize]
    }
}
