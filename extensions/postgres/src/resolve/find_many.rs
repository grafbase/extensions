use grafbase_database_definition::TableId;
use grafbase_sdk::SdkError;
use sql_ast::renderer;

use crate::context::{
    Context,
    selection_iterator::collection_args::{CollectionArgs, CollectionParameters},
};

use super::{builder::SelectBuilder, query};

pub(crate) fn execute(ctx: Context<'_>, table_id: TableId) -> Result<serde_json::Value, SdkError> {
    let table = ctx.database_definition.walk(table_id);
    let mut builder = SelectBuilder::new(table, ctx.collection_selection(table)?, "root");
    let collection_params = ctx.field.arguments::<CollectionParameters>(ctx.arguments)?;

    let args = CollectionArgs::new(ctx.database_definition, table, collection_params)?;
    builder.set_collection_args(args);

    if let Ok(filter) = ctx.filter(table) {
        builder.set_filter(filter);
    }

    let ast = query::select::build(builder)?;
    let query = renderer::postgres::render(ast);
    let connection = ctx.pool.acquire()?;
    let mut rows = query.fetch(&connection)?;

    let mut row = match rows.next() {
        Some(row) => row,
        None => return Ok(serde_json::Value::Array(Vec::new())),
    };

    let col = match row.next() {
        Some(Ok(col)) => col,
        Some(Err(e)) => return Err(SdkError::from(format!("query error: {e}"))),
        None => return Ok(serde_json::Value::Array(Vec::new())),
    };

    let data = col.as_json()?.unwrap_or(serde_json::Value::Array(Vec::new()));

    Ok(data)
}
