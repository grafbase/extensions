use super::{StringId, TableId};

#[derive(serde::Deserialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum KeyType {
    Primary,
    Unique,
}

#[derive(Debug, Clone)]
pub struct Key<T> {
    pub(super) table_id: TableId,
    pub(super) r#type: KeyType,
    pub(super) constraint_name: T,
}

impl<T> Key<T> {
    pub(crate) fn table_id(&self) -> TableId {
        self.table_id
    }
}

impl Key<String> {
    pub fn new(table_id: TableId, constraint_name: String, r#type: KeyType) -> Self {
        Self {
            table_id,
            r#type,
            constraint_name,
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.constraint_name
    }
}

impl Key<StringId> {
    pub(crate) fn r#type(&self) -> KeyType {
        self.r#type
    }
}
