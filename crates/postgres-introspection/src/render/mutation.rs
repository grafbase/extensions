use grafbase_database_definition::{DatabaseDefinition, TableWalker};
use inflector::Inflector;

use super::ast::{
    directive::{Argument, Directive},
    field::Field,
    schema::Schema,
    r#type::Type,
};

pub fn render<'a>(database_definition: &'a DatabaseDefinition, prefix: Option<&str>, rendered: &mut Schema<'a>) {
    let mut mutation = Type::new("Mutation");

    for table in database_definition.tables().filter(|t| t.allowed_in_client()) {
        if !table.mutations_allowed() {
            continue;
        }

        render_create_mutations(&mut mutation, prefix, table);
        render_update_mutations(&mut mutation, prefix, table);
        render_delete_mutations(&mut mutation, prefix, table);
    }

    if mutation.has_fields() {
        rendered.push_type(mutation);
    }
}

fn render_delete_mutations<'a>(mutation: &mut Type<'a>, prefix: Option<&str>, table: TableWalker<'a>) {
    // delete one
    let delete_field_name = match prefix {
        Some(prefix) => format!("{}_{}Delete", prefix, table.client_name()).to_camel_case(),
        None => format!("{}Delete", table.client_name().to_camel_case()),
    };

    let mut field = Field::new(delete_field_name, format!("{}DeletePayload!", table.client_name()));

    field.set_description(format!("Delete a unique {}", table.client_name()));
    field.push_directive(Directive::new("pgDeleteOne"));

    field.push_argument({
        let mut argument = Argument::constant("lookup", format!("{}LookupInput!", table.client_name()));
        argument.set_description(format!("Lookup input for unique {} deletion", table.client_name()));
        argument
    });

    mutation.push_field(field);

    // delete many
    let delete_field_name = match prefix {
        Some(prefix) => format!("{}_{}DeleteMany", prefix, table.client_name()).to_camel_case(),
        None => format!("{}DeleteMany", table.client_name().to_camel_case()),
    };

    let mut field = Field::new(delete_field_name, format!("{}DeleteManyPayload!", table.client_name()));

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

fn render_update_mutations<'a>(mutation: &mut Type<'a>, prefix: Option<&str>, table: TableWalker<'a>) {
    // update one
    let field_name = match prefix {
        Some(prefix) => format!("{}_{}Update", prefix, table.client_name()).to_camel_case(),
        None => format!("{}Update", table.client_name().to_camel_case()),
    };

    let mut field = Field::new(field_name, format!("{}UpdatePayload!", table.client_name()));

    field.set_description(format!("Update a unique {}", table.client_name()));
    field.push_directive(Directive::new("pgUpdateOne"));

    field.push_argument({
        let mut argument = Argument::constant("lookup", format!("{}LookupInput!", table.client_name()));
        argument.set_description(format!("Lookup input for unique {} update", table.client_name()));
        argument
    });

    field.push_argument({
        let argument_name = match prefix {
            Some(prefix) => format!("{}_{}UpdateInput!", prefix, table.client_name()).to_pascal_case(),
            None => format!("{}UpdateInput!", table.client_name()),
        };

        let mut argument = Argument::constant("input", argument_name);
        argument.set_description(format!("Input for updating a {}", table.client_name()));
        argument
    });

    mutation.push_field(field);

    // update many
    let field_name = match prefix {
        Some(prefix) => format!("{}_{}UpdateMany", prefix, table.client_name()).to_camel_case(),
        None => format!("{}UpdateMany", table.client_name().to_camel_case()),
    };

    let mut field = Field::new(field_name, format!("{}UpdateManyPayload!", table.client_name()));

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
        let argument_name = match prefix {
            Some(prefix) => format!("{}_{}UpdateInput!", prefix, table.client_name()).to_pascal_case(),
            None => format!("{}UpdateInput!", table.client_name()),
        };

        let mut argument = Argument::constant("input", argument_name);
        argument.set_description(format!("Input for updating multiple {} instances", table.client_name()));
        argument
    });

    mutation.push_field(field);
}

fn render_create_mutations<'a>(mutation: &mut Type<'a>, prefix: Option<&str>, table: TableWalker<'a>) {
    // create one
    let field_name = match prefix {
        Some(prefix) => format!("{}_{}Create", prefix, table.client_name()).to_camel_case(),
        None => format!("{}Create", table.client_name().to_camel_case()),
    };

    let mut field = Field::new(field_name, format!("{}CreatePayload!", table.client_name()));

    field.set_description(format!("Create a single {}", table.client_name()));

    field.push_directive(Directive::new("pgInsertOne"));

    field.push_argument({
        let argument_name = match prefix {
            Some(prefix) => format!("{}_{}CreateInput", prefix, table.client_name()).to_pascal_case(),
            None => format!("{}CreateInput", table.client_name()),
        };

        let mut argument = Argument::constant("input", format!("{argument_name}!"));
        argument.set_description(format!("Input for creating a single {}", table.client_name()));
        argument
    });

    mutation.push_field(field);

    // create many
    let field_name = match prefix {
        Some(prefix) => format!("{}_{}CreateMany", prefix, table.client_name()).to_camel_case(),
        None => format!("{}CreateMany", table.client_name().to_camel_case()),
    };

    let mut field = Field::new(field_name, format!("{}CreateManyPayload!", table.client_name()));

    field.set_description(format!(
        "Create multiple {}",
        table.client_name().to_camel_case().to_plural()
    ));

    field.push_directive(Directive::new("pgInsertMany"));

    field.push_argument({
        let argument_name = match prefix {
            Some(prefix) => format!("{}_{}CreateInput", prefix, table.client_name()).to_pascal_case(),
            None => format!("{}CreateInput", table.client_name()),
        };

        let mut argument = Argument::constant("input", format!("[{argument_name}!]!"));
        argument.set_description(format!("Input for creating multiple {} instances", table.client_name()));
        argument
    });

    mutation.push_field(field);
}
