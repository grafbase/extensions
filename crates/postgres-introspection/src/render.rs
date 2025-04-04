mod ast;
mod enums;
mod input_types;
mod mutation;
mod output_types;
mod query;
mod scalars;
mod schema_directives;
mod tables;

use ast::schema::Schema;
use grafbase_database_definition::DatabaseDefinition;

pub fn to_sdl(database_definition: DatabaseDefinition, extension_url: &str, default_schema: &str) -> String {
    let mut rendered = Schema::new();

    scalars::render(&mut rendered);
    schema_directives::render(&database_definition, extension_url, &mut rendered);
    enums::render(&database_definition, default_schema, &mut rendered);
    input_types::render(&database_definition, &mut rendered);
    output_types::render(&database_definition, &mut rendered);
    tables::render(&database_definition, default_schema, &mut rendered);
    query::render(&database_definition, &mut rendered);
    mutation::render(&database_definition, &mut rendered);

    rendered.to_string()
}
