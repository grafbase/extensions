use std::str::FromStr;

use grafbase_sdk::SdkError;
use inflector::Inflector;

use super::{ColumnType, StringId, TableId};

#[derive(Debug, Clone, Copy)]
pub enum IdentityGeneration {
    /// Cannot insert a custom value to the column, always generated.
    Always,
    /// Can optionally insert a custom value to the column, by default generated.
    ByDefault,
}

impl FromStr for IdentityGeneration {
    type Err = SdkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "always" => Ok(IdentityGeneration::Always),
            "by default" => Ok(IdentityGeneration::ByDefault),
            _ => Err(SdkError::from(format!("Invalid identity generation: {}", s))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TableColumn<T> {
    pub(super) table_id: TableId,
    pub(super) database_name: T,
    pub(super) database_type: ColumnType,
    pub(super) client_name: T,
    pub(super) nullable: bool,
    pub(super) has_default: bool,
    pub(super) identity_generation: Option<IdentityGeneration>,
    pub(super) description: Option<T>,
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

    pub fn set_nullable(&mut self, value: bool) {
        self.nullable = value;
    }

    pub fn set_has_default(&mut self, value: bool) {
        self.has_default = value;
    }

    pub fn identity_generation(&self) -> Option<IdentityGeneration> {
        self.identity_generation
    }

    pub fn set_identity_generation(&mut self, value: IdentityGeneration) {
        self.identity_generation = Some(value);
    }

    pub fn set_description(&mut self, value: T) {
        self.description = Some(value);
    }
}

impl TableColumn<String> {
    pub fn new(
        table_id: TableId,
        database_type: ColumnType,
        database_name: String,
        client_name: Option<String>,
    ) -> Self {
        let client_name = client_name.unwrap_or_else(|| database_name.to_camel_case());

        Self {
            table_id,
            database_name,
            database_type,
            client_name,
            nullable: false,
            has_default: false,
            identity_generation: None,
            description: None,
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

    pub fn description(&self) -> Option<StringId> {
        self.description
    }
}
