use crate::config::Config;
use grafbase_database_definition::{DatabaseDefinition, RelationKind, Table};
use sqlx::{PgConnection, Row};

pub(crate) async fn introspect_database(
    conn: &mut PgConnection,
    config: &Config,
    database_definition: &mut DatabaseDefinition,
) -> anyhow::Result<()> {
    let query = indoc::indoc! {r#"
        SELECT
          pg_class.relname AS name,
          pg_namespace.nspname AS schema,
          pg_description.description AS description,
          pg_class.relkind AS relation_kind
        FROM pg_class
        INNER JOIN pg_namespace ON pg_namespace.oid = pg_class.relnamespace
        LEFT JOIN pg_description ON pg_description.objoid = pg_class.oid AND pg_description.objsubid = 0
        WHERE pg_class.relkind IN ('r', 'v', 'm')
        AND pg_namespace.nspname <> ALL ( $1 )
        ORDER BY CASE pg_class.relkind
            WHEN 'r' THEN 1
            ELSE 2
          END,
          schema,
          name;
    "#};

    let rows = sqlx::query(query)
        .bind(super::blocked_schemas())
        .fetch_all(conn)
        .await?;

    for row in rows {
        let schema_name: String = row.get(1);
        let table_name: String = row.get(0);

        let Some(schema_id) = database_definition.get_schema_id(&schema_name) else {
            continue;
        };

        // Skip tables that are not in the allowlist or are in the denylist
        if !config.is_table_included(&schema_name, &table_name) {
            continue;
        };

        let kind = match row.get::<i8, _>(3) as u8 as char {
            'r' => RelationKind::Relation,
            'v' => RelationKind::View,
            'm' => RelationKind::MaterializedView,
            _ => unreachable!(),
        };

        let mut table = Table::<String>::new(schema_id, table_name, kind, None);

        if let Some(description) = row.get(2) {
            table.set_description(description);
        }

        database_definition.push_table(table);
    }

    Ok(())
}
