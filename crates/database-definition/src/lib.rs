mod r#enum;
mod enum_variant;
mod foreign_key;
mod foreign_key_column;
mod ids;
mod key;
mod key_column;
mod names;
mod relations;
mod table;
mod table_column;
mod r#type;
mod walkers;

use std::collections::HashMap;

pub use r#enum::Enum;
pub use enum_variant::EnumVariant;
pub use foreign_key::ForeignKey;
pub use foreign_key_column::ForeignKeyColumn;
use grafbase_sdk::types::DefinitionId;
pub use ids::{
    BackRelationId, EnumId, EnumVariantId, ForeignKeyColumnId, ForeignKeyId, ForwardRelationId, KeyColumnId, KeyId,
    RelationId, SchemaId, TableColumnId, TableId,
};
pub use key::{Key, KeyType};
pub use key_column::KeyColumn;
use names::{Names, StringId};
use relations::Relations;
pub use table::Table;
pub use table_column::{IdentityGeneration, TableColumn};
pub use r#type::{ColumnType, DatabaseType, EnumType, ScalarKind, ScalarType};
pub use walkers::{EnumWalker, KeyWalker, RelationWalker, TableColumnWalker, TableWalker, Walker};

/// Definition of a PostgreSQL database. Contains all the
/// tables, enums, columns, constraints etc. for us to render
/// a GraphQL schema, and for us to allow querying the database
/// efficiently.
///
/// Due to Grafbase dependency tree, mutating this structure
/// outside of introspection is not recommended. Some of the
/// mutations are public, but from the perspective of the user,
/// the important call points are the table and enum iterators,
/// and the find methods with string slices.
///
/// Be aware that this structure is serialized in a cache for
/// fast worker startup. Any changes here must be backwards-compatible.
///
/// There will be a test failure if something changes to alert you.
#[derive(Debug)]
pub struct DatabaseDefinition {
    /// The name of the database.
    database_name: String,
    /// Ordered by name.
    schemas: Vec<String>,
    /// Ordered by schema id, then table name.
    tables: Vec<Table<StringId>>,
    /// Ordered by schema id, table id and then column position.
    table_columns: Vec<TableColumn<StringId>>,
    /// Ordered by schema id, then enum name.
    enums: Vec<Enum<StringId>>,
    /// Ordered by schema id, enum id and finally the variant position.
    enum_variants: Vec<EnumVariant<StringId>>,
    /// Ordered by schema id, table id and foreign key constraint name.
    foreign_keys: Vec<ForeignKey<StringId>>,
    /// Ordered by schema id, table id, foreign key id and the column position.
    foreign_key_columns: Vec<ForeignKeyColumn>,
    /// Ordered by schema id, table id and constraint name.
    keys: Vec<Key<StringId>>,
    /// Ordered by schema id, table id, constraint id and the column position.
    key_columns: Vec<KeyColumn>,
    names: Names,
    relations: Relations,
    /// A mapping from a field definition ID to a full SQL operation. Mapped from
    /// query and mutation definitions.
    operations: HashMap<DefinitionId, Operation>,
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    FindOne(TableId),
    FindMany(TableId),
    DeleteOne(TableId),
    DeleteMany(TableId),
    CreateOne(TableId),
    CreateMany(TableId),
    UpdateOne(TableId),
    UpdateMany(TableId),
}

impl DatabaseDefinition {
    /// Creates a new database definition with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the database
    ///
    /// # Returns
    ///
    /// A new `DatabaseDefinition` instance with default empty collections
    pub fn new(name: String) -> Self {
        Self {
            database_name: name,
            schemas: Vec::new(),
            tables: Vec::new(),
            table_columns: Vec::new(),
            enums: Vec::new(),
            enum_variants: Vec::new(),
            foreign_keys: Vec::new(),
            foreign_key_columns: Vec::new(),
            keys: Vec::new(),
            key_columns: Vec::new(),
            names: Names::default(),
            relations: Relations::default(),
            operations: HashMap::default(),
        }
    }

    /// The name of the database.
    pub fn name(&self) -> &str {
        &self.database_name
    }

    /// Iterates over all tables of the introspected database.
    pub fn tables(&self) -> impl ExactSizeIterator<Item = TableWalker<'_>> + '_ {
        (0..self.tables.len()).map(move |id| self.walk(TableId(id as u32)))
    }

    /// Iterates over all enums of the introspected database.
    pub fn enums(&self) -> impl ExactSizeIterator<Item = EnumWalker<'_>> + '_ {
        (0..self.enums.len()).map(move |id| self.walk(EnumId(id as u32)))
    }

    /// Find a table in a specified schema with the specified name.
    pub fn find_table(&self, schema_name: &str, table_name: &str) -> Option<TableWalker<'_>> {
        let schema_id = self.get_schema_id(schema_name)?;

        self.get_table_id(schema_id, table_name)
            .map(|table_id| self.walk(table_id))
    }

    /// Finds an enum in a specified schema with the specified name.
    pub fn find_enum(&self, schema_name: &str, enum_name: &str) -> Option<EnumWalker<'_>> {
        let schema_id = self.get_schema_id(schema_name)?;
        self.get_enum_id(schema_id, enum_name).map(|enum_id| self.walk(enum_id))
    }

    /// Find a table that represents the given client type.
    pub fn find_table_for_client_type(&self, client_type: &str) -> Option<TableWalker<'_>> {
        self.names
            .get_table_id_for_client_type(client_type)
            .map(|table_id| self.walk(table_id))
    }

    /// Find a column that represents the given client field.
    pub fn find_column_for_client_field(&self, client_field: &str, table_id: TableId) -> Option<TableColumnWalker<'_>> {
        self.names
            .get_column_id_for_client_field(client_field, table_id)
            .map(|table_id| self.walk(table_id))
    }

    /// Find a unique constraint that represents the given client field.
    pub fn find_unique_constraint_for_client_field(
        &self,
        client_field: &str,
        table_id: TableId,
    ) -> Option<KeyWalker<'_>> {
        self.names
            .get_unique_constraint_id_for_client_field(client_field, table_id)
            .map(|constraint_id| self.walk(constraint_id))
    }

    /// Retrieves a TableColumnWalker for a given definition ID.
    pub fn column_for_field_definition(&self, field_definition_id: DefinitionId) -> Option<TableColumnWalker<'_>> {
        self.names
            .get_column_id_for_definition_id(field_definition_id)
            .map(|id| self.walk(id))
    }

    /// Adds a schema to the definition.
    pub fn push_schema(&mut self, schema: String) -> SchemaId {
        let id = self.next_schema_id();
        self.schemas.push(schema);

        id
    }

    /// Adds an SQL operation for a given field definition ID.
    pub fn push_operation(&mut self, definition_id: DefinitionId, operation: Operation) {
        self.operations.insert(definition_id, operation);
    }

    pub fn get_operation(&self, definition_id: DefinitionId) -> Option<Operation> {
        self.operations.get(&definition_id).copied()
    }

    /// Adds a table to the definition.
    pub fn push_table(&mut self, table: Table<String>) -> TableId {
        let id = self.next_table_id();

        self.names.intern_table(&table, id);
        self.push_client_type_mapping(table.client_name(), id);

        self.tables.push(Table {
            schema_id: table.schema_id(),
            database_name: self.names.intern_string(table.database_name()),
            client_name: self.names.intern_string(table.client_name()),
            client_field_name: self.names.intern_string(table.client_field_name()),
            client_field_name_plural: self.names.intern_string(table.client_field_name_plural()),
        });

        id
    }

    /// Adds a table column to the definition.
    pub fn push_table_column(
        &mut self,
        column: TableColumn<String>,
        definition_id: Option<DefinitionId>,
    ) -> TableColumnId {
        let id = self.next_table_column_id();

        self.names.intern_table_column(&column, id);

        if let Some(definition_id) = definition_id {
            self.names.intern_field_column(definition_id, id);
        }

        self.push_client_field_mapping(column.client_name(), column.table_id(), id);

        self.table_columns.push(TableColumn {
            table_id: column.table_id(),
            database_name: self.names.intern_string(column.database_name()),
            database_type: column.database_type(),
            client_name: self.names.intern_string(column.client_name()),
            nullable: column.nullable,
            has_default: column.has_default,
            identity_generation: column.identity_generation,
        });

        id
    }

    /// Adds an enum to the definition.
    pub fn push_enum(&mut self, r#enum: Enum<String>) -> EnumId {
        let id = self.next_enum_id();

        self.names.intern_enum(&r#enum, id);

        self.enums.push(Enum {
            schema_id: r#enum.schema_id(),
            database_name: self.names.intern_string(r#enum.database_name()),
            client_name: self.names.intern_string(r#enum.client_name()),
        });

        id
    }

    /// Adds an enum variant to the definition.
    pub fn push_enum_variant(&mut self, enum_variant: EnumVariant<String>) -> EnumVariantId {
        let id = self.next_enum_variant_id();

        self.names.intern_enum_variant(&enum_variant, id);

        self.enum_variants.push(EnumVariant {
            enum_id: enum_variant.enum_id(),
            database_name: self.names.intern_string(enum_variant.database_name()),
            client_name: self.names.intern_string(enum_variant.client_name()),
        });

        id
    }

    /// Adds a foreign key to the definition.
    pub fn push_foreign_key(
        &mut self,
        foreign_key: ForeignKey<String>,
    ) -> (ForeignKeyId, ForwardRelationId, BackRelationId) {
        let id = self.next_foreign_key_id();

        let (forward, back) = self.relations.push_relation(&foreign_key, id);
        self.names.intern_foreign_key(&foreign_key, id);

        self.foreign_keys.push(ForeignKey {
            constraint_name: self.names.intern_string(foreign_key.constraint_name()),
            schema_id: foreign_key.schema_id(),
            constrained_table_id: foreign_key.constrained_table_id(),
            referenced_table_id: foreign_key.referenced_table_id(),
        });

        (id, forward, back)
    }

    /// Adds a foreign key column to the definition.
    pub fn push_foreign_key_column(&mut self, foreign_key_column: ForeignKeyColumn) -> ForeignKeyColumnId {
        let id = self.next_foreign_key_column_id();
        self.foreign_key_columns.push(foreign_key_column);

        id
    }

    /// Adds a unique constraint to the definition.
    pub fn push_key(&mut self, unique_constraint: Key<String>) -> KeyId {
        let id = self.next_key_id();
        self.names.intern_key(&unique_constraint, id);

        self.keys.push(Key {
            table_id: unique_constraint.table_id(),
            constraint_name: self.names.intern_string(unique_constraint.name()),
            r#type: unique_constraint.r#type,
        });

        id
    }

    /// Adds a unique constraint column to the definition.
    pub fn push_key_column(&mut self, key_column: KeyColumn) -> KeyColumnId {
        let id = self.next_key_column_id();

        self.key_columns.push(key_column);

        id
    }

    /// Adds an index from client type name to table id.
    pub fn push_client_type_mapping(&mut self, type_name: &str, table_id: TableId) {
        self.names.intern_client_type(type_name, table_id);
    }

    /// Adds an index from client field name and table id to table column id.
    pub fn push_client_field_mapping(&mut self, field_name: &str, table_id: TableId, column_id: TableColumnId) {
        self.names.intern_client_field(field_name, table_id, column_id);
    }

    /// Adds an index from client field name and table id to unique constraint id.
    pub fn push_client_field_key_mapping(&mut self, field_name: &str, table_id: TableId, constraint_id: KeyId) {
        self.names.intern_client_key(field_name, table_id, constraint_id);
    }

    /// Adds an index from client enum name to the corresponding enum id.
    pub fn push_client_enum_mapping(&mut self, enum_name: &str, enum_id: EnumId) {
        self.names.intern_client_enum(enum_name, enum_id);
    }

    pub fn push_client_definition_to_name(&mut self, field_name: &str, definition_id: DefinitionId) {
        self.names.intern_definition_to_field_name(field_name, definition_id);
    }

    pub fn push_client_field_definition_to_return_type_definition_id(
        &mut self,
        field_definition_id: DefinitionId,
        return_type_definition_id: DefinitionId,
    ) {
        self.names.intern_client_field_definition_to_return_type_definition_id(
            field_definition_id,
            return_type_definition_id,
        );
    }

    pub fn push_client_id_relation_mapping(&mut self, field_definition_id: DefinitionId, relation_id: RelationId) {
        self.names.intern_client_id_relation(field_definition_id, relation_id);
    }

    pub fn push_client_name_relation_mapping(&mut self, table_id: TableId, name: &str, relation_id: RelationId) {
        self.names.intern_client_name_relation(table_id, name, relation_id);
    }

    pub fn get_name_for_field_definition(&self, field_definition_id: DefinitionId) -> Option<&str> {
        self.names.get_field_name_with_definition_id(field_definition_id)
    }

    /// Gets the relation ID for a client field.
    pub fn get_relation_id_for_client_field_id(&self, definition_id: DefinitionId) -> Option<RelationId> {
        self.names.get_relation_id_for_client_field_id(definition_id)
    }

    pub fn get_relation_for_client_name(&self, table_id: TableId, field_name: &str) -> Option<RelationWalker<'_>> {
        self.names
            .get_relation_id_for_client_name(table_id, field_name)
            .map(|id| self.walk(id))
    }

    pub fn get_output_table_for_field_definition(&self, field_definition_id: DefinitionId) -> Option<TableWalker> {
        self.names
            .find_return_table_column_for_field_definition_id(field_definition_id)
            .map(|id| self.walk(id).table())
    }

    /// Finds the id of a schema with the given name, if existing.
    pub fn get_schema_id(&self, schema: &str) -> Option<SchemaId> {
        self.schemas
            .binary_search_by(|schema_name| schema_name.as_str().cmp(schema))
            .ok()
            .map(|position| SchemaId(position as u32))
    }

    /// Finds the id of a table with the given name, if existing.
    pub fn get_table_id(&self, schema_id: SchemaId, table_name: &str) -> Option<TableId> {
        self.names.get_table_id(schema_id, table_name)
    }

    /// Finds the id of a column in a table with the given name, if existing.
    pub fn get_table_column_id(&self, table_id: TableId, column_name: &str) -> Option<TableColumnId> {
        self.names.get_table_column_id(table_id, column_name)
    }

    /// Finds the id of an enum with the given name, if existing.
    pub fn get_enum_id(&self, schema_id: SchemaId, enum_name: &str) -> Option<EnumId> {
        self.names.get_enum_id(schema_id, enum_name)
    }

    /// Finds the id of an enum with the given name, if existing.
    pub fn get_foreign_key_id(&self, schema_id: SchemaId, constraint_name: &str) -> Option<ForeignKeyId> {
        self.names.get_foreign_key_id(schema_id, constraint_name)
    }

    /// Finds the id of a unique constraint with the given name, if existing.
    pub fn get_unique_constraint_id(&self, table_id: TableId, constraint_name: &str) -> Option<KeyId> {
        self.names.get_unique_constraint_id(table_id, constraint_name)
    }

    /// Finalizes the definition. Handles name deduplication, and sorts the internal data structures
    /// accordingly.
    pub fn finalize(&mut self) {
        self.relations.from.sort_by_key(|(table_id, _)| *table_id);
        self.relations.to.sort_by_key(|(table_id, _)| *table_id);
    }

    /// Walk an item in the definition by its ID.
    pub fn walk<Id>(&self, id: Id) -> Walker<'_, Id> {
        Walker {
            id,
            database_definition: self,
        }
    }

    fn next_schema_id(&self) -> SchemaId {
        SchemaId(self.schemas.len() as u32)
    }

    fn next_table_id(&self) -> TableId {
        TableId(self.tables.len() as u32)
    }

    fn next_table_column_id(&self) -> TableColumnId {
        TableColumnId(self.table_columns.len() as u32)
    }

    fn next_enum_id(&self) -> EnumId {
        EnumId(self.enums.len() as u32)
    }

    fn next_enum_variant_id(&self) -> EnumVariantId {
        EnumVariantId(self.enum_variants.len() as u32)
    }

    fn next_foreign_key_id(&self) -> ForeignKeyId {
        ForeignKeyId(self.foreign_keys.len() as u32)
    }

    fn next_foreign_key_column_id(&self) -> ForeignKeyColumnId {
        ForeignKeyColumnId(self.foreign_key_columns.len() as u32)
    }

    fn next_key_id(&self) -> KeyId {
        KeyId(self.keys.len() as u32)
    }

    fn next_key_column_id(&self) -> KeyColumnId {
        KeyColumnId(self.key_columns.len() as u32)
    }
}
