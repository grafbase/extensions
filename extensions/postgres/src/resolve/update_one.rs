use grafbase_database_definition::TableId;
use grafbase_sdk::{SdkError, host_io::logger::log, types::Data};
use sql_ast::renderer;

use crate::context::Context;

use super::query;

pub(crate) fn execute(ctx: Context<'_>, table_id: TableId) -> Result<Data, SdkError> {
    let table = ctx.database_definition.walk(table_id);
    let ast = query::update::build(&ctx, table, ctx.unique_filter(table)?)?;
    let query = renderer::postgres::render(ast);

    log::debug!(query = query.to_string(); "executing query");

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

        let data = Data::Json(serde_json::to_vec(&data).unwrap());

        Ok(data)
    } else {
        let row_count = query.execute(&connection)?;

        let data = serde_json::json!({
            "rowCount": row_count,
        });

        let data = Data::Json(serde_json::to_vec(&data).unwrap());

        Ok(data)
    }
}
