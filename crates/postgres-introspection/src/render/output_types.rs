use grafbase_database_definition::{DatabaseDefinition, TableWalker};
use inflector::Inflector;

use super::ast::{
    directive::{Argument, Directive},
    field::Field,
    schema::Schema,
    r#type::Type,
};

pub fn render<'a>(database_definition: &'a DatabaseDefinition, rendered: &mut Schema<'a>) {
    render_page_info(rendered);

    for table in database_definition.tables().filter(|t| t.allowed_in_client()) {
        if table.mutations_allowed() {
            let returning_type = render_returning_type(rendered, table);
            render_mutation_types(rendered, table, returning_type);
        }

        render_edge(rendered, table);
        render_connection(rendered, table);
    }
}

fn render_connection<'a>(rendered: &mut Schema<'a>, table: TableWalker<'a>) {
    let mut r#type = Type::new(format!("{}Connection", table.client_name()));
    r#type.set_description(format!("The connection type for {}", table.client_name()));

    r#type.push_directive({
        let mut directive = Directive::new("pgConnection");
        directive.push_argument(Argument::string("type", table.client_name()));
        directive
    });

    r#type.push_field({
        let mut field = Field::new("edges", format!("[{}Edge!]!", table.client_name()));
        field.push_directive(Directive::new("shareable"));
        field.set_description("A list of edges");
        field
    });

    r#type.push_field({
        let mut field = Field::new("pageInfo", "PageInfo!");
        field.push_directive(Directive::new("shareable"));
        field.set_description("Information to aid in pagination");
        field
    });

    rendered.push_type(r#type);
}

fn render_edge<'a>(rendered: &mut Schema<'a>, table: TableWalker<'a>) {
    let mut r#type = Type::new(format!("{}Edge", table.client_name()));
    r#type.set_description("An edge in a connection. Contains the node and its cursor");

    r#type.push_field({
        let mut field = Field::new("node", format!("{}!", table.client_name()));
        field.push_directive(Directive::new("shareable"));
        field.set_description("The item at the end of the edge");
        field
    });

    r#type.push_field({
        let mut field = Field::new("cursor", "String!");
        field.push_directive(Directive::new("shareable"));
        field.set_description("A cursor for use in pagination");
        field
    });

    rendered.push_type(r#type);
}

fn render_mutation_types<'a>(rendered: &mut Schema<'a>, table: TableWalker<'a>, returning_type: String) {
    let mutations = [
        (
            format!("{}CreatePayload", table.client_name()),
            format!("Return type when creating one {}", table.client_name()),
            returning_type.clone(),
            table.client_name(),
        ),
        (
            format!("{}CreateManyPayload", table.client_name()),
            format!(
                "Return type when creating many {}",
                table.client_name().to_plural().to_camel_case()
            ),
            format!("[{returning_type}]!"),
            table.client_name(),
        ),
        (
            format!("{}UpdatePayload", table.client_name()),
            format!("Return type when updating one {}", table.client_name()),
            returning_type.clone(),
            table.client_name(),
        ),
        (
            format!("{}UpdateManyPayload", table.client_name()),
            format!(
                "Return type when updating many {}",
                table.client_name().to_plural().to_camel_case()
            ),
            format!("[{returning_type}]!"),
            table.client_name(),
        ),
        (
            format!("{}DeletePayload", table.client_name()),
            format!("Return type when deleting one {}", table.client_name()),
            returning_type.clone(),
            table.client_name(),
        ),
        (
            format!("{}DeleteManyPayload", table.client_name()),
            format!(
                "Return type when deleting many {}",
                table.client_name().to_plural().to_camel_case()
            ),
            format!("[{returning_type}]!"),
            table.client_name(),
        ),
    ];

    for (type_name, type_description, returning_type, target) in mutations {
        let mut r#type = Type::new(type_name);

        r#type.set_description(type_description);

        r#type.push_directive({
            let mut directive = Directive::new("pgMutation");
            directive.push_argument(Argument::string("type", target));
            directive
        });

        r#type.push_field({
            let mut field = Field::new("returning", returning_type);
            field.push_directive(Directive::new("shareable"));
            field.set_description("Returned item(s) from the mutation");
            field
        });

        r#type.push_field({
            let mut field = Field::new("rowCount", "Int!");
            field.push_directive(Directive::new("shareable"));
            field.set_description("The number of rows mutated");
            field
        });

        rendered.push_type(r#type);
    }
}

fn render_returning_type<'a>(rendered: &mut Schema<'a>, table: TableWalker<'a>) -> String {
    let returning_type = format!("{}Returning", table.client_name());
    let mut r#type = Type::new(returning_type.clone());

    r#type.set_description(format!(
        "Return type containing fields of the mutated or created {} object",
        table.client_name()
    ));

    r#type.push_directive({
        let mut directive = Directive::new("pgReturning");
        directive.push_argument(Argument::string("type", table.client_name()));
        directive
    });

    for column in table.columns() {
        let mut field = Field::new(column.client_name(), column.client_type(None).unwrap());

        if column.is_part_of_a_key() {
            field.push_directive(Directive::new("shareable"));
        }

        field.set_description(format!("The value of the {} field", column.client_name()));
        r#type.push_field(field);
    }

    rendered.push_type(r#type);
    returning_type
}

fn render_page_info(rendered: &mut Schema<'_>) {
    let mut r#type = Type::new("PageInfo");

    r#type.set_description("Information about pagination in a collection of objects");
    r#type.push_directive(Directive::new("shareable"));

    r#type.push_field({
        let mut field = Field::new("hasPreviousPage", "Boolean!");
        field.set_description("When paginating backwards, are there more items?");
        field.push_directive(Directive::new("shareable"));
        field
    });

    r#type.push_field({
        let mut field = Field::new("hasNextPage", "Boolean!");
        field.set_description("When paginating forwards, are there more items?");
        field.push_directive(Directive::new("shareable"));
        field
    });

    r#type.push_field({
        let mut field = Field::new("startCursor", "String!");
        field.set_description("The cursor of the first item in the page");
        field.push_directive(Directive::new("shareable"));
        field
    });

    r#type.push_field({
        let mut field = Field::new("endCursor", "String!");
        field.set_description("The cursor of the last item in the page");
        field.push_directive(Directive::new("shareable"));
        field
    });

    rendered.push_type(r#type);
}
