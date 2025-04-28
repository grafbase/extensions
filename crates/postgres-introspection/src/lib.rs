use grafbase_database_definition::DatabaseDefinition;

mod columns;
mod enums;
mod foreign_keys;
mod keys;
mod render;
mod schemas;
mod tables;

/// Options for Postgres introspection.
pub struct IntrospectionOptions<'a> {
    /// Name of the database to introspect.
    pub database_name: &'a str,
    /// URL of the extension to use.
    pub extension_url: &'a str,
    /// Default schema to use and omit from the SDL output.
    pub default_schema: &'a str,
}

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
pub async fn introspect(conn: &mut sqlx::PgConnection, opts: IntrospectionOptions<'_>) -> anyhow::Result<String> {
    let mut database_definition = DatabaseDefinition::new(opts.database_name.to_string());

    schemas::introspect_database(conn, &mut database_definition).await?;
    enums::introspect_database(conn, &mut database_definition).await?;
    tables::introspect_database(conn, &mut database_definition).await?;
    columns::introspect_database(conn, &mut database_definition).await?;
    foreign_keys::introspect_database(conn, &mut database_definition).await?;
    keys::introspect_database(conn, &mut database_definition).await?;

    database_definition.finalize();

    Ok(render::to_sdl(
        database_definition,
        opts.extension_url,
        opts.default_schema,
        opts.database_name,
    ))
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
