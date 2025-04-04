use std::borrow::Cow;

use grafbase_database_definition::{DatabaseDefinition, TableWalker};
use inflector::Inflector;
use itertools::Itertools;

use crate::render::ast::{directive::Directive, field::Field, input::InputType};

use super::ast::schema::Schema;

const SCALARS: &[&str] = &["String", "Int", "Float", "Boolean", "JSON"];

const FILTERS: &[(&str, &str)] = &[
    ("eq", "The value is exactly the one given"),
    ("ne", "The value is not the one given"),
    ("gt", "The value is greater than the one given"),
    ("lt", "The value is less than the one given"),
    ("gte", "The value is greater than, or equal to the one given"),
    ("lte", "The value is less than, or equal to the one given"),
];

const ARRAY_FILTERS: &[(&str, &str)] = &[
    ("in", "The value is in the given array of values"),
    ("nin", "The value is not in the given array of values"),
];

const ARRAYS: &[(&str, &str)] = &[
    ("[String]", "String"),
    ("[Int]", "Int"),
    ("[Float]", "Float"),
    ("[Boolean]", "Boolean"),
    ("[JSON]", "JSON"),
];

pub fn render<'a>(database_definition: &'a DatabaseDefinition, rendered: &mut Schema<'a>) {
    render_scalar_filters(database_definition, rendered);

    for table in database_definition.tables().filter(|t| t.allowed_in_client()) {
        render_order_input(rendered, table);
        render_fetch_input(rendered, table);
        render_filter_input(rendered, table);
        render_create_input(rendered, table);
        render_update_input(rendered, table);
    }
}

fn render_update_input<'a>(rendered: &mut Schema<'a>, table: TableWalker<'a>) {
    let mut mutation_input = InputType::new(format!("{}UpdateInput", table.client_name()));
    mutation_input.set_description(format!("Input for updating an existing {}", table.client_name()));

    for column in table.columns() {
        if !column.allows_user_input() {
            continue;
        }

        let type_name = if column.is_array() {
            Cow::Owned(format!("[{}]", column.client_base_type().unwrap()))
        } else {
            Cow::Borrowed(column.client_base_type().unwrap())
        };

        mutation_input.push_field({
            let mut field = Field::new(column.client_name(), type_name);
            field.set_description(format!("Set field value for {}", column.client_name()));
            field
        });
    }

    rendered.push_input(mutation_input);
}

fn render_create_input<'a>(rendered: &mut Schema<'a>, table: TableWalker<'a>) {
    let type_name = format!("{}CreateInput", table.client_name());
    let mut mutation_input = InputType::new(type_name);

    mutation_input.set_description(format!("Input for creating a new {}", table.client_name()));

    for column in table.columns() {
        if !column.allows_user_input() {
            continue;
        }

        let type_name = if column.is_array() {
            Cow::Owned(format!("[{}]", column.client_base_type().unwrap()))
        } else {
            Cow::Borrowed(column.client_base_type().unwrap())
        };

        let type_name = if column.allows_null_input() {
            type_name
        } else {
            Cow::Owned(format!("{}!", type_name))
        };

        mutation_input.push_field({
            let mut field = Field::new(column.client_name(), type_name);
            field.set_description(format!("Set field value for {}", column.client_name()));
            field
        });
    }

    rendered.push_input(mutation_input);
}

fn render_filter_input<'a>(rendered: &mut Schema<'a>, table: TableWalker<'a>) {
    let type_name = format!("{}FilterInput", table.client_name());
    let mut filter_input = InputType::new(type_name.clone());

    filter_input.set_description(format!("Filter input type for {} objects.", table.client_name()));

    filter_input.push_directive(Directive::new("oneOf"));

    for column in table.columns() {
        let scalar = column.client_base_type().unwrap();

        let filter_type = if column.is_array() {
            format!("{scalar}ArrayFilterInput")
        } else {
            format!("{scalar}FilterInput")
        };

        filter_input.push_field({
            let mut field = Field::new(column.client_name(), filter_type);
            field.set_description(format!("Filter by the given {}", column.client_name()));
            field
        });
    }

    let mut collection_input = InputType::new(format!("{}CollectionFilterInput", table.client_name()));

    collection_input.set_description(format!("Filter input type for {} collections", table.client_name()));

    collection_input.push_field({
        let mut field = Field::new("contains", type_name.clone());
        field.set_description("The object is related to an object with the given fields");
        field
    });

    rendered.push_input(collection_input);

    for relation in table.relations() {
        if relation.is_other_side_one() {
            filter_input.push_field({
                let mut field = Field::new(
                    relation.client_field_name(),
                    format!("{}FilterInput", relation.referenced_table().client_name()),
                );

                field.set_description(format!(
                    "Filter by the related {} object",
                    relation.referenced_table().client_name()
                ));

                field
            });
        } else {
            filter_input.push_field({
                let mut field = Field::new(
                    relation.client_field_name(),
                    format!("{}CollectionFilterInput", relation.referenced_table().client_name()),
                );

                field.set_description(format!(
                    "Filter by the related {} objects",
                    relation.referenced_table().client_name()
                ));

                field
            });
        }
    }

    for (op, desc) in &[
        ("ALL", "All of the filters must match"),
        ("NONE", "None of the filters must match"),
        ("ANY", "At least one of the filters must match"),
    ] {
        filter_input.push_field({
            let mut field = Field::new(*op, format!("[{}]", type_name.clone()));
            field.set_description(*desc);
            field
        });
    }

    rendered.push_input(filter_input);
}

fn render_fetch_input<'a>(rendered: &mut Schema<'a>, table: TableWalker<'a>) {
    let mut filter_input = InputType::new(format!("{}LookupInput", table.client_name()));
    filter_input.set_description(format!("Input type to select a unique {}", table.client_name()));

    filter_input.push_directive(Directive::new("oneOf"));

    for key in table.keys() {
        if key.columns().count() == 1 {
            let column = key.columns().next().unwrap().table_column();

            filter_input.push_field({
                let mut field = Field::new(column.client_name(), column.client_base_type().unwrap());
                field.set_description(format!("Select by the '{}' field", column.client_name()));
                field
            });
        } else {
            let type_name = format!(
                "{}_{}_Input",
                table.client_name(),
                key.columns().map(|c| c.table_column().client_name()).join("_")
            )
            .to_pascal_case();

            let mut composite_input = InputType::new(type_name.clone());

            composite_input.set_description(format!(
                "Input type to select a unique {} with multiple fields",
                table.client_name()
            ));

            for column in key.columns() {
                let column = column.table_column();

                composite_input.push_field({
                    let mut field = Field::new(column.client_name(), column.client_type(None).unwrap());

                    field.set_description(format!("Select by the '{}' field", column.client_name()));
                    field
                });
            }

            rendered.push_input(composite_input);

            filter_input.push_field({
                let field_name = key
                    .columns()
                    .map(|c| c.table_column().client_name())
                    .join("_")
                    .to_camel_case();

                let mut field = Field::new(field_name, type_name);

                field.set_description(format!(
                    "Select {} by composite columns '{}'",
                    table.client_name(),
                    key.columns().map(|c| c.table_column().client_name()).join(", ")
                ));

                field
            });
        }
    }

    rendered.push_input(filter_input);
}

fn render_order_input<'a>(rendered: &mut Schema<'a>, table: TableWalker<'a>) {
    let mut order_input = InputType::new(table.order_by_input_name());
    order_input.push_directive(Directive::new("oneOf"));
    order_input.set_description(format!("Specifies the ordering for {} results.", table.client_name()));

    for column in table.columns() {
        order_input.push_field({
            let mut field = Field::new(column.client_name(), "OrderDirection");

            field.set_description(format!(
                "Order {} by {}",
                table.client_name().to_camel_case().to_plural(),
                column.client_name()
            ));

            field
        });
    }

    for relation in table.relations().filter(|r| r.is_other_side_one()) {
        order_input.push_field({
            let mut field = Field::new(
                relation.client_field_name(),
                relation.referenced_table().order_by_input_name(),
            );

            field.set_description(format!(
                "Order {} results by {} fields",
                relation.referencing_table().client_name(),
                relation.referenced_table().client_name(),
            ));

            field
        });
    }

    rendered.push_input(order_input);
}

fn render_scalar_filters<'a>(database_definition: &'a DatabaseDefinition, rendered: &mut Schema<'a>) {
    for scalar in SCALARS {
        rendered.push_input(create_scalar_filters(scalar));
    }

    for (return_type, scalar) in ARRAYS {
        let input_type_name = format!("{scalar}ArrayFilterInput");
        let mut input = InputType::new(input_type_name.clone());

        input.set_description(format!("Search filter input for {scalar} array type."));
        input.push_directive(Directive::new("oneOf"));

        for (filter, description) in FILTERS {
            let mut field = Field::new(*filter, *return_type);
            field.set_description(*description);

            input.push_field(field);
        }

        for (filter, description) in ARRAY_FILTERS {
            let mut field = Field::new(*filter, format!("[{return_type}]"));
            field.set_description(*description);

            input.push_field(field);
        }

        input.push_field({
            let mut field = Field::new("contains", *return_type);
            field.set_description("Checks if the array contains all elements of the provided array");
            field
        });

        input.push_field({
            let mut field = Field::new("contained", *return_type);
            field.set_description("Checks if the array is contained within the provided array");
            field
        });

        input.push_field({
            let mut field = Field::new("overlaps", *return_type);
            field.set_description("Checks if the array has any elements in common with the provided array");
            field
        });

        input.push_field({
            let mut field = Field::new("not", input_type_name);
            field.set_description("A negation of the given filter");
            field
        });

        rendered.push_input(input);
    }

    for r#enum in database_definition.enums() {
        rendered.push_input(create_scalar_filters(r#enum.client_name()));
    }
}

fn create_scalar_filters(scalar: &str) -> InputType<'_> {
    let input_type_name = format!("{scalar}FilterInput");
    let mut input = InputType::new(input_type_name.clone());

    input.set_description(format!("Search filter input for {scalar} type."));
    input.push_directive(Directive::new("oneOf"));

    for (filter, description) in FILTERS {
        let mut field = Field::new(*filter, scalar);
        field.set_description(*description);

        input.push_field(field);
    }

    if scalar == "String" {
        input.push_field({
            let mut field = Field::new("like", scalar);
            field.set_description("The given input is part of the column value");

            field
        });
    }

    for (filter, description) in ARRAY_FILTERS {
        let mut field = Field::new(*filter, format!("[{scalar}]"));
        field.set_description(*description);

        input.push_field(field);
    }

    input.push_field({
        let mut field = Field::new("not", input_type_name);
        field.set_description("A negation of the given filter");
        field
    });

    input
}
