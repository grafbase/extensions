use inflector::Inflector;

use super::{SchemaId, StringId};

#[derive(Debug, Clone)]
pub struct Table<T> {
    pub(super) schema_id: SchemaId,
    pub(super) database_name: T,
    pub(super) client_name: T,
    pub(super) client_field_name: T,
    pub(super) client_field_name_plural: T,
    pub(super) kind: RelationKind,
    pub(super) description: Option<T>,
}

#[derive(serde::Deserialize, Debug, Clone, Copy, Default, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RelationKind {
    #[default]
    Relation,
    View,
    MaterializedView,
}

impl RelationKind {
    pub fn client_name(self) -> &'static str {
        match self {
            RelationKind::Relation => "RELATION",
            RelationKind::View => "VIEW",
            RelationKind::MaterializedView => "MATERIALIZED_VIEW",
        }
    }
}

impl<T> Copy for Table<T> where T: Copy {}

impl<T> Table<T> {
    pub(crate) fn schema_id(&self) -> SchemaId {
        self.schema_id
    }

    pub(super) fn set_client_name(&mut self, client_name: T) {
        self.client_name = client_name;
    }

    pub(super) fn set_client_field_name(&mut self, client_field_name: T) {
        self.client_field_name = client_field_name;
    }

    pub(super) fn set_client_field_name_plural(&mut self, client_field_name_plural: T) {
        self.client_field_name_plural = client_field_name_plural;
    }

    pub fn set_description(&mut self, description: T) {
        self.description = Some(description);
    }

    pub(crate) fn kind(&self) -> RelationKind {
        self.kind
    }
}

impl Table<String> {
    pub fn new(schema_id: SchemaId, database_name: String, kind: RelationKind, client_name: Option<String>) -> Self {
        let client_name = client_name.unwrap_or_else(|| database_name.to_pascal_case().to_singular());
        let client_field_name = client_name.to_camel_case();
        let client_field_name_plural = client_field_name.to_plural();

        Self {
            schema_id,
            database_name,
            client_name,
            client_field_name,
            client_field_name_plural,
            kind,
            description: None,
        }
    }

    pub(crate) fn database_name(&self) -> &str {
        &self.database_name
    }

    pub(crate) fn client_name(&self) -> &str {
        &self.client_name
    }

    pub(crate) fn client_field_name(&self) -> &str {
        &self.client_field_name
    }

    pub(crate) fn client_field_name_plural(&self) -> &str {
        &self.client_field_name_plural
    }
}

impl Table<StringId> {
    pub(crate) fn database_name(&self) -> StringId {
        self.database_name
    }

    pub(crate) fn client_name(&self) -> StringId {
        self.client_name
    }

    pub(crate) fn client_field_name(&self) -> StringId {
        self.client_field_name
    }

    pub(crate) fn client_field_name_plural(&self) -> StringId {
        self.client_field_name_plural
    }

    pub(crate) fn description(&self) -> Option<StringId> {
        self.description
    }
}
