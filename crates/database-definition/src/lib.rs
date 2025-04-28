mod r#enum;
mod enum_variant;
mod foreign_key;
mod foreign_key_column;
mod ids;
mod indices;
mod interner;
mod key;
mod key_column;
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
use indices::Indices;
use inflector::Inflector;
pub(crate) use interner::StringId;
use interner::StringInterner;
pub use key::{Key, KeyType};
pub use key_column::KeyColumn;
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
    /// Used for deduplicating strings in the definition.
    interner: StringInterner,
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
    /// Various indices for faster lookups.
    indices: Indices,
    /// Stores the relations between tables based on foreign keys.
    relations: Relations,
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    FindOne(TableId),
    FindMany(TableId),
    Lookup(TableId),
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
            indices: Indices::default(),
            relations: Relations::default(),
            interner: Default::default(),
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

    /// Find a table that represents the given client type.
    pub fn find_table_for_client_type(&self, client_type: &str) -> Option<TableWalker<'_>> {
        self.interner
            .lookup(client_type)
            .and_then(|string_id| self.indices.client_types.get(&string_id))
            .map(|id| self.walk(*id))
    }

    /// Find a unique constraint that represents the given client field.
    pub fn find_unique_constraint_for_client_field(
        &self,
        client_field: &str,
        table_id: TableId,
    ) -> Option<KeyWalker<'_>> {
        self.interner
            .lookup(client_field)
            .and_then(|string_id| self.indices.client_unique_constraints.get(&(table_id, string_id)))
            .copied()
            .map(|id| self.walk(id))
    }

    /// Retrieves a TableColumnWalker for a given definition ID.
    pub fn column_for_field_definition(&self, field_definition_id: DefinitionId) -> Option<TableColumnWalker<'_>> {
        self.indices
            .field_definition_to_column
            .get(&field_definition_id)
            .copied()
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
        self.indices.operations.insert(definition_id, operation);
    }

    pub fn get_operation(&self, definition_id: DefinitionId) -> Option<Operation> {
        self.indices.operations.get(&definition_id).copied()
    }

    /// Adds a table to the definition.
    pub fn push_table(&mut self, table: Table<String>) -> TableId {
        let id = self.next_table_id();
        let string_id = self.interner.intern(table.database_name());

        self.indices.tables.insert((table.schema_id(), string_id), id);
        self.push_client_type_mapping(table.client_name(), id);

        self.tables.push(Table {
            schema_id: table.schema_id(),
            database_name: self.interner.intern(table.database_name()),
            client_name: self.interner.intern(table.client_name()),
            client_field_name: self.interner.intern(table.client_field_name()),
            client_field_name_plural: self.interner.intern(table.client_field_name_plural()),
            description: table.description.map(|desc| self.interner.intern(&desc)),
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

        let string_id = self.interner.intern(column.database_name());
        self.indices.table_columns.insert((column.table_id(), string_id), id);

        if column.database_name() != column.client_name() {
            let string_id = self.interner.intern(column.client_name());
            self.indices.table_fields.insert((column.table_id(), string_id), id);

            let string_id = self.interner.intern(column.client_name());
            self.indices.table_columns.insert((column.table_id(), string_id), id);
        }

        if let Some(definition_id) = definition_id {
            self.push_column_to_definition(definition_id, id);
        }

        self.push_client_field_mapping(column.client_name(), column.table_id(), id);

        self.table_columns.push(TableColumn {
            table_id: column.table_id(),
            database_name: self.interner.intern(column.database_name()),
            database_type: column.database_type(),
            client_name: self.interner.intern(column.client_name()),
            nullable: column.nullable,
            has_default: column.has_default,
            identity_generation: column.identity_generation,
            description: column.description.map(|d| self.interner.intern(&d)),
        });

        id
    }

    /// Associates a column with a field definition in the GraphQL schema.
    pub fn push_column_to_definition(&mut self, definition_id: DefinitionId, column_id: TableColumnId) {
        self.indices.field_definition_to_column.insert(definition_id, column_id);
    }

    /// Adds an enum to the definition.
    pub fn push_enum(&mut self, r#enum: Enum<String>) -> EnumId {
        let id = self.next_enum_id();

        let string_id = self.interner.intern(r#enum.database_name());
        self.indices.enums.insert((r#enum.schema_id(), string_id), id);

        if r#enum.database_name() != r#enum.client_name() {
            let string_id = self.interner.intern(r#enum.client_name());
            self.indices.enums.insert((r#enum.schema_id(), string_id), id);
        }

        self.enums.push(Enum {
            schema_id: r#enum.schema_id(),
            database_name: self.interner.intern(r#enum.database_name()),
            client_name: self.interner.intern(r#enum.client_name()),
            description: r#enum.description.map(|d| self.interner.intern(&d)),
        });

        id
    }

    /// Adds an enum variant to the definition.
    pub fn push_enum_variant(&mut self, enum_variant: EnumVariant<String>) -> EnumVariantId {
        let id = self.next_enum_variant_id();
        let string_id = self.interner.intern(enum_variant.database_name());

        self.indices
            .enum_variants
            .insert((enum_variant.enum_id(), string_id), id);

        self.enum_variants.push(EnumVariant {
            enum_id: enum_variant.enum_id(),
            database_name: self.interner.intern(enum_variant.database_name()),
            client_name: self.interner.intern(enum_variant.client_name()),
            description: enum_variant.description.map(|d| self.interner.intern(&d)),
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
        let string_id = self.interner.intern(foreign_key.constraint_name());

        self.indices
            .foreign_keys
            .insert((foreign_key.schema_id(), string_id), id);

        self.foreign_keys.push(ForeignKey {
            constraint_name: self.interner.intern(foreign_key.constraint_name()),
            schema_id: foreign_key.schema_id(),
            constrained_table_id: foreign_key.constrained_table_id(),
            referenced_table_id: foreign_key.referenced_table_id(),
            description: foreign_key.description.map(|d| self.interner.intern(&d)),
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
    pub fn push_key(&mut self, key: Key<String>) -> KeyId {
        let id = self.next_key_id();
        let string_id = self.interner.intern(key.name());

        self.indices.keys.insert((key.table_id(), string_id), id);

        self.keys.push(Key {
            table_id: key.table_id(),
            constraint_name: self.interner.intern(key.name()),
            r#type: key.r#type,
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
        let string_id = self.interner.intern(type_name);
        self.indices.client_types.insert(string_id, table_id);
    }

    /// Adds an index from client field name and table id to table column id.
    pub fn push_client_field_mapping(&mut self, field_name: &str, table_id: TableId, column_id: TableColumnId) {
        let string_id = self.interner.intern(field_name);
        self.indices.client_fields.insert((table_id, string_id), column_id);
    }

    /// Adds an index from client field name and table id to unique constraint id.
    pub fn push_client_field_key_mapping(&mut self, field_name: &str, table_id: TableId, constraint_id: KeyId) {
        let string_id = self.interner.intern(field_name);

        self.indices
            .client_unique_constraints
            .insert((table_id, string_id), constraint_id);
    }

    /// Adds an index from a field definition ID to its name.
    pub fn push_client_definition_to_name(&mut self, field_name: &str, definition_id: DefinitionId) {
        let string_id = self.interner.intern(field_name);
        self.indices.definition_to_field_name.insert(definition_id, string_id);
    }

    /// Adds an index from a field definition ID to its return type definition ID.
    pub fn push_field_definition_to_type_definition(
        &mut self,
        field_definition_id: DefinitionId,
        return_type_definition_id: DefinitionId,
    ) {
        self.indices
            .field_definition_to_return_type_definition_id
            .insert(field_definition_id, return_type_definition_id);
    }

    /// Adds an index from a field definition ID to a relation id.
    pub fn push_client_id_relation_mapping(&mut self, field_definition_id: DefinitionId, relation_id: RelationId) {
        self.indices.client_relations.insert(field_definition_id, relation_id);
    }

    /// Adds an index from a table id and a client field name to a relation id.
    pub fn push_client_name_relation_mapping(&mut self, table_id: TableId, name: &str, relation_id: RelationId) {
        let string_id = self.interner.intern(name);
        self.indices
            .client_name_relations
            .insert((table_id, string_id), relation_id);
    }

    /// Gets the client field name for a field definition ID.
    pub fn get_name_for_field_definition(&self, field_definition_id: DefinitionId) -> Option<&str> {
        self.indices
            .definition_to_field_name
            .get(&field_definition_id)
            .map(|id| self.interner.get(*id))
    }

    /// Gets the relation ID for a client field.
    pub fn get_relation_id_for_client_field_id(&self, definition_id: DefinitionId) -> Option<RelationId> {
        self.indices.client_relations.get(&definition_id).copied()
    }

    /// Gets the relation walker for a given client field name and table ID.
    pub fn get_relation_for_client_name(&self, table_id: TableId, field_name: &str) -> Option<RelationWalker<'_>> {
        self.interner
            .lookup(field_name)
            .and_then(|string_id| self.indices.client_name_relations.get(&(table_id, string_id)))
            .copied()
            .map(|id| self.walk(id))
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
        self.interner
            .lookup(table_name)
            .and_then(|string_id| self.indices.tables.get(&(schema_id, string_id)))
            .copied()
    }

    /// Finds the id of a column in a table with the given name, if existing.
    pub fn get_table_column_id(&self, table_id: TableId, column_name: &str) -> Option<TableColumnId> {
        self.interner
            .lookup(column_name)
            .and_then(|string_id| self.indices.table_columns.get(&(table_id, string_id)))
            .copied()
    }

    /// Finds the id of a column in a table by its client field name, if existing.
    pub fn get_table_column_id_for_field(&self, table_id: TableId, field_name: &str) -> Option<TableColumnId> {
        self.interner
            .lookup(field_name)
            .and_then(|string_id| self.indices.client_fields.get(&(table_id, string_id)))
            .copied()
    }

    /// Finds the column walker for a given client field name and table ID.
    pub fn find_column_for_client_field(&self, field_name: &str, id: TableId) -> Option<TableColumnWalker<'_>> {
        self.get_table_column_id_for_field(id, field_name)
            .map(|id| self.walk(id))
    }

    /// Finds the id of an enum with the given name, if existing.
    pub fn get_enum_id(&self, schema_id: SchemaId, enum_name: &str) -> Option<EnumId> {
        self.interner
            .lookup(enum_name)
            .and_then(|string_id| self.indices.enums.get(&(schema_id, string_id)))
            .copied()
    }

    /// Finds the id of an enum with the given name, if existing.
    pub fn get_foreign_key_id(&self, schema_id: SchemaId, constraint_name: &str) -> Option<ForeignKeyId> {
        self.interner
            .lookup(constraint_name)
            .and_then(|string_id| self.indices.foreign_keys.get(&(schema_id, string_id)))
            .copied()
    }

    /// Finds the id of a unique constraint with the given name, if existing.
    pub fn get_key_id(&self, table_id: TableId, constraint_name: &str) -> Option<KeyId> {
        self.interner
            .lookup(constraint_name)
            .and_then(|string_id| self.indices.keys.get(&(table_id, string_id)))
            .copied()
    }

    /// Finalizes the definition. Handles name deduplication, and sorts the internal data structures
    /// accordingly.
    pub fn finalize(&mut self) {
        self.deduplicate_names();

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

    /// Tables and enums are namespaced per schema in PostgreSQL, but in GraphQL all schemas are in the same namespace.
    ///
    /// If a table or enum has a duplicate name in different schemas, we'll prefix the name with the name of the schema.
    fn deduplicate_names(&mut self) {
        let mut names = HashMap::new();

        for table in &self.tables {
            let counter = names.entry(table.client_name()).or_default();
            *counter += 1;
        }

        for table in &mut self.tables {
            if names.get(&table.client_name()).copied().unwrap_or(0) < 2 {
                continue;
            }

            let schema_name = &self.schemas[table.schema_id().0 as usize];
            let client_name = self.interner.get(table.client_name());

            let new_client_name = format!("{schema_name}_{client_name}").to_pascal_case();
            let client_name = self.interner.intern(&new_client_name);

            let new_client_field_name = self.interner.intern(&new_client_name.to_camel_case());
            let new_client_field_name_plural = self.interner.intern(&new_client_name.to_camel_case().to_plural());

            table.set_client_name(client_name);
            table.set_client_field_name(new_client_field_name);
            table.set_client_field_name_plural(new_client_field_name_plural);
        }

        names.clear();

        for r#enum in &self.enums {
            let counter = names.entry(r#enum.client_name()).or_default();
            *counter += 1;
        }

        for r#enum in &mut self.enums {
            if names.get(&r#enum.client_name()).copied().unwrap_or(0) < 2 {
                continue;
            }

            let schema_name = &self.schemas[r#enum.schema_id().0 as usize];
            let client_name = self.interner.get(r#enum.client_name());

            let client_name = self
                .interner
                .intern(&format!("{schema_name}_{client_name}").to_pascal_case());

            r#enum.set_client_name(client_name);
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
