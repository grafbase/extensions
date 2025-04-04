use super::{TableId, names::StringId};

#[derive(serde::Deserialize, Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum KeyType {
    PRIMARY,
    UNIQUE,
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
    pub(crate) fn name(&self) -> StringId {
        self.constraint_name
    }

    pub(crate) fn r#type(&self) -> KeyType {
        self.r#type
    }
}
