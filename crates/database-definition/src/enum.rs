use inflector::Inflector;

use crate::StringId;

use super::SchemaId;

#[derive(Debug, Clone)]
pub struct Enum<T> {
    pub(super) schema_id: SchemaId,
    pub(super) database_name: T,
    pub(super) client_name: T,
    pub(super) description: Option<T>,
}

impl<T> Enum<T> {
    pub(crate) fn schema_id(&self) -> SchemaId {
        self.schema_id
    }

    pub(crate) fn set_client_name(&mut self, client_name: T) {
        self.client_name = client_name;
    }
}

impl Enum<String> {
    pub fn new(schema_id: SchemaId, database_name: String, client_name: Option<String>) -> Self {
        let client_name = client_name.unwrap_or_else(|| database_name.to_pascal_case());

        Self {
            schema_id,
            database_name,
            client_name,
            description: None,
        }
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn database_name(&self) -> &str {
        &self.database_name
    }

    pub fn client_name(&self) -> &str {
        &self.client_name
    }
}

impl Enum<StringId> {
    pub fn database_name(&self) -> StringId {
        self.database_name
    }

    pub fn client_name(&self) -> StringId {
        self.client_name
    }

    pub fn description(&self) -> Option<StringId> {
        self.description
    }
}
