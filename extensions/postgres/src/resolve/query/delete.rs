use grafbase_database_definition::TableWalker;
use grafbase_sdk::SdkError;
use sql_ast::ast::{Aliasable, ConditionTree, Delete, Expression, Table, json_build_object};

use crate::context::{Context, filter::FilterIterator, selection_iterator::TableSelection};

pub fn build<'a>(
    ctx: &'a Context<'a>,
    filter: FilterIterator<'a>,
    table: TableWalker<'a>,
) -> Result<Delete<'a>, SdkError> {
    let sql_table = Table::from((table.schema(), table.database_name())).alias(table.database_name());

    let mut query = Delete::from_table(sql_table);
    let mut conditions = Vec::new();

    for condition in filter {
        conditions.push(Expression::from(condition?));
    }

    let condition = if conditions.is_empty() {
        ConditionTree::NoCondition
    } else {
        ConditionTree::And(conditions)
    };

    query.so_that(condition);

    if let Some(selection) = ctx.returning_selection(table)? {
        let mut returning = Vec::new();

        for selection in selection {
            match selection? {
                TableSelection::Column(select) => {
                    let (column, expr, alias) = select.into_expression(None);
                    let alias = alias.unwrap_or_else(|| column.client_name());

                    returning.push((alias, expr));
                }
                TableSelection::ColumnUnnest(unnest) => {
                    let (column, nested, alias) = unnest.into_select(None);
                    let alias = alias.unwrap_or_else(|| column.client_name());

                    returning.push((alias, Expression::from(nested)));
                }
                // our output type doesn't have relations, so this is never reachable
                TableSelection::JoinMany(..) | TableSelection::JoinUnique(..) => {
                    unreachable!("we cannot join in a delete statement")
                }
            }
        }

        query.returning([json_build_object(returning).alias("root")]);
    }

    Ok(query)
}
