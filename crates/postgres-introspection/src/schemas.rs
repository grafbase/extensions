use grafbase_database_definition::DatabaseDefinition;
use sqlx::{PgConnection, Row};

pub(crate) async fn introspect_database(
    conn: &mut PgConnection,
    database_definition: &mut DatabaseDefinition,
) -> anyhow::Result<()> {
    let query = "SELECT nspname AS name FROM pg_namespace WHERE nspname <> ALL ($1) ORDER BY name";

    let rows = sqlx::query(query)
        .bind(super::blocked_schemas())
        .fetch_all(conn)
        .await?;

    for row in rows {
        database_definition.push_schema(row.get(0));
    }

    Ok(())
}
