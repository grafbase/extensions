mod directive;
mod r#enum;
mod field;
mod input;
mod schema;
mod r#type;

use directive::{Argument, ArgumentValue, Directive};
use r#enum::{Enum, EnumVariant};
use field::Field;
use grafbase_database_definition::{DatabaseDefinition, DatabaseType};
use inflector::Inflector;
use input::InputType;
use itertools::Itertools;
use r#type::Type;

pub fn to_sdl(database_definition: DatabaseDefinition, extension_url: &str, default_schema: &str) -> String {
    let mut rendered = schema::Schema::new();

    render_schema_directives(&database_definition, extension_url, &mut rendered);
    render_enums(&database_definition, default_schema, &mut rendered);
    render_tables(&database_definition, default_schema, &mut rendered);
    render_input_types(&database_definition, &mut rendered);
    render_queries(&database_definition, &mut rendered);
    render_mutations(&database_definition, &mut rendered);

    rendered.to_string()
}

fn render_input_types<'a>(database_definition: &'a DatabaseDefinition, rendered: &mut schema::Schema<'a>) {
    const SCALARS: &[&str] = &["String", "Int", "Float", "Boolean", "JSON"];

    for scalar in SCALARS {
        let mut input = InputType::new(format!("{scalar}FilterInput"));
        input.push_directive(Directive::new("oneOf"));

        input.push_field(Field::new("eq", *scalar));
        input.push_field(Field::new("ne", *scalar));
        input.push_field(Field::new("gt", *scalar));
        input.push_field(Field::new("lt", *scalar));
        input.push_field(Field::new("gte", *scalar));
        input.push_field(Field::new("lte", *scalar));
        input.push_field(Field::new("in", format!("[{scalar}!]")));
        input.push_field(Field::new("nin", format!("[{scalar}!]")));
        input.push_field(Field::new("not", *scalar));

        if *scalar == "String" {
            input.push_field(Field::new("like", *scalar));
        }

        rendered.push_input(input);
    }

    const ARRAYS: &[&str] = &["StringArray", "IntArray", "FloatArray", "BooleanArray", "JSONArray"];

    for scalar in ARRAYS {
        let mut input = InputType::new(format!("{scalar}FilterInput"));
        input.push_directive(Directive::new("oneOf"));

        input.push_field(Field::new("eq", format!("[{scalar}!]")));
        input.push_field(Field::new("ne", format!("[{scalar}!]")));
        input.push_field(Field::new("gt", format!("[{scalar}!]")));
        input.push_field(Field::new("lt", format!("[{scalar}!]")));
        input.push_field(Field::new("gte", format!("[{scalar}!]")));
        input.push_field(Field::new("lte", format!("[{scalar}!]")));
        input.push_field(Field::new("not", format!("[{scalar}!]")));
        input.push_field(Field::new("in", format!("[[{scalar}!]!]")));
        input.push_field(Field::new("nin", format!("[[{scalar}!]!]")));
        input.push_field(Field::new("not", format!("[{scalar}!]")));
        input.push_field(Field::new("contains", format!("[{scalar}!]")));
        input.push_field(Field::new("contained", format!("[{scalar}!]")));
        input.push_field(Field::new("overlaps", format!("[{scalar}!]")));

        rendered.push_input(input);
    }

    for r#enum in database_definition.enums() {
        let mut input = InputType::new(format!("{}FilterInput", r#enum.client_name()));
        input.push_directive(Directive::new("oneOf"));

        input.push_field(Field::new("eq", r#enum.client_name()));
        input.push_field(Field::new("ne", r#enum.client_name()));
        input.push_field(Field::new("gt", r#enum.client_name()));
        input.push_field(Field::new("lt", r#enum.client_name()));
        input.push_field(Field::new("gte", r#enum.client_name()));
        input.push_field(Field::new("lte", r#enum.client_name()));
        input.push_field(Field::new("in", format!("[{}!]!", r#enum.client_name())));
        input.push_field(Field::new("nin", format!("[{}!]!", r#enum.client_name())));
        input.push_field(Field::new("not", r#enum.client_name()));

        rendered.push_input(input);
    }

    for table in database_definition.tables() {
        let mut order_input = InputType::new(table.order_by_input_name());
        order_input.push_directive(Directive::new("oneOf"));

        let mut filter_input = InputType::new(table.filter_input_name());
        filter_input.push_directive(Directive::new("oneOf"));

        for column in table.columns() {
            order_input.push_field(Field::new(column.client_name(), "OrderDirection"));

            let scalar = column.client_base_type().unwrap();
            let filter_type = if column.is_array() {
                format!("{scalar}ArrayFilterInput")
            } else {
                format!("{scalar}FilterInput")
            };

            filter_input.push_field(Field::new(column.client_name(), filter_type));
        }

        for relation in table.relations().filter(|r| r.is_other_side_one()) {
            order_input.push_field(Field::new(
                relation.client_field_name(),
                relation.referenced_table().order_by_input_name(),
            ));
        }

        rendered.push_input(order_input);

        for op in &["ALL", "NONE", "ANY"] {
            filter_input.push_field(Field::new(*op, format!("[{}]", table.filter_input_name())));
        }

        rendered.push_input(filter_input);
    }
}

fn render_schema_directives<'a>(
    database_definition: &'a DatabaseDefinition,
    extension_url: &'a str,
    rendered: &mut schema::Schema<'a>,
) {
    rendered.push_directive({
        let mut directive = Directive::new("link");

        directive.render_multiline();
        directive.push_argument(Argument::string("url", extension_url));

        let import = vec![
            ArgumentValue::String("@pgDatabase"),
            ArgumentValue::String("@pgTable"),
            ArgumentValue::String("@pgColumn"),
            ArgumentValue::String("@pgEnum"),
            ArgumentValue::String("@pgEnumVariant"),
            ArgumentValue::String("@pgRelation"),
            ArgumentValue::String("@pgKey"),
            ArgumentValue::String("@pgSelectOne"),
            ArgumentValue::String("@pgSelectMany"),
            ArgumentValue::String("@pgInsertOne"),
            ArgumentValue::String("@pgInsertMany"),
            ArgumentValue::String("@pgUpdateOne"),
            ArgumentValue::String("@pgUpdateMany"),
            ArgumentValue::String("@pgDeleteOne"),
            ArgumentValue::String("@pgDeleteMany"),
            ArgumentValue::String("PgKeyType"),
            ArgumentValue::String("PgColumnType"),
        ];

        directive.push_argument(Argument::new(
            "import",
            ArgumentValue::MultiLineArray {
                indent: "    ",
                values: import,
            },
        ));

        directive
    });

    rendered.push_directive({
        let mut directive = Directive::new("pgDatabase");
        directive.push_argument(Argument::string("name", database_definition.name()));

        directive
    });
}

fn render_enums<'a>(
    database_definition: &'a DatabaseDefinition,
    default_schema: &'a str,
    rendered: &mut schema::Schema<'a>,
) {
    rendered.push_enum({
        {
            let mut r#enum = Enum::new("OrderDirection");
            r#enum.push_variant(EnumVariant::new("ASC"));
            r#enum.push_variant(EnumVariant::new("DESC"));

            r#enum
        }
    });

    for r#enum in database_definition.enums() {
        let mut render = Enum::new(r#enum.client_name());

        render.push_directive({
            let mut directive = Directive::new("pgEnum");
            directive.push_argument(Argument::string("name", r#enum.database_name()));

            if r#enum.schema() != default_schema {
                directive.push_argument(Argument::string("schema", r#enum.schema()));
            }

            directive
        });

        for variant in r#enum.variants() {
            let mut variant_render = EnumVariant::new(variant.client_name());

            variant_render.push_directive({
                let mut directive = Directive::new("pgEnumVariant");
                directive.push_argument(Argument::string("name", variant.database_name()));
                directive
            });

            render.push_variant(variant_render);
        }

        rendered.push_enum(render);
    }
}

fn render_tables<'a>(
    database_definition: &'a DatabaseDefinition,
    default_schema: &str,
    rendered: &mut schema::Schema<'a>,
) {
    for table in database_definition.tables() {
        let mut render = Type::new(table.client_name());

        render.push_directive({
            let mut directive = Directive::new("pgTable");
            directive.push_argument(Argument::string("name", table.database_name()));

            if table.schema() != default_schema {
                directive.push_argument(Argument::string("schema", table.schema()));
            }

            directive
        });

        for key in table.keys() {
            let mut directive = Directive::new("pgKey");

            let fields = key
                .columns()
                .map(|c| ArgumentValue::String(c.table_column().client_name()))
                .collect();

            directive.push_argument(Argument::new("fields", ArgumentValue::Array(fields)));

            directive.push_argument(Argument::constant(
                "type",
                if key.is_primary() { "PRIMARY" } else { "UNIQUE" },
            ));

            render.push_directive(directive);
        }

        for column in table.columns() {
            let Some(client_type) = column.client_type(None) else {
                continue;
            };

            let mut field = Field::new(column.client_name(), client_type);

            field.push_directive({
                let mut directive = Directive::new("pgColumn");

                directive.push_argument(Argument::string("name", column.database_name()));
                directive.push_argument(Argument::constant("type", column.database_type().as_str()));

                if let DatabaseType::Enum(r#enum) = column.database_type() {
                    directive.push_argument(Argument::constant("enumType", r#enum.client_name()));

                    if r#enum.schema() != table.schema() {
                        directive.push_argument(Argument::constant("enumSchema", r#enum.schema()));
                    }
                }

                directive
            });

            render.push_field(field);
        }

        for relation in table.relations() {
            let mut field = Field::new(relation.client_field_name(), relation.client_type());

            field.push_directive({
                let mut directive = Directive::new("pgRelation");

                directive.push_argument(Argument::string("name", relation.name()));

                if relation.is_referencing_side() {
                    let mut fields = Vec::new();
                    let mut references = Vec::new();

                    for column in relation.referencing_columns() {
                        fields.push(ArgumentValue::String(column.client_name()));
                    }

                    for column in relation.referenced_columns() {
                        references.push(ArgumentValue::String(column.client_name()));
                    }

                    directive.push_argument(Argument::new("fields", ArgumentValue::Array(fields)));
                    directive.push_argument(Argument::new("references", ArgumentValue::Array(references)));
                }

                directive
            });

            render.push_field(field);
        }

        rendered.push_type(render);
    }
}

fn render_queries<'a>(database_definition: &'a DatabaseDefinition, rendered: &mut schema::Schema<'a>) {
    let mut query = Type::new("Query");

    for table in database_definition.tables() {
        for key in table.keys() {
            let field_name = format!(
                "{}_by_{}",
                table.client_name(),
                key.columns().map(|c| c.table_column().client_name()).join("_")
            )
            .to_camel_case();

            let mut field = Field::new(field_name, table.client_name());
            field.push_directive(Directive::new("pgSelectOne"));

            for column in key.columns() {
                let column = column.table_column();
                let r#type = column.client_type(None).unwrap();

                field.push_argument(Argument::constant(column.client_name(), r#type));
            }

            query.push_field(field);
        }

        let mut field = Field::new(
            table.client_name().to_plural().to_camel_case(),
            format!("{}Collection!", table.client_name()),
        );

        field.push_directive(Directive::new("pgSelectMany"));

        field.push_argument(Argument::constant("filter", table.filter_input_name()));
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

fn render_mutations(database_definition: &DatabaseDefinition, rendered: &mut schema::Schema<'_>) {
    let mut mutation = Type::new("Mutation");

    for table in database_definition.tables() {
        let mut field = Field::new(
            format!("{}_Create", table.client_name()).to_camel_case(),
            format!("{}_Create_Payload!", table.client_name()).to_pascal_case(),
        );

        field.push_directive(Directive::new("pgCreateOne"));
        field.push_argument(Argument::constant("input", format!("{}!", table.client_name())));

        mutation.push_field(field);
    }

    rendered.push_type(mutation);
}
