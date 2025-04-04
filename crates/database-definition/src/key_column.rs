use super::{KeyId, TableColumnId};

#[derive(Debug, Clone)]
pub struct KeyColumn {
    key_id: KeyId,
    column_id: TableColumnId,
}

impl KeyColumn {
    pub fn new(key_id: KeyId, column_id: TableColumnId) -> Self {
        Self { key_id, column_id }
    }

    pub(crate) fn key_id(&self) -> KeyId {
        self.key_id
    }

    pub(crate) fn column_id(&self) -> TableColumnId {
        self.column_id
    }
}
