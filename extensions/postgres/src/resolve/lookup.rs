use grafbase_sdk::{SdkError, host_io::logger::log, types::Data};
use sql_ast::renderer;

use crate::resolve::{builder::SelectBuilder, query};

fn empty() -> Data {
    Data::Json(serde_json::to_vec(&serde_json::Value::Array(Vec::new())).unwrap())
}

pub(crate) fn execute(
    ctx: crate::context::Context<'_>,
    table_id: grafbase_database_definition::TableId,
) -> Result<Data, SdkError> {
    let table = ctx.database_definition.walk(table_id);
    let mut builder = SelectBuilder::new(table, ctx.selection(table)?, "root");

    if let Some(lookup_order) = ctx.lookup_order(table)? {
        builder.set_lookup_order(lookup_order);
    }

    let ast = query::lookup::build(builder)?;
    let query = renderer::postgres::render(ast);

    log::debug!(query = query.to_string(); "executing query");

    let connection = ctx.pool.acquire()?;
    let mut rows = query.fetch(&connection)?;

    let mut row = match rows.next() {
        Some(row) => row,
        None => return Ok(empty()),
    };

    let col = match row.next() {
        Some(Ok(col)) => col,
        Some(Err(e)) => return Err(SdkError::from(format!("query error: {e}"))),
        None => return Ok(empty()),
    };

    let data = col.into_bytes().map(Data::Json).unwrap_or_else(empty);

    Ok(data)
}
