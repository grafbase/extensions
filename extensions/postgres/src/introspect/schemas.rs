use grafbase_database_definition::DatabaseDefinition;
use grafbase_sdk::types::{SubgraphSchema, TypeDefinition};
use std::collections::BTreeSet;

use super::{PgEnum, PgTable};

pub(crate) fn introspect_sdl(schema: &SubgraphSchema, database_definition: &mut DatabaseDefinition) {
    use TypeDefinition::*;

    let mut schemas = BTreeSet::new();

    for r#type in schema.type_definitions() {
        match r#type {
            Object(definition) => {
                let Some(pg_table) = definition
                    .directives()
                    .find(|directive| directive.name() == "pgTable")
                    .and_then(|d| d.arguments::<PgTable>().ok())
                else {
                    continue;
                };

                schemas.insert(pg_table.schema);
            }
            Enum(definition) => {
                let Some(pg_enum) = definition
                    .directives()
                    .find(|directive| directive.name() == "pgEnum")
                    .and_then(|d| d.arguments::<PgEnum>().ok())
                else {
                    continue;
                };

                schemas.insert(pg_enum.schema);
            }
            Scalar(_) | Interface(_) | Union(_) | InputObject(_) => continue,
        }
    }

    for schema in schemas {
        database_definition.push_schema(schema);
    }
}
