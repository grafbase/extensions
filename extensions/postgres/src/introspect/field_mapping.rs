use grafbase_database_definition::{DatabaseDefinition, Operation};
use grafbase_sdk::types::{ObjectDefinition, SubgraphSchema, TypeDefinition};

#[derive(serde::Deserialize)]
struct PgTypePointer<'a> {
    r#type: &'a str,
}

pub(super) fn introspect(schema: &SubgraphSchema<'_>, database_definition: &mut DatabaseDefinition) {
    for r#type in schema.type_definitions() {
        let TypeDefinition::Object(object_definition) = r#type else {
            continue;
        };

        for field in object_definition.fields() {
            database_definition.push_client_definition_to_name(field.name(), field.id());

            database_definition.push_field_definition_to_type_definition(field.id(), field.ty().definition().id());
        }

        if Some(r#type.id()) == schema.query().map(|q| q.id()) || Some(r#type.id()) == schema.mutation().map(|m| m.id())
        {
            map_operations(schema, &object_definition, database_definition);
            continue;
        }

        if let Some(returning) = object_definition
            .directives()
            .find(|d| d.name() == "pgReturning" || d.name() == "pgConnection")
            .and_then(|d| d.arguments::<PgTypePointer>().ok())
        {
            map_pointer_type(&object_definition, returning, database_definition);
        }
    }
}

fn map_pointer_type(
    object_definition: &ObjectDefinition<'_>,
    returning: PgTypePointer<'_>,
    database_definition: &mut DatabaseDefinition,
) {
    let Some(table_id) = database_definition
        .find_table_for_client_type(returning.r#type)
        .map(|t| t.id())
    else {
        return;
    };

    for field in object_definition.fields() {
        let Some(column_id) = database_definition.get_table_column_id_for_field(table_id, field.name()) else {
            continue;
        };

        database_definition.push_column_to_definition(field.id(), column_id);
    }
}

fn map_operations(
    schema: &SubgraphSchema<'_>,
    object_definition: &ObjectDefinition<'_>,
    database_definition: &mut DatabaseDefinition,
) {
    for field in object_definition.fields() {
        let Some(TypeDefinition::Object(r#type)) = schema.type_definition(&field.ty().definition().id()) else {
            continue;
        };

        let r#type = match r#type
            .directives()
            .find(|d| d.name() == "pgMutation" || d.name() == "pgConnection")
            .and_then(|d| d.arguments::<PgTypePointer<'_>>().ok())
        {
            Some(args) => args.r#type,
            None => r#type.name(),
        };

        let Some(table_id) = database_definition.find_table_for_client_type(r#type).map(|t| t.id()) else {
            continue;
        };

        for directive in field.directives() {
            match directive.name() {
                "pgSelectMany" => {
                    database_definition.push_operation(field.id(), Operation::FindMany(table_id));
                }
                "pgSelectOne" => {
                    database_definition.push_operation(field.id(), Operation::FindOne(table_id));
                }
                "pgInsertMany" => {
                    database_definition.push_operation(field.id(), Operation::CreateMany(table_id));
                }
                "pgInsertOne" => {
                    database_definition.push_operation(field.id(), Operation::CreateOne(table_id));
                }
                "pgUpdateMany" => {
                    database_definition.push_operation(field.id(), Operation::UpdateMany(table_id));
                }
                "pgUpdateOne" => {
                    database_definition.push_operation(field.id(), Operation::UpdateOne(table_id));
                }
                "pgDeleteMany" => {
                    database_definition.push_operation(field.id(), Operation::DeleteMany(table_id));
                }
                "pgDeleteOne" => {
                    database_definition.push_operation(field.id(), Operation::DeleteOne(table_id));
                }
                _ => {}
            }
        }
    }
}
