use std::collections::BTreeMap;

use grafbase_database_definition::{
    DatabaseDefinition, DatabaseType, RelationKind, RelationWalker, TableColumnWalker, TableWalker,
};
use itertools::Itertools;

use super::{
    EnabledOperations,
    ast::{
        directive::{Argument, ArgumentValue, Directive},
        field::Field,
        schema::Schema,
        r#type::Type,
    },
};

pub fn render<'a>(
    database_definition: &'a DatabaseDefinition,
    default_schema: &str,
    operations: EnabledOperations,
    mut derived_fields: BTreeMap<&'a str, Vec<Field<'a>>>,
    rendered: &mut Schema<'a>,
) {
    for table in database_definition.tables().filter(|t| t.allowed_in_client()) {
        let mut render = Type::new(table.client_name());
        render_directives(&mut render, default_schema, table);

        for column in table.columns() {
            render_column(&mut render, table, column);
        }

        // we do not do nested mutations, so we do not need to render relation
        // fields if no queries are allowed
        if operations.has_queries {
            for relation in table.relations() {
                render_relation(&mut render, relation);
            }
        }

        if let Some(description) = table.description() {
            render.set_description(description);
        }

        if let Some(fields) = derived_fields.remove(table.client_name()) {
            fields.into_iter().for_each(|field| {
                render.push_field(field);
            });
        }

        rendered.push_type(render);
    }
}

fn render_relation<'a>(render: &mut Type<'a>, relation: RelationWalker<'a>) {
    let mut field = Field::new(relation.client_field_name(), relation.client_type());

    field.push_directive({
        let mut directive = Directive::new("pgRelation");

        directive.push_argument(Argument::string("name", relation.name()));

        if relation.is_referencing_side() {
            let mut fields = Vec::new();
            let mut references = Vec::new();

            for column in relation.referencing_columns() {
                fields.push(ArgumentValue::String(column.client_name().into()));
            }

            for column in relation.referenced_columns() {
                references.push(ArgumentValue::String(column.client_name().into()));
            }

            directive.push_argument(Argument::new("fields", ArgumentValue::Array(fields)));
            directive.push_argument(Argument::new("references", ArgumentValue::Array(references)));
        }

        directive
    });

    if !relation.is_other_side_one() {
        field.push_argument({
            let mut argument = Argument::constant(
                "filter",
                format!("{}FilterInput", relation.referenced_table().client_name()),
            );

            argument.set_description(format!(
                "Filter the related {} instances",
                relation.referenced_table().client_name()
            ));

            argument
        });

        field.push_argument({
            let mut argument = Argument::constant("first", "Int");

            argument.set_description(format!(
                "Select the first {} instances",
                relation.referenced_table().client_name()
            ));

            argument
        });

        field.push_argument({
            let mut argument = Argument::constant("last", "Int");

            argument.set_description(format!(
                "Select the last {} instances",
                relation.referenced_table().client_name()
            ));

            argument
        });

        field.push_argument({
            let mut argument = Argument::constant("before", "String");

            argument.set_description(format!(
                "Select the {} instances before the given cursor",
                relation.referenced_table().client_name()
            ));

            argument
        });

        field.push_argument({
            let mut argument = Argument::constant("after", "String");

            argument.set_description(format!(
                "Select the {} instances after the given cursor",
                relation.referenced_table().client_name()
            ));

            argument
        });

        field.push_argument({
            let mut argument = Argument::constant(
                "orderBy",
                format!("[{}OrderByInput!]", relation.referenced_table().client_name()),
            );

            argument.set_description(format!(
                "Order the {} instances by the given fields",
                relation.referenced_table().client_name()
            ));

            argument
        });
    }

    if let Some(description) = relation.description() {
        field.set_description(description);
    }

    render.push_field(field);
}

fn render_column<'a>(render: &mut Type<'a>, table: TableWalker<'a>, column: TableColumnWalker<'a>) {
    let Some(client_type) = column.client_type(None) else {
        return;
    };

    let mut field = Field::new(column.client_name(), client_type);

    field.push_directive({
        let mut directive = Directive::new("pgColumn");

        directive.push_argument(Argument::string("name", column.database_name()));
        directive.push_argument(Argument::constant("type", column.database_type().as_str()));

        if let DatabaseType::Enum(r#enum) = column.database_type() {
            if r#enum.schema() != table.schema() {
                directive.push_argument(Argument::string("enumSchema", r#enum.schema()));
            }
        }

        directive
    });

    if let Some(description) = column.description() {
        field.set_description(description);
    }

    render.push_field(field);
}

fn render_directives<'a>(render: &mut Type<'a>, default_schema: &str, table: TableWalker<'a>) {
    for key in table.keys() {
        let mut directive = Directive::new("key");

        let fields = key.columns().map(|c| c.table_column().client_name()).join(" ");
        directive.push_argument(Argument::string("fields", fields));

        render.push_directive(directive);
    }

    render.push_directive({
        let mut directive = Directive::new("pgTable");
        directive.push_argument(Argument::string("name", table.database_name()));

        if table.schema() != default_schema {
            directive.push_argument(Argument::string("schema", table.schema()));
        }

        if table.relation_kind() != RelationKind::Relation {
            directive.push_argument(Argument::constant("kind", table.relation_kind().client_name()));
        }

        directive
    });

    for key in table.keys() {
        let mut directive = Directive::new("pgKey");

        let fields = key
            .columns()
            .map(|c| ArgumentValue::String(c.table_column().client_name().into()))
            .collect();

        directive.push_argument(Argument::new("fields", ArgumentValue::Array(fields)));

        directive.push_argument(Argument::constant(
            "type",
            if key.is_primary() { "PRIMARY" } else { "UNIQUE" },
        ));

        render.push_directive(directive);
    }
}
