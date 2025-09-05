use grafbase_database_definition::DatabaseDefinition;
use inflector::Inflector;

use crate::config::Config;

use super::ast::{
    directive::{Argument, Directive},
    field::Field,
    schema::Schema,
    r#type::Type,
};

pub fn render<'a>(
    database_definition: &'a DatabaseDefinition,
    config: &Config,
    prefix: Option<&str>,
    rendered: &mut Schema<'a>,
) {
    let mut query = Type::new("Query");

    for table in database_definition.tables().filter(|t| t.allowed_in_client()) {
        if !config.queries_allowed(table) {
            continue;
        }

        let field_name = match prefix {
            Some(prefix) => format!("{}_{}", prefix, table.client_name()).to_camel_case(),
            None => table.client_name().to_camel_case(),
        };

        let mut field = Field::new(field_name, table.client_name());

        field.set_description(format!("Query a unique {}", table.client_name()));

        field.push_directive(Directive::new("pgSelectOne"));

        field.push_argument({
            let mut argument = Argument::constant("lookup", format!("{}LookupInput!", table.client_name()));
            argument.set_description(format!("Input for unique {} lookup", table.client_name()));
            argument
        });

        query.push_field(field);

        let field_name = match prefix {
            Some(prefix) => format!("{}_{}", prefix, table.client_name().to_plural()).to_camel_case(),
            None => table.client_name().to_plural().to_camel_case(),
        };

        let mut field = Field::new(field_name, format!("{}Connection!", table.client_name()));

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

        let field_name = match prefix {
            Some(prefix) => format!("{}_{}_lookup", prefix, table.client_name()).to_camel_case(),
            None => format!("{}_lookup", table.client_name()).to_camel_case(),
        };

        let mut field = Field::new(field_name, format!("[{}]", table.client_name()));

        field.set_description(format!(
            "Lookup multiple {} for subgraph joins",
            table.client_name().to_camel_case().to_plural()
        ));

        field.push_directive(Directive::new("pgLookup"));
        field.push_directive(Directive::new("lookup"));
        field.push_directive(Directive::new("inaccessible"));

        field.push_argument({
            let mut argument = Argument::constant("lookup", format!("{}ManyLookupInput", table.client_name()));
            argument.push_directive(Directive::new("inaccessible"));

            argument.set_description(format!(
                "Filter {} with an array of keys",
                table.client_name().to_camel_case().to_plural()
            ));

            argument
        });

        query.push_field(field);
    }

    if query.has_fields() {
        rendered.push_type(query);
    }
}
