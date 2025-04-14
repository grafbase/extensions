use grafbase_database_definition::TableWalker;
use grafbase_sdk::SdkError;
use sql_ast::ast::{Aliasable, Column, CommonTableExpression, ConditionTree, Query, Select, Update, json_build_object};

use crate::context::{Context, filter::FilterIterator, selection_iterator::TableSelection};

pub fn build<'a>(
    ctx: &'a Context<'a>,
    table: TableWalker<'a>,
    mut filter: FilterIterator<'a>,
) -> Result<Query<'a>, SdkError> {
    let mut update = Update::table(table.database_name());

    let condition = filter.try_fold(ConditionTree::NoCondition, |acc, filter| {
        Result::<_, SdkError>::Ok(ConditionTree::and(acc, filter?))
    })?;

    update.so_that(condition);

    for item in ctx.update_input(table)? {
        let item = item?;
        update.set(item.column.database_name(), item.expression);
    }

    if let Some(selection) = ctx.returning_selection(table)? {
        let update_name = format!("{}_{}_update", table.schema(), table.database_name());

        let mut returning = Vec::new();
        let mut selected_data = Vec::new();

        for selection in selection {
            match selection? {
                TableSelection::Column(column) => {
                    selected_data.push((
                        column.database_name(),
                        Column::from((update_name.clone(), column.database_name())),
                    ));

                    returning.push(column.database_name());
                }
                // we will not have relations in the first phase
                TableSelection::JoinUnique(..) | TableSelection::JoinMany(..) => {
                    todo!("we'll get back to this with nested updates")
                }
            }
        }

        update.returning(returning);

        let mut select = Select::from_table(update_name.clone());
        select.with(CommonTableExpression::new(update_name, update));
        select.value(json_build_object(selected_data).alias("root"));

        Ok(Query::from(select))
    } else {
        Ok(Query::from(update))
    }
}
