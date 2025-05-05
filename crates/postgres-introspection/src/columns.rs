use anyhow::bail;
use grafbase_database_definition::DatabaseDefinition;
use sqlx::{PgConnection, Row};
use std::str::FromStr;

use crate::config::Config;

pub(crate) async fn introspect_database(
    conn: &mut PgConnection,
    config: &Config,
    database_definition: &mut DatabaseDefinition,
) -> anyhow::Result<()> {
    introspect_sql(conn, config, database_definition).await?;
    check_overriden_columns(config, database_definition)?;

    Ok(())
}

async fn introspect_sql(
    conn: &mut PgConnection,
    config: &Config,
    database_definition: &mut DatabaseDefinition,
) -> Result<(), anyhow::Error> {
    use grafbase_database_definition::{ColumnType, EnumType, IdentityGeneration, ScalarType, TableColumn};

    let query = indoc::indoc! {r#"
        SELECT n.nspname                            AS schema,      -- Schema name from pg_namespace
               c.relname                            AS table_name,  -- Relation name (table, view, mview) from pg_class
               a.attname                            AS column_name, -- Attribute (column) name from pg_attribute
               t.oid::int4                          AS type_id,     -- Type OID from pg_type
               t.typname                            AS type_name,   -- Type name from pg_type
               tn.nspname                           AS type_schema, -- Schema name of the type from pg_namespace
               -- Determine if it's an array: check if base type ('b') and has an element type (typelem != 0)
               (t.typtype = 'b' AND t.typelem != 0) AS is_array,
               -- Check if a default value exists by looking for an entry in pg_attrdef
               def.adrelid IS NOT NULL              AS has_default,
               -- Determine nullability directly from pg_attribute.
               -- NOTE: For standard views (relkind='v'), this might report FALSE
               -- even if the underlying column is NOT NULL. Configuration override might still be needed for views.
               NOT a.attnotnull                     AS is_nullable,
               -- Get identity generation info from information_schema (requires LEFT JOIN)
               isc.identity_generation,
               -- Get column description from pg_description
               d.description,
               -- Include the relation kind (r=table, v=view, m=materialized view)
               c.relkind
        FROM
            -- Start with relations (tables, views, mviews)
            pg_catalog.pg_class c
                JOIN
            -- Get schema name
                pg_catalog.pg_namespace n ON n.oid = c.relnamespace
                JOIN
            -- Get columns for these relations
                pg_catalog.pg_attribute a ON a.attrelid = c.oid
                JOIN
            -- Get type information for each column
                pg_catalog.pg_type t ON t.oid = a.atttypid
                JOIN
            -- Get the schema name for the type
                pg_catalog.pg_namespace tn ON tn.oid = t.typnamespace
                LEFT JOIN
            -- Check for default values
                pg_catalog.pg_attrdef def ON def.adrelid = a.attrelid AND def.adnum = a.attnum
                LEFT JOIN
            -- Get column comments/descriptions
                pg_catalog.pg_description d ON d.objoid = a.attrelid AND d.objsubid = a.attnum
                LEFT JOIN
            -- LEFT JOIN information_schema.columns primarily to get identity_generation
            -- This join might fail for materialized views in your env, resulting in NULL for identity_generation
                information_schema.columns isc ON isc.table_schema = n.nspname
                AND isc.table_name = c.relname
                AND isc.column_name = a.attname
        WHERE
          -- Filter for Tables, Views, and Materialized Views
            c.relkind IN ('r', 'v', 'm')
          -- Filter out system schemas
          AND n.nspname <> ALL ( $1 )
          -- Filter out system columns (like ctid, oid, etc.)
          AND a.attnum > 0
          -- Filter out dropped columns
          AND NOT a.attisdropped
        ORDER BY  CASE WHEN c.relkind = 'r' THEN 1 ELSE 2 END,
                 schema,
                 table_name,
                 -- Order by column number to maintain logical order
                 a.attnum;

    "#};

    let rows = sqlx::query(query)
        .bind(super::blocked_schemas())
        .fetch_all(conn)
        .await?;

    for row in rows {
        let schema_name = row.get(0);
        let Some(schema_id) = database_definition.get_schema_id(schema_name) else {
            continue;
        };

        let table_name = row.get(1);
        let Some(table_id) = database_definition.get_table_id(schema_id, table_name) else {
            continue;
        };

        // If the type is an array, it's named `_type` in the database. We don't need that info in the type
        // name, we store enums without an underscore in our interner.
        let type_name = row.get::<&str, _>(4).trim_start_matches('_');

        let enum_id = database_definition
            .get_schema_id(row.get(5))
            .and_then(|enum_schema_id| database_definition.get_enum_id(enum_schema_id, type_name));

        let database_type = match enum_id {
            Some(id) => ColumnType::Enum(EnumType {
                id,
                is_array: row.get(6),
            }),
            None => ColumnType::Scalar(ScalarType::from(row.get::<i32, _>(3) as u32)),
        };

        let column_name: &str = row.get(2);

        let column_config = config
            .schemas
            .get(schema_name)
            .and_then(|s| s.views.get(table_name))
            .and_then(|v| v.columns.get(column_name));

        match column_config {
            Some(config) => {
                let mut column =
                    TableColumn::new(table_id, database_type, column_name.to_string(), config.rename.clone());

                column.set_nullable(config.nullable);

                if let Some(description) = row.get(10) {
                    column.set_description(description);
                }

                database_definition.push_table_column(column, None);
            }
            None => {
                let mut column = TableColumn::new(table_id, database_type, column_name.to_string(), None);

                column.set_nullable(row.get(8));
                column.set_has_default(row.get(7));

                if let Some(s) = row.get(9) {
                    column.set_identity_generation(IdentityGeneration::from_str(s)?);
                }

                if let Some(description) = row.get(10) {
                    column.set_description(description);
                }

                database_definition.push_table_column(column, None);
            }
        }
    }

    Ok(())
}

fn check_overriden_columns(config: &Config, database_definition: &mut DatabaseDefinition) -> anyhow::Result<()> {
    for (schema, schema_config) in &config.schemas {
        let Some(schema_id) = database_definition.get_schema_id(schema) else {
            bail!("Schema `{schema}` not found. Check your configuration.")
        };

        for (view, view_config) in &schema_config.views {
            let Some(table_id) = database_definition.get_table_id(schema_id, view) else {
                bail!("View `{view}` not found in schema `{schema}`. Check your configuration.")
            };

            for column in view_config.columns.keys() {
                if database_definition.get_table_column_id(table_id, column).is_none() {
                    bail!("Column `{column}` not found in view `{view}`. Check your configuration.")
                }
            }
        }
    }

    Ok(())
}
