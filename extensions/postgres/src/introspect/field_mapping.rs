use grafbase_database_definition::{DatabaseDefinition, Operation};
use grafbase_sdk::types::{SubgraphSchema, TypeDefinition};

pub(super) fn introspect(schema: &SubgraphSchema<'_>, database_definition: &mut DatabaseDefinition) {
    for r#type in schema.type_definitions() {
        let TypeDefinition::Object(object_definition) = r#type else {
            continue;
        };

        for field in object_definition.fields() {
            database_definition.push_client_definition_to_name(field.name(), field.id());

            database_definition
                .push_client_field_definition_to_return_type_definition_id(field.id(), field.ty().definition().id());
        }

        if Some(r#type.id()) != schema.query().map(|q| q.id()) && Some(r#type.id()) != schema.mutation().map(|m| m.id())
        {
            continue;
        }

        for field in object_definition.fields() {
            let Some(r#type) = schema.type_definition(&field.ty().definition().id()).map(|t| t.name()) else {
                continue;
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
}
