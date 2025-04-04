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

        field.push_argument(Argument::constant(
            "lookup",
            format!("{}LookupInput!", table.client_name()),
        ));

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

        field.push_argument(Argument::constant(
            "filter",
            format!("{}FilterInput", table.client_name()),
        ));

        field.push_argument(Argument::constant("first", "Int"));
        field.push_argument(Argument::constant("last", "Int"));
        field.push_argument(Argument::constant("before", "String"));
        field.push_argument(Argument::constant("after", "String"));

        field.push_argument(Argument::constant(
            "orderBy",
            format!("[{}!]", table.order_by_input_name()),
        ));

        query.push_field(field);
    }

    rendered.push_type(query);
}
