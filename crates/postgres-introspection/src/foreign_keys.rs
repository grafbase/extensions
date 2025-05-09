use std::collections::BTreeMap;

use anyhow::bail;
use grafbase_database_definition::{DatabaseDefinition, ForeignKey, ForeignKeyColumn, SchemaId};
use sqlx::{PgConnection, Row};

use crate::config::{Config, RelationConfig};

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
        SELECT "constraint".conname               AS constraint_name,           -- 0
               "constraint".schema                AS constrained_schema,        -- 1
               "constraint".table_name            AS constrained_table_name,    -- 2
               child_attribute.attname            AS constrained_column_name,   -- 3
               relation_namespace.nspname         AS referenced_schema,         -- 4
               parent_class.relname               AS referenced_table_name,     -- 5
               parent_attribute.attname           AS referenced_column_name,    -- 6
               pg_catalog.obj_description("constraint".oid, 'pg_constraint')
                                                  AS description                -- 7

        FROM (SELECT pg_namespace.nspname                         AS schema,
                     unnest(pg_constraint.conkey)                 AS child, -- list of constrained columns
                     unnest(pg_constraint.confkey)                AS parent, -- list of referenced columns
                     pg_class.relname                             AS table_name,
                     pg_namespace.nspname                         AS schema_name,
                     generate_subscripts(pg_constraint.conkey, 1) AS conkey_idx,
                     pg_constraint.oid, -- Needed for obj_description
                     pg_constraint.confrelid,
                     pg_constraint.conrelid,
                     pg_constraint.conname
              FROM pg_class
                       JOIN pg_constraint ON pg_constraint.conrelid = pg_class.oid
                       JOIN pg_namespace ON pg_class.relnamespace = pg_namespace.oid
              WHERE pg_constraint.contype = 'f' -- f = foreign key constraint
              ORDER BY conkey_idx) "constraint"

        JOIN pg_attribute parent_attribute
          ON parent_attribute.attrelid = "constraint".confrelid
          AND parent_attribute.attnum = "constraint".parent
        JOIN pg_class parent_class
          ON parent_class.oid = "constraint".confrelid
        JOIN pg_attribute child_attribute
          ON child_attribute.attrelid = "constraint".conrelid
          AND child_attribute.attnum = "constraint".child
        -- Join namespace based on the referenced table's namespace OID.
        JOIN pg_namespace relation_namespace
          ON parent_class.relnamespace = relation_namespace.oid -- Use parent_class join for clarity

        WHERE "constraint".schema <> ALL ( $1 )

        -- order matters, be careful if changing
        ORDER BY constrained_schema, constrained_table_name, constraint_name, "constraint".conkey_idx;
    "#};

    let rows = sqlx::query(query)
        .bind(super::blocked_schemas())
        .fetch_all(conn)
        .await?;

    #[allow(clippy::manual_let_else)] // sorry, but match looks better here
    for row in rows {
        let constrained_schema_id = match database_definition.get_schema_id(row.get(1)) {
            Some(id) => id,
            None => continue,
        };

        let constrained_table_id = match database_definition.get_table_id(constrained_schema_id, row.get(2)) {
            Some(id) => id,
            None => continue,
        };

        let constrained_column_id = match database_definition.get_table_column_id(constrained_table_id, row.get(3)) {
            Some(id) => id,
            None => continue,
        };

        let referenced_schema_id = match database_definition.get_schema_id(row.get(4)) {
            Some(id) => id,
            None => continue,
        };

        let referenced_table_id = match database_definition.get_table_id(referenced_schema_id, row.get(5)) {
            Some(id) => id,
            None => continue,
        };

        let referenced_column_id = match database_definition.get_table_column_id(referenced_table_id, row.get(6)) {
            Some(id) => id,
            None => continue,
        };

        let foreign_key_id = match database_definition.get_foreign_key_id(constrained_schema_id, row.get(0)) {
            Some(id) => id,
            None => {
                let mut foreign_key = ForeignKey::new(
                    row.get(0),
                    constrained_schema_id,
                    constrained_table_id,
                    referenced_table_id,
                );

                if let Some(description) = row.get(7) {
                    foreign_key.set_description(description);
                }

                database_definition.push_foreign_key(foreign_key).0
            }
        };

        let column = ForeignKeyColumn::new(foreign_key_id, constrained_column_id, referenced_column_id);
        database_definition.push_foreign_key_column(column);
    }

    Ok(())
}

fn introspect_overrides(config: &Config, database_definition: &mut DatabaseDefinition) -> anyhow::Result<()> {
    for (schema, schema_config) in &config.schemas {
        let Some(constrained_schema_id) = database_definition.get_schema_id(schema) else {
            bail!("Could not find {schema} schema from the database. Check your configuration.")
        };

        for (view_name, view_config) in &schema_config.views {
            override_relations(
                database_definition,
                constrained_schema_id,
                view_name,
                &view_config.relations,
            )?;
        }

        for (table_name, table_config) in &schema_config.tables {
            override_relations(
                database_definition,
                constrained_schema_id,
                table_name,
                &table_config.relations,
            )?;
        }
    }

    Ok(())
}

fn override_relations(
    database_definition: &mut DatabaseDefinition,
    constrained_schema_id: SchemaId,
    table_name: &str,
    relations: &BTreeMap<String, RelationConfig>,
) -> anyhow::Result<()> {
    let Some(constrained_table_id) = database_definition.get_table_id(constrained_schema_id, table_name) else {
        bail!("Table `{table_name}` not found in relation configuration.")
    };

    for (relation, relation_config) in relations.iter() {
        let referenced_schema_id = match database_definition.get_schema_id(&relation_config.referenced_schema) {
            Some(id) => id,
            None => {
                bail!(
                    "Schema `{}` not found in relation configuration.",
                    relation_config.referenced_schema
                );
            }
        };

        let referenced_table_id =
            match database_definition.get_table_id(referenced_schema_id, &relation_config.referenced_table) {
                Some(id) => id,
                None => {
                    bail!(
                        "Table `{}` not found in relation configuration.",
                        relation_config.referenced_table
                    );
                }
            };

        let mut column_ids = Vec::new();

        for (constrained, referenced) in relation_config
            .referencing_columns
            .iter()
            .zip(&relation_config.referenced_columns)
        {
            let constrained_column_id = match database_definition.get_table_column_id(constrained_table_id, constrained)
            {
                Some(id) => id,
                None => {
                    bail!(
                        "Column `{}` not found in table `{}` in the relation configuration.",
                        constrained,
                        relation_config.referenced_table
                    );
                }
            };

            let referenced_column_id = match database_definition.get_table_column_id(referenced_table_id, referenced) {
                Some(id) => id,
                None => {
                    bail!(
                        "Column `{}` not found in table `{}` in the relation configuration.",
                        referenced,
                        relation_config.referenced_table
                    );
                }
            };

            column_ids.push((constrained_column_id, referenced_column_id));
        }

        let (foreign_key_id, _, _) = database_definition.push_foreign_key(ForeignKey::new(
            relation.clone(),
            constrained_schema_id,
            constrained_table_id,
            referenced_table_id,
        ));

        for (constrained_column_id, referenced_column_id) in column_ids {
            database_definition.push_foreign_key_column(ForeignKeyColumn::new(
                foreign_key_id,
                constrained_column_id,
                referenced_column_id,
            ));
        }
    }

    Ok(())
}
