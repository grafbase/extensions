use grafbase_database_definition::DatabaseDefinition;
use sqlx::{PgConnection, Row};
use std::str::FromStr;

pub(crate) async fn introspect_database(
    conn: &mut PgConnection,
    database_definition: &mut DatabaseDefinition,
) -> anyhow::Result<()> {
    use grafbase_database_definition::{ColumnType, EnumType, IdentityGeneration, ScalarType, TableColumn};

    let query = indoc::indoc! {r#"
        SELECT columns.table_schema                         AS schema,
               columns.table_name                           AS table_name,
               columns.column_name                          AS column_name,
               CAST(columns.udt_name::regtype::oid AS int4) AS type_id,
               columns.udt_name                             AS type_name,
               columns.udt_schema                           AS type_schema,
               columns.data_type = 'ARRAY'                  AS is_array,
               pg_attrdef.adbin IS NOT NULL                 AS has_default,
               columns.is_nullable = 'YES'                  AS is_nullable,
               columns.identity_generation                  AS identity_generation,
               pg_description.description                   AS description

        FROM information_schema.columns columns

                 -- for default values
                 JOIN pg_attribute ON pg_attribute.attname = columns.column_name

                 -- also for defaults
                 JOIN (SELECT pg_class.oid,
                              relname,
                              pg_namespace.nspname AS namespace
                       FROM pg_class
                                JOIN pg_namespace ON pg_namespace.oid = pg_class.relnamespace) AS pg_class
                      ON pg_class.oid = pg_attribute.attrelid
                          AND pg_class.relname = columns.table_name
                          AND pg_class.namespace = columns.table_schema

                 -- also for defaults
                 LEFT OUTER JOIN pg_attrdef
                                 ON pg_attrdef.adrelid = pg_attribute.attrelid
                                     AND pg_attrdef.adnum = pg_attribute.attnum
                                     AND pg_class.namespace = columns.table_schema

                 -- for column comments
                 LEFT OUTER JOIN pg_description
                                 ON pg_description.objoid = pg_class.oid
                                     AND pg_description.objsubid = pg_attribute.attnum

        WHERE table_schema <> ALL ( $1 )
        ORDER BY schema, table_name, columns.ordinal_position;
    "#};

    let rows = sqlx::query(query)
        .bind(super::blocked_schemas())
        .fetch_all(conn)
        .await?;

    for row in rows {
        let Some(schema_id) = database_definition.get_schema_id(row.get(0)) else {
            continue;
        };
        let Some(table_id) = database_definition.get_table_id(schema_id, row.get(1)) else {
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

        let mut column = TableColumn::new(table_id, database_type, row.get(2), None);

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

    Ok(())
}
