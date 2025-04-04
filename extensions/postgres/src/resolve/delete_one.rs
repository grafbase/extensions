use super::query;
use crate::context::Context;
use grafbase_database_definition::TableId;
use grafbase_sdk::{SdkError, types::Data};
use sql_ast::renderer;

pub(crate) fn execute(ctx: Context<'_>, table_id: TableId) -> Result<Data, SdkError> {
    let table = ctx.database_definition.walk(table_id);
    let ast = query::delete::build(&ctx, ctx.unique_filter(table)?, table)?;
    let query = renderer::postgres::render(ast);

    tracing::debug!("Executing query: {}", query);

    let connection = ctx.pool.acquire()?;

    if ctx.mutation_is_returning() {
        let mut rows = query.fetch(&connection)?;

        let result = match rows.next() {
            Some(mut row) => match row.next() {
                Some(Ok(col)) => col.as_json()?.unwrap_or(serde_json::Value::Null),
                Some(Err(err)) => return Err(SdkError::from(format!("query error: {err}"))),
                None => serde_json::Value::Null,
            },
            None => serde_json::Value::Null,
        };

        let row_count = if result.is_null() { 0 } else { 1 };

        let data = serde_json::json!({
            "returning": result,
            "rowCount": row_count,
        });

        Ok(Data::Json(serde_json::to_vec(&data).unwrap()))
    } else {
        let row_count = query.execute(&connection)?;

        let data = serde_json::json!({
            "rowCount": row_count,
        });

        Ok(Data::Json(serde_json::to_vec(&data).unwrap()))
    }
}
