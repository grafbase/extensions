use grafbase_database_definition::TableId;
use grafbase_sdk::{SdkError, types::Data};
use sql_ast::renderer;

use crate::context::{
    Context,
    selection_iterator::collection_args::{CollectionArgs, CollectionParameters},
};

use super::{builder::SelectBuilder, query};

fn empty() -> Data {
    Data::Json(serde_json::to_vec(&serde_json::Value::Array(Vec::new())).unwrap())
}

pub(crate) fn execute(ctx: Context<'_>, table_id: TableId) -> Result<Data, SdkError> {
    let table = ctx.database_definition.walk(table_id);
    let mut builder = SelectBuilder::new(table, ctx.collection_selection(table)?, "root");
    let collection_params = ctx.field.arguments::<CollectionParameters>(ctx.arguments)?;

    let args = CollectionArgs::new(ctx.database_definition, table, collection_params)?;
    builder.set_collection_args(args);

    if let Ok(filter) = ctx.filter(table) {
        builder.set_filter(filter);
    }

    let ast = query::select::build(builder, false)?;
    let query = renderer::postgres::render(ast);

    tracing::debug!("Executing query: {}", query);

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
