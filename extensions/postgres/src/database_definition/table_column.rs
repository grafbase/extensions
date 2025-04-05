use super::{ColumnType, TableId, names::StringId};

#[derive(Debug, Clone)]
pub struct TableColumn<T> {
    pub(super) table_id: TableId,
    pub(super) database_name: T,
    pub(super) database_type: ColumnType,
    pub(super) client_name: T,
}

impl<T> TableColumn<T> {
    pub(crate) fn database_type(&self) -> ColumnType {
        self.database_type
    }

    pub(crate) fn table_id(&self) -> TableId {
        self.table_id
    }

    pub(crate) fn is_array(&self) -> bool {
        self.database_type.is_array()
    }
}

impl TableColumn<String> {
    pub fn new(table_id: TableId, database_name: String, client_name: String, database_type: ColumnType) -> Self {
        Self {
            table_id,
            database_name,
            database_type,
            client_name,
        }
    }

    pub(crate) fn database_name(&self) -> &str {
        &self.database_name
    }

    pub(crate) fn client_name(&self) -> &str {
        &self.client_name
    }
}

impl TableColumn<StringId> {
    pub(crate) fn database_name(&self) -> StringId {
        self.database_name
    }

    pub(crate) fn client_name(&self) -> StringId {
        self.client_name
    }
}
