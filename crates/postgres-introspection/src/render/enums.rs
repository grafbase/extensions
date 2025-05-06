use grafbase_database_definition::DatabaseDefinition;

use super::{
    EnabledOperations,
    ast::{
        directive::{Argument, Directive},
        r#enum::{Enum, EnumVariant},
        schema::Schema,
    },
};

pub fn render<'a>(
    database_definition: &'a DatabaseDefinition,
    default_schema: &'a str,
    operations: &EnabledOperations,
    rendered: &mut Schema<'a>,
) {
    if operations.has_queries {
        rendered.push_enum({
            let mut r#enum = Enum::new("OrderDirection");
            r#enum.set_description("Specifies the direction for ordering results.");

            for (variant, description) in [
                ("ASC", "Specifies an ascending order for a given orderBy argument."),
                ("DESC", "Specifies a descending order for a given orderBy argument."),
            ] {
                let mut variant = EnumVariant::new(variant);
                variant.set_description(description);

                r#enum.push_variant(variant);
            }

            r#enum
        });
    }

    for r#enum in database_definition.enums() {
        let mut render = Enum::new(r#enum.client_name());

        if let Some(description) = r#enum.description() {
            render.set_description(description);
        }

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

            if let Some(description) = variant.description() {
                variant_render.set_description(description);
            }

            render.push_variant(variant_render);
        }

        rendered.push_enum(render);
    }
}
