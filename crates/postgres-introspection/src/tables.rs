use grafbase_database_definition::{DatabaseDefinition, Table};
use sqlx::{PgConnection, Row};

pub(crate) async fn introspect_database(
    conn: &mut PgConnection,
    database_definition: &mut DatabaseDefinition,
) -> anyhow::Result<()> {
    let query = indoc::indoc! {r#"
        SELECT
          pg_class.relname AS name,
          pg_namespace.nspname AS schema,
          pg_description.description AS description
        FROM pg_class
        INNER JOIN pg_namespace ON pg_namespace.oid = pg_class.relnamespace
        LEFT JOIN pg_description ON pg_description.objoid = pg_class.oid AND pg_description.objsubid = 0
        WHERE pg_class.relkind = 'r' -- r = relation, e.g. a table
        AND pg_namespace.nspname <> ALL ( $1 )
        ORDER BY schema, name;
    "#};

    let rows = sqlx::query(query)
        .bind(super::blocked_schemas())
        .fetch_all(conn)
        .await?;

    for row in rows {
        let Some(schema_id) = database_definition.get_schema_id(row.get(1)) else {
            continue;
        };

        let mut table = Table::<String>::new(schema_id, row.get(0), None);

        if let Some(description) = row.get(2) {
            table.set_description(description);
        }

        database_definition.push_table(table);
    }

    Ok(())
}
