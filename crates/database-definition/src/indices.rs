use std::collections::HashMap;

use grafbase_sdk::types::DefinitionId;

use crate::{Operation, StringId};

use super::{EnumId, EnumVariantId, ForeignKeyId, KeyId, RelationId, SchemaId, TableColumnId, TableId};

/// Various indices used to quickly look up items within the schema.
#[derive(Default, Debug, Clone)]
pub(super) struct Indices {
    /// Provides a fast lookup for a table by its schema ID and name.
    pub(super) tables: HashMap<(SchemaId, StringId), TableId>,
    /// Provides a fast lookup for a table column by its table ID and column name.
    pub(super) table_columns: HashMap<(TableId, StringId), TableColumnId>,
    /// Provides a fast lookup for a table column by its table ID and *client* field name.
    /// Note: This might be redundant with `client_fields`.
    pub(super) table_fields: HashMap<(TableId, StringId), TableColumnId>,
    /// Provides a fast lookup for an enum by its schema ID and name.
    pub(super) enums: HashMap<(SchemaId, StringId), EnumId>,
    /// Provides a fast lookup for an enum variant by its enum ID and variant name.
    pub(super) enum_variants: HashMap<(EnumId, StringId), EnumVariantId>,
    /// Provides a fast lookup for a foreign key by its schema ID and name.
    pub(super) foreign_keys: HashMap<(SchemaId, StringId), ForeignKeyId>,
    /// Provides a fast lookup for a key (e.g., primary, unique) by its table ID and name.
    pub(super) keys: HashMap<(TableId, StringId), KeyId>,
    /// Provides a fast lookup for a table ID by its client-facing type name.
    pub(super) client_types: HashMap<StringId, TableId>,
    /// Provides a fast lookup for a table column ID by its table ID and client-facing field name.
    pub(super) client_fields: HashMap<(TableId, StringId), TableColumnId>,
    /// Provides a fast lookup for a key ID by its table ID and client-facing unique constraint name.
    pub(super) client_unique_constraints: HashMap<(TableId, StringId), KeyId>,
    /// Provides a fast lookup for a relation ID by the `DefinitionId` of the client-facing relation field.
    pub(super) client_relations: HashMap<DefinitionId, RelationId>,
    /// Provides a fast lookup for a relation ID by its table ID and client-facing relation field name.
    pub(super) client_name_relations: HashMap<(TableId, StringId), RelationId>,
    /// Maps the `DefinitionId` of a client-facing scalar field to its corresponding `TableColumnId`.
    pub(super) field_definition_to_column: HashMap<DefinitionId, TableColumnId>,
    /// Maps a `DefinitionId` (e.g., of a field) to its client-facing name (`StringId`).
    pub(super) definition_to_field_name: HashMap<DefinitionId, StringId>,
    /// Maps the `DefinitionId` of a client-facing field to the `DefinitionId` of its return type.
    pub(super) field_definition_to_return_type_definition_id: HashMap<DefinitionId, DefinitionId>,
    /// A mapping from a field definition ID to a full SQL operation. Mapped from
    /// query and mutation definitions.
    pub(super) operations: HashMap<DefinitionId, Operation>,
}
