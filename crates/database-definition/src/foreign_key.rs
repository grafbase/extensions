use super::{SchemaId, StringId, TableId};

#[derive(Debug, Clone)]
pub struct ForeignKey<T> {
    pub(super) constraint_name: T,
    pub(super) schema_id: SchemaId,
    pub(super) constrained_table_id: TableId,
    pub(super) referenced_table_id: TableId,
    pub(super) description: Option<T>,
}

impl<T> ForeignKey<T> {
    pub(crate) fn schema_id(&self) -> SchemaId {
        self.schema_id
    }

    pub(crate) fn constrained_table_id(&self) -> TableId {
        self.constrained_table_id
    }

    pub(crate) fn referenced_table_id(&self) -> TableId {
        self.referenced_table_id
    }
}

impl ForeignKey<String> {
    pub fn new(
        constraint_name: String,
        schema_id: SchemaId,
        constrained_table_id: TableId,
        referenced_table_id: TableId,
    ) -> Self {
        Self {
            constraint_name,
            schema_id,
            constrained_table_id,
            referenced_table_id,
            description: None,
        }
    }

    pub(crate) fn constraint_name(&self) -> &str {
        &self.constraint_name
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }
}

impl ForeignKey<StringId> {
    pub(crate) fn constraint_name(&self) -> StringId {
        self.constraint_name
    }

    pub fn description(&self) -> Option<StringId> {
        self.description
    }
}
