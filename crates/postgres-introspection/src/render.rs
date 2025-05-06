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

use crate::config::Config;

const DEFAULT_DATABASE_NAME: &str = "default";

/// Defines if at least one table has queries or mutations enabled.
#[derive(Debug, Clone, Copy)]
struct EnabledOperations {
    has_queries: bool,
    has_mutations: bool,
}

pub fn to_sdl(database_definition: DatabaseDefinition, config: &Config) -> String {
    let database_name = config.database_name.as_str();
    let extension_url = config.extension_url.as_str();
    let default_schema = config.default_schema.as_str();

    let mut rendered = Schema::new();

    let mut operations = EnabledOperations {
        has_queries: false,
        has_mutations: false,
    };

    let prefix = if database_name == DEFAULT_DATABASE_NAME {
        None
    } else {
        Some(database_name)
    };

    scalars::render(&mut rendered);
    schema_directives::render(&database_definition, extension_url, &mut rendered);
    input_types::render(&database_definition, config, &mut operations, prefix, &mut rendered);
    enums::render(&database_definition, default_schema, &operations, &mut rendered);
    output_types::render(&database_definition, config, operations, &mut rendered);
    tables::render(&database_definition, default_schema, operations, &mut rendered);

    if operations.has_queries {
        query::render(&database_definition, config, prefix, &mut rendered);
    }

    if operations.has_mutations {
        mutation::render(&database_definition, config, prefix, &mut rendered);
    }

    rendered.to_string()
}
