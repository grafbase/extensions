use grafbase_database_definition::DatabaseDefinition;
use inflector::Inflector;

use super::ast::{
    directive::{Argument, Directive},
    field::Field,
    schema::Schema,
    r#type::Type,
};

pub fn render<'a>(database_definition: &'a DatabaseDefinition, rendered: &mut Schema<'a>) {
    let mut query = Type::new("Query");

    for table in database_definition.tables().filter(|t| t.allowed_in_client()) {
        let mut field = Field::new(table.client_name().to_camel_case(), table.client_name());

        field.push_directive(Directive::new("pgSelectOne"));
        field.set_description(format!("Query a unique {}", table.client_name()));

        field.push_argument({
            let mut argument = Argument::constant("lookup", format!("{}LookupInput!", table.client_name()));
            argument.set_description(format!("Input for unique {} lookup", table.client_name()));
            argument
        });

        query.push_field(field);

        let mut field = Field::new(
            table.client_name().to_plural().to_camel_case(),
            format!("{}Connection!", table.client_name()),
        );

        field.set_description(format!(
            "Query and paginate multiple {}",
            table.client_name().to_camel_case().to_plural()
        ));

        field.push_directive(Directive::new("pgSelectMany"));

        field.push_argument({
            let mut argument = Argument::constant("filter", format!("{}FilterInput", table.client_name()));
            argument.set_description(format!("Filter for {}", table.client_name()));
            argument
        });

        field.push_argument({
            let mut argument = Argument::constant("first", "Int");
            argument.set_description("Limit the number of results, from the beginning");
            argument
        });

        field.push_argument({
            let mut argument = Argument::constant("last", "Int");
            argument.set_description("Limit the number of results, from the end");
            argument
        });

        field.push_argument({
            let mut argument = Argument::constant("before", "String");
            argument
                .set_description("Cursor for pagination, select items before the cursor. Use together with `last`.");
            argument
        });

        field.push_argument({
            let mut argument = Argument::constant("after", "String");
            argument
                .set_description("Cursor for pagination, select items after the cursor. Use together with `first`.");
            argument
        });

        field.push_argument({
            let mut argument = Argument::constant("orderBy", format!("[{}!]", table.order_by_input_name()));
            argument.set_description("Order the results by selected fields");
            argument
        });

        query.push_field(field);
    }

    rendered.push_type(query);
}
