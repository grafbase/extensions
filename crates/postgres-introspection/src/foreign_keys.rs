use grafbase_database_definition::{DatabaseDefinition, ForeignKey, ForeignKeyColumn};
use sqlx::{PgConnection, Row};

pub(crate) async fn introspect_database(
    conn: &mut PgConnection,
    database_definition: &mut DatabaseDefinition,
) -> anyhow::Result<()> {
    let query = indoc::indoc! {r#"
        SELECT "constraint".conname       AS constraint_name,
               "constraint".schema        AS constrained_schema,
               "constraint".table_name    AS constrained_table_name,
               child_attribute.attname    AS constrained_column_name,
               relation_namespace.nspname AS referenced_schema,
               parent_class.relname       AS referenced_table_name,
               parent_attribute.attname   AS referenced_column_name

        FROM (SELECT pg_namespace.nspname                         AS schema,
                     unnest(pg_constraint.conkey)                 AS child, -- list of constrained columns
                     unnest(pg_constraint.confkey)                AS parent, -- list of referenced columns
                     pg_class.relname                             AS table_name,
                     pg_namespace.nspname                         AS schema_name,
                     generate_subscripts(pg_constraint.conkey, 1) AS conkey_idx,
                     pg_constraint.oid,
                     pg_constraint.confrelid,
                     pg_constraint.conrelid,
                     pg_constraint.conname
              FROM pg_class
                       JOIN pg_constraint ON pg_constraint.conrelid = pg_class.oid
                       JOIN pg_namespace ON pg_class.relnamespace = pg_namespace.oid
              WHERE pg_constraint.contype = 'f' -- f = foreign key
              ORDER BY conkey_idx) "constraint"

        JOIN pg_attribute parent_attribute
          ON parent_attribute.attrelid = "constraint".confrelid
          AND parent_attribute.attnum = "constraint".parent
        JOIN pg_class parent_class
          ON parent_class.oid = "constraint".confrelid
        JOIN pg_attribute child_attribute
          ON child_attribute.attrelid = "constraint".conrelid
          AND child_attribute.attnum = "constraint".child
        JOIN pg_class child_class
          ON "constraint".confrelid = child_class.oid
        JOIN pg_namespace relation_namespace
          ON child_class.relnamespace = relation_namespace.oid

        WHERE "constraint".conname <> ALL ( $1 )

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
                let foreign_key = ForeignKey::new(
                    row.get(0),
                    constrained_schema_id,
                    constrained_table_id,
                    referenced_table_id,
                );

                database_definition.push_foreign_key(foreign_key).0
            }
        };

        let column = ForeignKeyColumn::new(foreign_key_id, constrained_column_id, referenced_column_id);
        database_definition.push_foreign_key_column(column);
    }

    Ok(())
}
