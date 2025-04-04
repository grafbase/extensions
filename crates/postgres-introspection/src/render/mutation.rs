use grafbase_database_definition::{DatabaseDefinition, TableWalker};
use inflector::Inflector;

use super::ast::{
    directive::{Argument, Directive},
    field::Field,
    schema::Schema,
    r#type::Type,
};

pub fn render<'a>(database_definition: &'a DatabaseDefinition, rendered: &mut Schema<'a>) {
    let mut mutation = Type::new("Mutation");

    for table in database_definition.tables().filter(|t| t.allowed_in_client()) {
        render_create_mutations(&mut mutation, table);
        render_update_mutations(&mut mutation, table);
        render_delete_mutations(&mut mutation, table);
    }

    rendered.push_type(mutation);
}

fn render_delete_mutations<'a>(mutation: &mut Type<'a>, table: TableWalker<'a>) {
    // delete one
    let mut field = Field::new(
        format!("{}Delete", table.client_name().to_camel_case()),
        format!("{}DeletePayload!", table.client_name()),
    );

    field.set_description(format!("Delete a unique {}", table.client_name()));
    field.push_directive(Directive::new("pgDeleteOne"));

    field.push_argument({
        let mut argument = Argument::constant("lookup", format!("{}LookupInput!", table.client_name()));
        argument.set_description(format!("Lookup input for unique {} deletion", table.client_name()));
        argument
    });

    mutation.push_field(field);

    // delete many
    let mut field = Field::new(
        format!("{}DeleteMany", table.client_name().to_camel_case()),
        format!("{}DeleteManyPayload!", table.client_name()),
    );

    field.set_description(format!(
        "Delete multiple {}",
        table.client_name().to_camel_case().to_plural()
    ));

    field.push_directive(Directive::new("pgDeleteMany"));

    field.push_argument({
        let mut argument = Argument::constant("filter", format!("{}FilterInput", table.client_name()));
        argument.set_description(format!("Filter for {} deletion", table.client_name()));
        argument
    });

    mutation.push_field(field);
}

fn render_update_mutations<'a>(mutation: &mut Type<'a>, table: TableWalker<'a>) {
    // update one
    let mut field = Field::new(
        format!("{}Update", table.client_name().to_camel_case()),
        format!("{}UpdatePayload!", table.client_name()),
    );

    field.set_description(format!("Update a unique {}", table.client_name()));
    field.push_directive(Directive::new("pgUpdateOne"));

    field.push_argument({
        let mut argument = Argument::constant("lookup", format!("{}LookupInput!", table.client_name()));
        argument.set_description(format!("Lookup input for unique {} update", table.client_name()));
        argument
    });

    field.push_argument({
        let mut argument = Argument::constant("input", format!("{}UpdateInput!", table.client_name()));
        argument.set_description(format!("Input for updating a {}", table.client_name()));
        argument
    });

    mutation.push_field(field);

    // update many
    let mut field = Field::new(
        format!("{}UpdateMany", table.client_name().to_camel_case()),
        format!("{}UpdateManyPayload!", table.client_name()),
    );

    field.set_description(format!(
        "Update multiple {}",
        table.client_name().to_camel_case().to_plural()
    ));

    field.push_directive(Directive::new("pgUpdateMany"));

    field.push_argument({
        let mut argument = Argument::constant("filter", format!("{}FilterInput", table.client_name()));
        argument.set_description(format!(
            "Filter for updating multiple {} instances",
            table.client_name()
        ));
        argument
    });

    field.push_argument({
        let mut argument = Argument::constant("input", format!("{}UpdateInput!", table.client_name()));
        argument.set_description(format!("Input for updating multiple {} instances", table.client_name()));
        argument
    });

    mutation.push_field(field);
}

fn render_create_mutations<'a>(mutation: &mut Type<'a>, table: TableWalker<'a>) {
    // create one
    let mut field = Field::new(
        format!("{}Create", table.client_name().to_camel_case()),
        format!("{}CreatePayload!", table.client_name()),
    );

    field.set_description(format!("Create a single {}", table.client_name()));

    field.push_directive(Directive::new("pgInsertOne"));

    field.push_argument({
        let mut argument = Argument::constant("input", format!("{}CreateInput!", table.client_name()));
        argument.set_description(format!("Input for creating a single {}", table.client_name()));
        argument
    });

    mutation.push_field(field);

    // create many
    let mut field = Field::new(
        format!("{}CreateMany", table.client_name().to_camel_case()),
        format!("{}CreateManyPayload!", table.client_name()),
    );

    field.set_description(format!(
        "Create multiple {}",
        table.client_name().to_camel_case().to_plural()
    ));

    field.push_directive(Directive::new("pgInsertMany"));

    field.push_argument({
        let mut argument = Argument::constant("input", format!("[{}CreateInput!]!", table.client_name()));
        argument.set_description(format!("Input for creating multiple {} instances", table.client_name()));
        argument
    });

    mutation.push_field(field);
}
