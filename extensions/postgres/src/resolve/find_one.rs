use grafbase_database_definition::TableId;
use grafbase_sdk::SdkError;
use sql_ast::renderer;

use crate::{
    context::Context,
    resolve::{builder::SelectBuilder, query},
};

pub(crate) fn execute(ctx: Context<'_>, table_id: TableId) -> Result<serde_json::Value, SdkError> {
    let table = ctx.database_definition.walk(table_id);

    let mut builder = SelectBuilder::new(table, ctx.selection(table)?, "root");

    if let Ok(filter) = ctx.unique_filter(table) {
        builder.set_filter(filter);
    }

    let ast = query::select::build(builder)?;
    let query = renderer::postgres::render(ast);

    println!("{query}"); // TODO: tracing

    let connection = ctx.pool.acquire()?;
    let mut rows = query.fetch(&connection)?;

    let mut row = match rows.next() {
        Some(row) => row,
        None => return Ok(serde_json::Value::Null),
    };

    let col = match row.next() {
        Some(Ok(col)) => col,
        Some(Err(e)) => return Err(SdkError::from(format!("query error: {e}"))),
        None => return Ok(serde_json::Value::Null),
    };

    let data = col.as_json()?.unwrap_or(serde_json::Value::Null);

    Ok(data)
}
