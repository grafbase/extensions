use grafbase_database_definition::TableId;
use grafbase_sdk::SdkError;
use sql_ast::renderer;

use crate::context::Context;

use super::query;

pub(crate) fn execute(ctx: Context<'_>, table_id: TableId) -> Result<serde_json::Value, SdkError> {
    let table = ctx.database_definition.walk(table_id);
    let ast = query::insert::build(&ctx, table, [ctx.create_input(table)?])?;
    let query = renderer::postgres::render(ast);
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

        Ok(serde_json::json!({
            "returning": result,
            "rowCount": row_count,
        }))
    } else {
        let row_count = query.execute(&connection)?;

        Ok(serde_json::json!({
            "rowCount": row_count,
        }))
    }
}
