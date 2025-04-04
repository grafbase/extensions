use grafbase_database_definition::{DatabaseDefinition, ForeignKey, ForeignKeyColumn, RelationId};
use grafbase_sdk::types::{SubgraphSchema, TypeDefinition};

use super::{PgRelation, PgTable};

pub(super) fn introspect_sdl(schema: &SubgraphSchema<'_>, database_definition: &mut DatabaseDefinition) {
    for r#type in schema.type_definitions() {
        let TypeDefinition::Object(constrained_object) = r#type else {
            continue;
        };

        let Some(pg_table) = constrained_object
            .directives()
            .find(|directive| directive.name() == "pgTable")
            .and_then(|d| d.arguments::<PgTable>().ok())
        else {
            continue;
        };

        let Some(constrained_schema_id) = database_definition.get_schema_id(&pg_table.schema) else {
            continue;
        };

        let Some(constrained_table_id) = database_definition.get_table_id(constrained_schema_id, &pg_table.name) else {
            continue;
        };

        for constrained_field in constrained_object.fields() {
            let TypeDefinition::Object(referenced_object) = constrained_field.ty().definition() else {
                continue;
            };

            let referenced_object = if referenced_object.directives().any(|d| d.name() == "pgConnection") {
                let Some(edges) = referenced_object.fields().find(|f| f.name() == "edges") else {
                    continue;
                };

                let TypeDefinition::Object(edges) = edges.ty().definition() else {
                    continue;
                };

                let Some(node) = edges.fields().find(|f| f.name() == "node") else {
                    continue;
                };

                let TypeDefinition::Object(node) = node.ty().definition() else {
                    continue;
                };

                node
            } else {
                referenced_object
            };

            let Some(referenced_table_id) = database_definition
                .find_table_for_client_type(referenced_object.name())
                .map(|t| t.id())
            else {
                continue;
            };

            if constrained_field
                .directives()
                .any(|directive| directive.name() == "pgColumn")
            {
                continue;
            };

            let Some(pg_relation) = constrained_field
                .directives()
                .find(|directive| directive.name() == "pgRelation")
                .and_then(|d| d.arguments::<PgRelation>().ok())
            else {
                continue;
            };

            if pg_relation.fields.is_empty() || pg_relation.references.is_empty() {
                continue;
            }

            let referenced_field = referenced_object
                .fields()
                .find(|f| {
                    f.directives()
                        .any(|d| d.arguments::<PgRelation>().map(|a| a.name).ok().as_ref() == Some(&pg_relation.name))
                })
                .expect("yolo");

            let foreign_key = ForeignKey::new(
                pg_relation.name,
                constrained_schema_id,
                constrained_table_id,
                referenced_table_id,
            );

            let (fk_id, forward, back) = database_definition.push_foreign_key(foreign_key);

            let columns = pg_relation.fields.into_iter().zip(pg_relation.references.into_iter());

            database_definition.push_client_id_relation_mapping(constrained_field.id(), RelationId::Forward(forward));
            database_definition.push_client_id_relation_mapping(referenced_field.id(), RelationId::Back(back));

            database_definition.push_client_name_relation_mapping(
                constrained_table_id,
                constrained_field.name(),
                RelationId::Forward(forward),
            );

            database_definition.push_client_name_relation_mapping(
                referenced_table_id,
                referenced_field.name(),
                RelationId::Back(back),
            );

            for (constrained_field_name, referenced_field_name) in columns {
                let Some(constrained_column_id) =
                    database_definition.get_table_column_id(constrained_table_id, &constrained_field_name)
                else {
                    continue;
                };

                let Some(referenced_column_id) =
                    database_definition.get_table_column_id(referenced_table_id, &referenced_field_name)
                else {
                    continue;
                };

                let fk_column = ForeignKeyColumn::new(fk_id, constrained_column_id, referenced_column_id);
                database_definition.push_foreign_key_column(fk_column);
            }
        }
    }
}
