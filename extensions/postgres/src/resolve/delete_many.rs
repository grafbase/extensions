use grafbase_database_definition::TableId;
use grafbase_sdk::{SdkError, types::Data};
use sql_ast::renderer;

use crate::context::Context;

use super::query;

pub(crate) fn execute(ctx: Context<'_>, table_id: TableId) -> Result<Data, SdkError> {
    let table = ctx.database_definition.walk(table_id);
    let ast = query::delete::build(&ctx, ctx.filter(table)?, table)?;
    let query = renderer::postgres::render(ast);

    tracing::debug!("Executing query: {}", query);

    let connection = ctx.pool.acquire()?;

    if ctx.mutation_is_returning() {
        let rows = query.fetch(&connection)?;
        let mut result = Vec::with_capacity(rows.size_hint().0);

        for mut row in rows {
            if let Some(col) = row.next() {
                result.push(col?.as_json::<serde_json::Value>()?);
            }
        }

        let row_count = result.len();

        let data = serde_json::to_vec(&serde_json::json!({
            "returning": result,
            "rowCount": row_count,
        }))
        .unwrap();

        Ok(Data::Json(data))
    } else {
        let row_count = query.execute(&connection)?;

        let data = serde_json::json!({
            "rowCount": row_count,
        });

        Ok(Data::Json(serde_json::to_vec(&data).unwrap()))
    }
}
