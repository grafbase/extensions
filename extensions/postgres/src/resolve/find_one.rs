use grafbase_database_definition::TableId;
use grafbase_sdk::{SdkError, host_io::logger::log, types::Data};
use sql_ast::renderer;

use crate::{
    context::Context,
    resolve::{builder::SelectBuilder, query},
};

fn null() -> Data {
    Data::Json(serde_json::to_vec(&serde_json::Value::Null).unwrap())
}

pub(crate) fn execute(ctx: Context<'_>, table_id: TableId) -> Result<Data, SdkError> {
    let table = ctx.database_definition.walk(table_id);

    let mut builder = SelectBuilder::new(table, ctx.selection(table)?, "root");

    if let Ok(filter) = ctx.unique_filter(table) {
        builder.set_filter(filter);
    }

    let ast = query::select::unique::build(builder)?;
    let query = renderer::postgres::render(ast);

    log::debug!(query = query.to_string(); "executing query");

    let connection = ctx.pool.acquire()?;
    let mut rows = query.fetch(&connection)?;

    let mut row = match rows.next() {
        Some(row) => row,
        None => return Ok(null()),
    };

    let col = match row.next() {
        Some(Ok(col)) => col,
        Some(Err(e)) => return Err(SdkError::from(format!("query error: {e}"))),
        None => return Ok(null()),
    };

    let data = col.into_bytes().map(Data::Json).unwrap_or_else(null);

    Ok(data)
}
