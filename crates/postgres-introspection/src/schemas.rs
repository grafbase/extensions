use grafbase_database_definition::DatabaseDefinition;
use sqlx::{PgConnection, Row};

use crate::config::Config;

pub(crate) async fn introspect_database(
    conn: &mut PgConnection,
    config: &Config,
    database_definition: &mut DatabaseDefinition,
) -> anyhow::Result<()> {
    let query = "SELECT nspname AS name FROM pg_namespace WHERE nspname <> ALL ($1) ORDER BY name";

    let rows = sqlx::query(query)
        .bind(super::blocked_schemas())
        .fetch_all(conn)
        .await?;

    for row in rows {
        let schema_name: String = row.get(0);

        if config.is_schema_included(&schema_name) {
            database_definition.push_schema(row.get(0));
        }
    }

    Ok(())
}
