use super::PgEnumVariant;
use crate::introspect::PgEnum;
use grafbase_database_definition::{DatabaseDefinition, Enum, EnumVariant};
use grafbase_sdk::types::{SubgraphSchema, TypeDefinition};

pub(crate) fn introspect_sdl(schema: &SubgraphSchema<'_>, database_definition: &mut DatabaseDefinition) {
    for r#type in schema.type_definitions() {
        let TypeDefinition::Enum(definition) = r#type else {
            continue;
        };

        let Some(pg_enum) = definition
            .directives()
            .find(|directive| directive.name() == "pgEnum")
            .and_then(|d| d.arguments::<PgEnum>().ok())
        else {
            continue;
        };

        let Some(schema_id) = database_definition.get_schema_id(&pg_enum.schema) else {
            continue;
        };

        let r#enum = Enum::new(schema_id, pg_enum.name, Some(r#type.name().to_string()));
        let enum_id = database_definition.push_enum(r#enum);

        for variant in definition.values() {
            let Some(pg_enum_variant) = variant
                .directives()
                .find(|directive| directive.name() == "pgEnumVariant")
                .and_then(|d| d.arguments::<PgEnumVariant>().ok())
            else {
                continue;
            };

            database_definition.push_enum_variant(EnumVariant::new(
                enum_id,
                pg_enum_variant.name,
                Some(variant.name().to_string()),
            ));
        }
    }
}
