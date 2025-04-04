use grafbase_database_definition::{DatabaseDefinition, Enum, EnumVariant};
use indoc::indoc;
use sqlx::{PgConnection, Row};

pub(crate) async fn introspect_database(
    conn: &mut PgConnection,
    database_definition: &mut DatabaseDefinition,
) -> anyhow::Result<()> {
    let query = indoc! {r#"
        SELECT
          nsp.nspname AS schema_name,                             -- 0
          t.typname AS enum_name,                                 -- 1
          e.enumlabel AS enum_value,                             -- 2
          pg_catalog.obj_description(t.oid, 'pg_type') AS enum_comment -- 3
        FROM pg_catalog.pg_type t
        JOIN pg_catalog.pg_namespace nsp ON nsp.oid = t.typnamespace
        JOIN pg_catalog.pg_enum e ON t.oid = e.enumtypid
        WHERE nsp.nspname <> ALL ( $1 ) -- Exclude system schemas
          AND t.typtype = 'e' -- Ensure it is an enum type
        ORDER BY nsp.nspname, t.typname, e.enumsortorder;
    "#};

    let rows = sqlx::query(query)
        .bind(super::blocked_schemas())
        .fetch_all(conn)
        .await?;

    for row in rows {
        let Some(schema_id) = database_definition.get_schema_id(row.get(0)) else {
            continue;
        };

        let enum_name: String = row.get(1);

        let enum_id = match database_definition.get_enum_id(schema_id, &enum_name) {
            Some(enum_id) => enum_id,
            None => {
                let mut r#enum = Enum::new(schema_id, enum_name, None);

                if let Some(description) = row.get(3) {
                    r#enum.set_description(description);
                }

                database_definition.push_enum(r#enum)
            }
        };

        let variant = EnumVariant::new(enum_id, row.get(2), None);
        database_definition.push_enum_variant(variant);
    }

    Ok(())
}
