use anyhow::bail;
use grafbase_database_definition::{DatabaseDefinition, Key, KeyColumn, KeyType};
use sqlx::{PgConnection, Row};

use crate::config::Config;

pub(crate) async fn introspect_database(
    conn: &mut PgConnection,
    config: &Config,
    database_definition: &mut DatabaseDefinition,
) -> anyhow::Result<()> {
    introspect_sql(conn, database_definition).await?;
    introspect_overrides(config, database_definition)?;

    Ok(())
}

async fn introspect_sql(
    conn: &mut PgConnection,
    database_definition: &mut DatabaseDefinition,
) -> Result<(), anyhow::Error> {
    let query = indoc::indoc! {r#"
        WITH rawindex AS (SELECT indrelid,
                                 indexrelid,
                                 indisprimary,
                                 unnest(indkey)                 AS indkeyid,
                                 generate_subscripts(indkey, 1) AS indkeyidx
                          FROM pg_index
                          WHERE indpred IS NULL -- filter out partial indexes
                            AND NOT indisexclusion -- filter out exclusion constraints
                            AND (indisunique OR indisprimary)
        )
        SELECT schemainfo.nspname    AS schema,
               indexinfo.relname     AS constraint_name,
               tableinfo.relname     AS table_name,
               columninfo.attname    AS column_name,
               rawindex.indisprimary AS is_primary_key
        FROM rawindex

        INNER JOIN pg_class AS tableinfo ON tableinfo.oid = rawindex.indrelid
        INNER JOIN pg_class AS indexinfo ON indexinfo.oid = rawindex.indexrelid
        INNER JOIN pg_namespace AS schemainfo ON schemainfo.oid = tableinfo.relnamespace

        LEFT JOIN pg_attribute AS columninfo
            ON columninfo.attrelid = tableinfo.oid AND columninfo.attnum = rawindex.indkeyid

        WHERE schemainfo.nspname <> ALL ( $1 )
        ORDER BY schema, table_name, constraint_name, rawindex.indkeyidx;
    "#};

    let rows = sqlx::query(query)
        .bind(super::blocked_schemas())
        .fetch_all(conn)
        .await?;

    for row in rows {
        let Some(schema_id) = database_definition.get_schema_id(row.get(0)) else {
            continue;
        };

        let Some(table_id) = database_definition.get_table_id(schema_id, row.get(2)) else {
            continue;
        };

        let Some(column_name): Option<&str> = row.get(3) else {
            continue;
        };

        let Some(column_id) = database_definition.get_table_column_id(table_id, column_name) else {
            continue;
        };

        let constraint_id = match database_definition.get_key_id(table_id, row.get(1)) {
            Some(id) => id,
            None => {
                let key_type = if row.get(4) { KeyType::Primary } else { KeyType::Unique };
                let key = Key::new(table_id, row.get(1), key_type);

                database_definition.push_key(key)
            }
        };

        let column = KeyColumn::new(constraint_id, column_id);
        database_definition.push_key_column(column);
    }

    Ok(())
}

fn introspect_overrides(config: &Config, database_definition: &mut DatabaseDefinition) -> anyhow::Result<()> {
    for (schema, schema_config) in &config.schemas {
        let Some(schema_id) = database_definition.get_schema_id(schema) else {
            bail!("Schema `{schema}` not found. Check your configuration.")
        };

        for (view, view_config) in &schema_config.views {
            let Some(table_id) = database_definition.get_table_id(schema_id, view) else {
                bail!("View `{view}` not found in schema `{schema}`. Check your configuration.")
            };

            if let Some(keys) = view_config.unique_keys.as_ref() {
                for key in keys {
                    let mut columns = Vec::new();

                    for column in key {
                        let Some(column_id) = database_definition.get_table_column_id(table_id, column) else {
                            bail!("Column `{column}` not found in view `{view}`. Check your configuration.")
                        };

                        columns.push(column_id);
                    }

                    let key_name = format!("{schema}_{view}_{}", key.join("_"));
                    let key_id = database_definition.push_key(Key::new(table_id, key_name, KeyType::Unique));

                    for column_id in columns {
                        let column = KeyColumn::new(key_id, column_id);
                        database_definition.push_key_column(column);
                    }
                }
            }

            for (column, column_config) in &view_config.columns {
                if !column_config.unique {
                    continue;
                }

                let Some(column_id) = database_definition.get_table_column_id(table_id, column) else {
                    continue;
                };

                let key_name = format!("{schema}_{view}_{column}");
                let key = Key::new(table_id, key_name, KeyType::Unique);
                let key_id = database_definition.push_key(key);

                let column = KeyColumn::new(key_id, column_id);
                database_definition.push_key_column(column);
            }
        }
    }

    Ok(())
}
