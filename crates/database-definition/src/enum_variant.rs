use inflector::Inflector;

use super::{EnumId, names::StringId};

#[derive(Debug, Clone)]
pub struct EnumVariant<T> {
    pub(super) enum_id: EnumId,
    pub(super) database_name: T,
    pub(super) client_name: T,
}

impl<T> EnumVariant<T> {
    pub(crate) fn enum_id(&self) -> EnumId {
        self.enum_id
    }
}

impl EnumVariant<String> {
    pub fn new(enum_id: EnumId, database_name: String, client_name: Option<String>) -> Self {
        let client_name = client_name.unwrap_or_else(|| database_name.to_screaming_snake_case());

        Self {
            enum_id,
            database_name,
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

impl EnumVariant<StringId> {
    pub(crate) fn database_name(&self) -> StringId {
        self.database_name
    }

    pub(crate) fn client_name(&self) -> StringId {
        self.client_name
    }
}
