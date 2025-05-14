use std::borrow::Cow;

use grafbase_database_definition::{DatabaseDefinition, TableWalker};
use inflector::Inflector;
use itertools::Itertools;

use crate::{
    config::Config,
    render::ast::{directive::Directive, field::Field, input::InputType},
};

use super::{EnabledOperations, ast::schema::Schema};

const SCALARS: &[&str] = &[
    "String",
    "BigInt",
    "Int",
    "Float",
    "Boolean",
    "Decimal",
    "Bytes",
    "JSON",
    "UUID",
    "Date",
    "Time",
    "TimeWithTimezone",
    "Timestamp",
    "DateTime",
    "Inet",
    "CIDR",
    "MacAddr",
    "Money",
    "BitString",
    "XML",
];

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
    ("[BigInt]", "BigInt"),
    ("[Decimal]", "Decimal"),
    ("[Float]", "Float"),
    ("[Boolean]", "Boolean"),
    ("[Bytes]", "Bytes"),
    ("[JSON]", "JSON"),
    ("[UUID]", "UUID"),
    ("[Date]", "Date"),
    ("[Time]", "Time"),
    ("[TimeWithTimezone]", "TimeWithTimezone"),
    ("[Timestamp]", "Timestamp"),
    ("[DateTime]", "DateTime"),
    ("[Inet]", "Inet"),
    ("[CIDR]", "CIDR"),
    ("[MacAddr]", "MacAddr"),
    ("[Money]", "Money"),
    ("[BitString]", "BitString"),
    ("[XML]", "XML"),
];

static NUMERIC_SCALARS: &[&str] = &["BigInt", "Float", "Decimal", "Int", "Money"];

pub fn render<'a>(
    database_definition: &'a DatabaseDefinition,
    config: &Config,
    operations: &mut EnabledOperations,
    prefix: Option<&str>,
    rendered: &mut Schema<'a>,
) {
    for table in database_definition.tables().filter(|t| t.allowed_in_client()) {
        if config.queries_allowed(table) {
            operations.has_queries = true;
            render_order_input(rendered, table);
            render_many_lookup_input(rendered, table);
        }

        render_lookup_input(rendered, table);
        render_filter_input(rendered, table);

        if config.mutations_allowed(table) {
            operations.has_mutations = true;

            render_create_input(rendered, prefix, table);
            render_update_input(rendered, prefix, table);
        }
    }

    render_scalar_inputs(database_definition, operations, rendered);
}

fn render_update_input<'a>(rendered: &mut Schema<'a>, prefix: Option<&str>, table: TableWalker<'a>) {
    let type_name = match prefix {
        Some(prefix) => format!("{}_{}_UpdateInput", prefix, table.client_name()).to_pascal_case(),
        None => format!("{}UpdateInput", table.client_name()),
    };

    let mut mutation_input = InputType::new(type_name);
    mutation_input.set_description(format!("Input for updating an existing {}", table.client_name()));

    for column in table.columns() {
        if !column.allows_user_input() {
            continue;
        }

        let type_name = if column.is_array() {
            format!("{}ArrayUpdateInput", column.client_base_type().unwrap())
        } else {
            format!("{}UpdateInput", column.client_base_type().unwrap())
        };

        mutation_input.push_field({
            let mut field = Field::new(column.client_name(), type_name);
            field.set_description(format!("Update field value for {}", column.client_name()));
            field
        });
    }

    rendered.push_input(mutation_input);
}

fn render_create_input<'a>(rendered: &mut Schema<'a>, prefix: Option<&str>, table: TableWalker<'a>) {
    let type_name = match prefix {
        Some(prefix) => format!("{}_{}_CreateInput", prefix, table.client_name()).to_pascal_case(),
        None => format!("{}CreateInput", table.client_name()),
    };

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

fn render_many_lookup_input<'a>(rendered: &mut Schema<'a>, table: TableWalker<'a>) {
    let type_name = format!("{}ManyLookupInput", table.client_name());
    let mut input = InputType::new(type_name.clone());

    input.push_directive(Directive::new("oneOf"));
    input.push_directive(Directive::new("inaccessible"));

    input.set_description(format!(
        "Lookup input type for {} objects for subgraph joins.",
        table.client_name()
    ));

    for key in table.keys() {
        if key.columns().count() == 1 {
            let column = key.columns().next().unwrap().table_column();

            input.push_field({
                let type_name = column.client_base_type().unwrap();
                let mut field = Field::new(column.client_name(), format!("[{}!]", type_name));

                field.push_directive(Directive::new("inaccessible"));
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

            let type_name = format!("[{}!]", type_name);

            input.push_field({
                let field_name = key
                    .columns()
                    .map(|c| c.table_column().client_name())
                    .join("_")
                    .to_camel_case();

                let mut field = Field::new(field_name, type_name);
                field.push_directive(Directive::new("inaccessible"));

                field.set_description(format!(
                    "Select {} by composite columns '{}'",
                    table.client_name(),
                    key.columns().map(|c| c.table_column().client_name()).join(", ")
                ));

                field
            });
        }
    }

    rendered.push_input(input);
}

fn render_filter_input<'a>(rendered: &mut Schema<'a>, table: TableWalker<'a>) {
    let filter_name = format!("{}FilterInput", table.client_name());
    let mut filter_input = InputType::new(filter_name.clone());

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
        let mut field = Field::new("contains", filter_name.clone());
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
            let mut field = Field::new(*op, format!("[{}]", filter_name.clone()));
            field.set_description(*desc);
            field
        });
    }

    rendered.push_input(filter_input);
}

fn render_lookup_input<'a>(rendered: &mut Schema<'a>, table: TableWalker<'a>) {
    let lookup_name = format!("{}LookupInput", table.client_name());

    let mut filter_input = InputType::new(lookup_name);
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

fn render_scalar_inputs<'a>(
    database_definition: &'a DatabaseDefinition,
    operations: &EnabledOperations,
    rendered: &mut Schema<'a>,
) {
    for scalar in SCALARS {
        rendered.push_input(create_scalar_filters(scalar));

        if operations.has_mutations {
            rendered.push_input(create_scalar_update_input(scalar));
            rendered.push_input(create_array_update_type(scalar));
        }
    }

    for (return_type, scalar) in ARRAYS {
        rendered.push_input(create_scalar_array_filters(scalar, *return_type));
    }

    for r#enum in database_definition.enums() {
        rendered.push_input(create_scalar_filters(r#enum.client_name()));

        let array_type = format!("[{}]", r#enum.client_name());
        rendered.push_input(create_scalar_array_filters(r#enum.client_name(), array_type));

        if operations.has_mutations {
            rendered.push_input(create_scalar_update_input(r#enum.client_name()));
            rendered.push_input(create_array_update_type(r#enum.client_name()));
        }
    }
}

fn create_array_update_type(scalar: &str) -> InputType<'_> {
    let input_type_name = format!("{scalar}ArrayUpdateInput");
    let mut input = InputType::new(input_type_name.clone());

    input.set_description(format!("Update input for {scalar} array type."));
    input.push_directive(Directive::new("oneOf"));

    input.push_field({
        let mut field = Field::new("set", format!("[{scalar}]"));
        field.set_description("Replaces the value of a field with the specified value.");
        field
    });

    input.push_field({
        let mut field = Field::new("append", format!("[{scalar}]"));
        field.set_description("Append an array value to the column.");
        field
    });

    input.push_field({
        let mut field = Field::new("prepend", format!("[{scalar}]"));
        field.set_description("Prepend an array value to the column.");
        field
    });

    input
}

fn create_scalar_update_input(scalar: &str) -> InputType<'_> {
    let input_type_name = format!("{scalar}UpdateInput");
    let mut input = InputType::new(input_type_name.clone());

    input.set_description(format!("Update input for {scalar} type."));
    input.push_directive(Directive::new("oneOf"));

    input.push_field({
        let mut field = Field::new("set", scalar);
        field.set_description("Replaces the value of a field with the specified value.");
        field
    });

    if NUMERIC_SCALARS.contains(&scalar) {
        input.push_field({
            let mut field = Field::new("increment", scalar);
            field.set_description("Increments the value of a field by the specified value.");
            field
        });

        input.push_field({
            let mut field = Field::new("decrement", scalar);
            field.set_description("Decrements the value of a field by the specified value.");
            field
        });

        input.push_field({
            let mut field = Field::new("multiply", scalar);
            field.set_description("Multiplies the value of a field by the specified value.");
            field
        });

        input.push_field({
            let mut field = Field::new("divide", scalar);
            field.set_description("Divides the value of a field by the specified value.");
            field
        });
    }

    if scalar == "JSON" {
        input.push_field({
            let mut field = Field::new("append", scalar);
            field.set_description("Append JSON value to the column.");
            field
        });

        input.push_field({
            let mut field = Field::new("prepend", scalar);
            field.set_description("Prepend JSON value to the column.");
            field
        });

        input.push_field({
            let mut field = Field::new("deleteKey", "String");

            field.set_description(
                "Deletes a key (and its value) from a JSON object, or matching string value(s) from a JSON array.",
            );

            field
        });

        input.push_field({
            let mut field = Field::new("deleteElem", "Int");

            field.set_description(
                "Deletes the array element with specified index (negative integers count from the end). Throws an error if JSON value is not an array.",
            );

            field
        });

        input.push_field({
            let mut field = Field::new("deleteAtPath", "[String!]");

            field.set_description(
                "Deletes the field or array element at the specified path, where path elements can be either field keys or array indexes.",
            );

            field
        });
    }

    input
}

fn create_scalar_array_filters<'a>(scalar: &'a str, return_type: impl Into<Cow<'a, str>>) -> InputType<'a> {
    let return_type = return_type.into();
    let input_type_name = format!("{scalar}ArrayFilterInput");
    let mut input = InputType::new(input_type_name.clone());

    input.set_description(format!("Search filter input for {scalar} array type."));
    input.push_directive(Directive::new("oneOf"));

    for (filter, description) in FILTERS {
        let mut field = Field::new(*filter, return_type.clone());
        field.set_description(*description);

        input.push_field(field);
    }

    for (filter, description) in ARRAY_FILTERS {
        let mut field = Field::new(*filter, format!("[{return_type}!]"));
        field.set_description(*description);

        input.push_field(field);
    }

    input.push_field({
        let mut field = Field::new("contains", return_type.clone());
        field.set_description("Checks if the array contains all elements of the provided array");
        field
    });

    input.push_field({
        let mut field = Field::new("contained", return_type.clone());
        field.set_description("Checks if the array is contained within the provided array");
        field
    });

    input.push_field({
        let mut field = Field::new("overlaps", return_type.clone());
        field.set_description("Checks if the array has any elements in common with the provided array");
        field
    });

    input.push_field({
        let mut field = Field::new("not", input_type_name);
        field.set_description("A negation of the given filter");
        field
    });

    input
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
        // never ever allow nulls in array filters. we use any/all instead of IN,
        // and a null value in the array will cause very interesting behavior...
        let mut field = Field::new(*filter, format!("[{scalar}!]"));
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
