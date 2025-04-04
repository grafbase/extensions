mod interner;

use std::collections::HashMap;

use grafbase_sdk::types::DefinitionId;

pub(super) use self::interner::{StringId, StringInterner};
use super::{
    Enum, EnumId, EnumVariant, EnumVariantId, ForeignKey, ForeignKeyId, Key, KeyId, RelationId, SchemaId, Table,
    TableColumn, TableColumnId, TableId,
};

#[derive(Default, Debug, Clone)]
pub(super) struct Names {
    interner: StringInterner,
    tables: HashMap<(SchemaId, StringId), TableId>,
    table_columns: HashMap<(TableId, StringId), TableColumnId>,
    table_fields: HashMap<(TableId, StringId), TableColumnId>,
    enums: HashMap<(SchemaId, StringId), EnumId>,
    enum_variants: HashMap<(EnumId, StringId), EnumVariantId>,
    foreign_keys: HashMap<(SchemaId, StringId), ForeignKeyId>,
    unique_constraints: HashMap<(TableId, StringId), KeyId>,
    client_types: HashMap<StringId, TableId>,
    client_fields: HashMap<(TableId, StringId), TableColumnId>,
    client_unique_constraints: HashMap<(TableId, StringId), KeyId>,
    client_enums: HashMap<StringId, EnumId>,
    client_relations: HashMap<DefinitionId, RelationId>,
    client_name_relations: HashMap<(TableId, StringId), RelationId>,
    field_definition_to_column: HashMap<DefinitionId, TableColumnId>,
    definition_to_field_name: HashMap<DefinitionId, StringId>,
    field_definition_to_return_type_definition_id: HashMap<DefinitionId, DefinitionId>,
}

impl Names {
    pub(super) fn intern_table(&mut self, table: &Table<String>, table_id: TableId) {
        let string_id = self.interner.intern(table.database_name());
        self.tables.insert((table.schema_id(), string_id), table_id);
    }

    pub(super) fn intern_table_column(&mut self, column: &TableColumn<String>, column_id: TableColumnId) {
        let string_id = self.interner.intern(column.database_name());
        self.table_columns.insert((column.table_id(), string_id), column_id);

        let string_id = self.interner.intern(column.client_name());
        self.table_fields.insert((column.table_id(), string_id), column_id);

        if column.database_name() != column.client_name() {
            let string_id = self.interner.intern(column.client_name());
            self.table_columns.insert((column.table_id(), string_id), column_id);
        }
    }

    pub(super) fn intern_enum(&mut self, r#enum: &Enum<String>, enum_id: EnumId) {
        let string_id = self.interner.intern(r#enum.database_name());
        self.enums.insert((r#enum.schema_id(), string_id), enum_id);
    }

    pub(super) fn intern_foreign_key(&mut self, foreign_key: &ForeignKey<String>, foreign_key_id: ForeignKeyId) {
        let string_id = self.interner.intern(foreign_key.constraint_name());

        self.foreign_keys
            .insert((foreign_key.schema_id(), string_id), foreign_key_id);
    }

    pub(super) fn intern_enum_variant(&mut self, variant: &EnumVariant<String>, variant_id: EnumVariantId) {
        let string_id = self.interner.intern(variant.database_name());
        self.enum_variants.insert((variant.enum_id(), string_id), variant_id);
    }

    pub(super) fn intern_key(&mut self, key: &Key<String>, constraint_id: KeyId) {
        let string_id = self.interner.intern(key.name());

        self.unique_constraints
            .insert((key.table_id(), string_id), constraint_id);
    }

    pub(super) fn intern_client_type(&mut self, type_name: &str, table_id: TableId) {
        let string_id = self.interner.intern(type_name);
        self.client_types.insert(string_id, table_id);
    }

    pub(super) fn intern_client_field(&mut self, field_name: &str, table_id: TableId, column_id: TableColumnId) {
        let string_id = self.interner.intern(field_name);
        self.client_fields.insert((table_id, string_id), column_id);
    }

    pub(super) fn intern_client_key(&mut self, field_name: &str, table_id: TableId, constraint_id: KeyId) {
        let string_id = self.interner.intern(field_name);

        self.client_unique_constraints
            .insert((table_id, string_id), constraint_id);
    }

    pub(super) fn get_field_name_with_definition_id(&self, definition_id: DefinitionId) -> Option<&str> {
        self.definition_to_field_name
            .get(&definition_id)
            .map(|id| self.interner.get(*id))
    }

    pub(super) fn intern_definition_to_field_name(&mut self, field_name: &str, definition_id: DefinitionId) {
        let string_id = self.interner.intern(field_name);
        self.definition_to_field_name.insert(definition_id, string_id);
    }

    pub(super) fn intern_client_enum(&mut self, enum_name: &str, enum_id: EnumId) {
        let string_id = self.interner.intern(enum_name);
        self.client_enums.insert(string_id, enum_id);
    }

    pub(super) fn intern_client_name_relation(&mut self, table_id: TableId, name: &str, relation_id: RelationId) {
        let string_id = self.interner.intern(name);
        self.client_name_relations.insert((table_id, string_id), relation_id);
    }

    pub(super) fn intern_client_id_relation(&mut self, definition_id: DefinitionId, relation_id: RelationId) {
        self.client_relations.insert(definition_id, relation_id);
    }

    pub(super) fn intern_field_column(&mut self, field_definition_id: DefinitionId, column_id: TableColumnId) {
        self.field_definition_to_column.insert(field_definition_id, column_id);
    }

    pub(super) fn intern_string(&mut self, string_value: &str) -> StringId {
        self.interner.intern(string_value)
    }

    pub(super) fn get_table_id_for_client_type(&self, type_name: &str) -> Option<TableId> {
        self.interner
            .lookup(type_name)
            .and_then(|string_id| self.client_types.get(&string_id))
            .copied()
    }

    pub(super) fn get_column_id_for_definition_id(&self, definition_id: DefinitionId) -> Option<TableColumnId> {
        self.field_definition_to_column.get(&definition_id).copied()
    }

    pub(super) fn get_column_id_for_client_field(&self, field_name: &str, table_id: TableId) -> Option<TableColumnId> {
        self.interner
            .lookup(field_name)
            .and_then(|string_id| self.client_fields.get(&(table_id, string_id)))
            .copied()
    }

    pub(super) fn get_relation_id_for_client_field_id(&self, definition_id: DefinitionId) -> Option<RelationId> {
        self.client_relations.get(&definition_id).copied()
    }

    pub(super) fn get_relation_id_for_client_name(&self, table_id: TableId, field_name: &str) -> Option<RelationId> {
        self.interner
            .lookup(field_name)
            .and_then(|string_id| self.client_name_relations.get(&(table_id, string_id)))
            .copied()
    }

    pub(super) fn get_unique_constraint_id_for_client_field(
        &self,
        field_name: &str,
        table_id: TableId,
    ) -> Option<KeyId> {
        self.interner
            .lookup(field_name)
            .and_then(|string_id| self.client_unique_constraints.get(&(table_id, string_id)))
            .copied()
    }

    pub(super) fn get_table_id(&self, schema_id: SchemaId, table_name: &str) -> Option<TableId> {
        self.lookup_name(table_name)
            .and_then(|string_id| self.tables.get(&(schema_id, string_id)))
            .copied()
    }

    pub(super) fn get_table_column_id(&self, table_id: TableId, column_name: &str) -> Option<TableColumnId> {
        self.lookup_name(column_name)
            .and_then(|string_id| self.table_columns.get(&(table_id, string_id)))
            .copied()
    }

    pub(super) fn get_enum_id(&self, schema_id: SchemaId, enum_name: &str) -> Option<EnumId> {
        self.lookup_name(enum_name)
            .and_then(|string_id| self.enums.get(&(schema_id, string_id)))
            .copied()
    }

    pub(super) fn get_foreign_key_id(&self, schema_id: SchemaId, foreign_key_name: &str) -> Option<ForeignKeyId> {
        self.lookup_name(foreign_key_name)
            .and_then(|string_id| self.foreign_keys.get(&(schema_id, string_id)))
            .copied()
    }

    pub(super) fn get_unique_constraint_id(&self, table_id: TableId, constraint_name: &str) -> Option<KeyId> {
        self.lookup_name(constraint_name)
            .and_then(|string_id| self.unique_constraints.get(&(table_id, string_id)))
            .copied()
    }

    pub(super) fn get_name(&self, string_id: StringId) -> &str {
        self.interner.get(string_id)
    }

    fn lookup_name(&self, name: &str) -> Option<StringId> {
        self.interner.lookup(name)
    }

    pub(super) fn intern_client_field_definition_to_return_type_definition_id(
        &mut self,
        field_definition_id: DefinitionId,
        return_type_definition_id: DefinitionId,
    ) {
        self.field_definition_to_return_type_definition_id
            .insert(field_definition_id, return_type_definition_id);
    }

    pub(crate) fn find_return_table_column_for_field_definition_id(
        &self,
        field_definition_id: DefinitionId,
    ) -> Option<TableColumnId> {
        self.field_definition_to_return_type_definition_id
            .get(&field_definition_id)
            .and_then(|id| self.field_definition_to_column.get(id))
            .copied()
    }
}
