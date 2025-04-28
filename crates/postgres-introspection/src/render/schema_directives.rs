use grafbase_database_definition::DatabaseDefinition;

use super::ast::{
    directive::{Argument, ArgumentValue, Directive},
    schema::Schema,
};

pub fn render<'a>(database_definition: &'a DatabaseDefinition, extension_url: &'a str, rendered: &mut Schema<'a>) {
    rendered.push_directive({
        let mut directive = Directive::new("link");

        directive.render_multiline();
        directive.push_argument(Argument::string("url", extension_url));

        let import = vec![
            ArgumentValue::String("@pgDatabase".into()),
            ArgumentValue::String("@pgTable".into()),
            ArgumentValue::String("@pgColumn".into()),
            ArgumentValue::String("@pgEnum".into()),
            ArgumentValue::String("@pgEnumVariant".into()),
            ArgumentValue::String("@pgRelation".into()),
            ArgumentValue::String("@pgKey".into()),
            ArgumentValue::String("@pgLookup".into()),
            ArgumentValue::String("@pgSelectOne".into()),
            ArgumentValue::String("@pgSelectMany".into()),
            ArgumentValue::String("@pgInsertOne".into()),
            ArgumentValue::String("@pgInsertMany".into()),
            ArgumentValue::String("@pgUpdateOne".into()),
            ArgumentValue::String("@pgUpdateMany".into()),
            ArgumentValue::String("@pgDeleteOne".into()),
            ArgumentValue::String("@pgDeleteMany".into()),
            ArgumentValue::String("@pgConnection".into()),
            ArgumentValue::String("@pgMutation".into()),
            ArgumentValue::String("@pgReturning".into()),
            ArgumentValue::String("PgKeyType".into()),
            ArgumentValue::String("PgColumnType".into()),
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
        let mut directive = Directive::new("link");
        directive.render_multiline();

        directive.push_argument(Argument::string(
            "url",
            "https://specs.grafbase.com/composite-schema/v1",
        ));

        directive.push_argument(Argument::new(
            "import",
            ArgumentValue::MultiLineArray {
                indent: "    ",
                values: vec![
                    ArgumentValue::String("@lookup".into()),
                    ArgumentValue::String("@key".into()),
                ],
            },
        ));

        directive
    });

    rendered.push_directive({
        let mut directive = Directive::new("link");
        directive.render_multiline();

        directive.push_argument(Argument::string("url", "https://specs.apollo.dev/federation/v2.3"));

        directive.push_argument(Argument::new(
            "import",
            ArgumentValue::MultiLineArray {
                indent: "    ",
                values: vec![
                    ArgumentValue::String("@shareable".into()),
                    ArgumentValue::String("@inaccessible".into()),
                ],
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
