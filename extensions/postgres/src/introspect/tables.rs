use grafbase_database_definition::{
    ColumnType, DatabaseDefinition, EnumType, ScalarKind, ScalarType, SchemaId, Table, TableColumn,
};
use grafbase_sdk::types::{FieldDefinition, SubgraphSchema, TypeDefinition};

use super::{PgColumn, PgTable};

pub(crate) fn introspect_sdl(schema: &SubgraphSchema<'_>, database_definition: &mut DatabaseDefinition) {
    for r#type in schema.type_definitions() {
        let TypeDefinition::Object(definition) = r#type else {
            continue;
        };

        let Some(pg_table) = definition
            .directives()
            .find(|directive| directive.name() == "pgTable")
            .and_then(|d| d.arguments::<PgTable>().ok())
        else {
            continue;
        };

        let Some(schema_id) = database_definition.get_schema_id(&pg_table.schema) else {
            continue;
        };

        let table = Table::<String>::new(schema_id, pg_table.name, Some(definition.name().to_string()));
        let table_id = database_definition.push_table(table);

        for field in definition.fields() {
            let Some(pg_column) = field
                .directives()
                .find(|directive| directive.name() == "pgColumn")
                .and_then(|d| d.arguments::<PgColumn>().ok())
            else {
                continue;
            };

            let Some(column_type) = introspect_type(database_definition, schema_id, field, &pg_column) else {
                continue;
            };

            let column =
                TableColumn::<String>::new(table_id, column_type, pg_column.name, Some(field.name().to_string()));

            database_definition.push_table_column(column, Some(field.id()));
        }
    }
}

fn introspect_type(
    database_definition: &DatabaseDefinition,
    schema_id: SchemaId,
    field: FieldDefinition<'_>,
    pg_column: &PgColumn,
) -> Option<ColumnType> {
    match pg_column.r#type {
        ScalarKind::Enum => {
            let schema_id = match pg_column.enum_schema {
                Some(ref schema) => database_definition.get_schema_id(schema)?,
                None => schema_id,
            };

            let enum_name = field.ty().definition().name();
            let enum_id = database_definition.get_enum_id(schema_id, enum_name)?;
            let is_array = field.ty().is_list();

            Some(ColumnType::Enum(EnumType::new(enum_id, is_array)))
        }
        kind => {
            let is_array = field.ty().is_list();

            Some(ColumnType::Scalar(ScalarType::new(kind, is_array)))
        }
    }
}
