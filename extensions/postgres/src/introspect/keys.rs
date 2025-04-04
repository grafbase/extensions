use grafbase_database_definition::{DatabaseDefinition, Key, KeyColumn, KeyType};
use grafbase_sdk::types::{SubgraphSchema, TypeDefinition};
use inflector::Inflector;

use super::{PgKey, PgTable};

pub(super) fn introspect_sdl(schema: &SubgraphSchema<'_>, database_definition: &mut DatabaseDefinition) {
    'main: for r#type in schema.type_definitions() {
        let TypeDefinition::Object(object) = r#type else {
            continue;
        };

        let Some(pg_table) = object
            .directives()
            .find(|directive| directive.name() == "pgTable")
            .and_then(|d| d.arguments::<PgTable>().ok())
        else {
            continue;
        };

        for pg_key in object.directives().filter(|d| d.name() == "pgKey") {
            let Some(pg_key) = pg_key.arguments::<PgKey>().ok() else {
                continue;
            };

            let Some(schema_id) = database_definition.get_schema_id(&pg_table.schema) else {
                continue;
            };

            let Some(table_id) = database_definition.get_table_id(schema_id, &pg_table.name) else {
                continue;
            };

            // TODO: revisit when everything's done. we might not need this.
            let constraint_name = match pg_key.r#type {
                KeyType::Primary => format!("pk_{}", object.name()),
                KeyType::Unique => format!("unique_{}", object.name()),
            };

            let key = Key::new(table_id, constraint_name, pg_key.r#type);
            let key_id = database_definition.push_key(key);

            for field_name in &pg_key.fields {
                let Some(column_id) = database_definition
                    .find_column_for_client_field(field_name, table_id)
                    .map(|c| c.id())
                else {
                    continue 'main;
                };

                let key_column = KeyColumn::new(key_id, column_id);
                database_definition.push_key_column(key_column);
                database_definition.push_client_field_key_mapping(field_name, table_id, key_id);
            }

            if pg_key.fields.len() > 1 {
                let field_name = pg_key.fields.join("_").to_camel_case();
                database_definition.push_client_field_key_mapping(&field_name, table_id, key_id);
            }
        }
    }
}
