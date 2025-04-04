use grafbase_database_definition::{DatabaseDefinition, Enum, EnumVariant};
use indoc::indoc;
use sqlx::{PgConnection, Row};

pub(crate) async fn introspect_database(
    conn: &mut PgConnection,
    database_definition: &mut DatabaseDefinition,
) -> anyhow::Result<()> {
    let query = indoc! {r#"
        SELECT
          pg_namespace.nspname AS schema,
          pg_type.typname      AS enum_name,
          pg_enum.enumlabel    AS enum_value
        FROM pg_type
        JOIN pg_enum ON pg_type.oid = pg_enum.enumtypid
        JOIN pg_namespace ON pg_namespace.oid = pg_type.typnamespace
        WHERE pg_namespace.nspname <> ALL ( $1 )
        ORDER BY pg_namespace.nspname, pg_type.typname, pg_enum.enumsortorder;
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
        let enum_value: String = row.get(2);

        let enum_id = match database_definition.get_enum_id(schema_id, &enum_name) {
            Some(enum_id) => enum_id,
            None => database_definition.push_enum(Enum::new(schema_id, enum_name, None)),
        };

        database_definition.push_enum_variant(EnumVariant::new(enum_id, enum_value, None));
    }

    Ok(())
}
