use config::Config;
use grafbase_database_definition::DatabaseDefinition;

mod columns;
pub mod config;
mod enums;
mod foreign_keys;
mod keys;
mod render;
mod schemas;
mod tables;

/// Introspects a PostgreSQL database schema.
///
/// This function connects to a PostgreSQL database and retrieves information about
/// its schema including schemas, enums, tables, columns, foreign keys, and primary keys.
/// It then renders this information as an SDL (Schema Definition Language) string.
///
/// # Arguments
///
/// * `conn` - A mutable reference to an active PostgreSQL connection.
/// * `opts` - Options for customizing the introspection process.
pub async fn introspect(conn: &mut sqlx::PgConnection, config: Config) -> anyhow::Result<String> {
    let mut database_definition = DatabaseDefinition::new(config.database_name.clone());

    schemas::introspect_database(conn, &config, &mut database_definition).await?;
    enums::introspect_database(conn, &mut database_definition).await?;
    tables::introspect_database(conn, &config, &mut database_definition).await?;
    columns::introspect_database(conn, &config, &mut database_definition).await?;
    foreign_keys::introspect_database(conn, &config, &mut database_definition).await?;
    keys::introspect_database(conn, &config, &mut database_definition).await?;

    database_definition.finalize();

    render::to_sdl(database_definition, &config)
}

/// A list of schemas to filter out automatically on every introspection.
static BLOCKED_SCHEMAS: &[&str] = &["pg_catalog", "pg_toast", "information_schema"];

fn blocked_schemas() -> Vec<String> {
    static SCHEMAS: std::sync::OnceLock<Vec<String>> = std::sync::OnceLock::new();

    let result = SCHEMAS
        .get_or_init(|| BLOCKED_SCHEMAS.iter().map(|schema| (*schema).to_string()).collect())
        .clone();

    result
}
