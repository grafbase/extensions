use grafbase_database_definition::TableWalker;
use grafbase_sdk::SdkError;
use sql_ast::ast::{
    Aliasable, Column, CommonTableExpression, Insert, MultiRowInsert, Query, Select, SingleRowInsert, default_value,
    json_build_object,
};

use crate::context::{
    Context,
    create_input::{CreateInputItem, CreateInputIterator},
    selection_iterator::TableSelection,
};

enum InsertType<'a> {
    Single(SingleRowInsert<'a>),
    Multi(MultiRowInsert<'a>),
}

pub fn build<'a>(
    ctx: &'a Context<'a>,
    table: TableWalker<'a>,
    input: impl IntoIterator<Item = CreateInputIterator<'a>>,
) -> Result<Query<'a>, SdkError> {
    let mut query = None;

    for input in input {
        match query.take() {
            None => {
                query = Some(InsertType::Single(create_insert(table, input)?));
            }
            Some(InsertType::Single(previous_insert)) => {
                let combined = previous_insert
                    .merge(create_insert(table, input)?)
                    .map_err(|error| SdkError::from(error.to_string()))?;

                query = Some(InsertType::Multi(combined));
            }
            Some(InsertType::Multi(mut previous_insert)) => {
                previous_insert
                    .extend(create_insert(table, input)?)
                    .map_err(|error| SdkError::from(error.to_string()))?;
            }
        }
    }

    let insert_name = format!("{}_{}_insert", table.schema(), table.database_name());

    let mut insert = match query.expect("we must have at least one input document") {
        InsertType::Single(insert) => insert.build(),
        InsertType::Multi(insert) => insert.build(),
    };

    if let Some(selection) = ctx.returning_selection(table)? {
        let mut select = Select::from_table(insert_name.clone());
        let mut returning = Vec::new();
        let mut selected_data = Vec::new();

        for selection in selection {
            match selection? {
                TableSelection::Column(column) => {
                    selected_data.push((
                        column.client_name(),
                        Column::from((insert_name.clone(), column.database_name())),
                    ));

                    returning.push(column.database_name());
                }
                // we will not have relations in the first phase
                TableSelection::JoinUnique(..) | TableSelection::JoinMany(..) => {
                    todo!("we'll get back to this with nested inserts")
                }
            }
        }

        insert.returning(returning);
        select.value(json_build_object(selected_data).alias("root"));
        select.with(CommonTableExpression::new(insert_name, insert));

        Ok(Query::from(select))
    } else {
        Ok(Query::from(insert))
    }
}

fn create_insert<'a>(table: TableWalker<'a>, input: CreateInputIterator<'a>) -> Result<SingleRowInsert<'a>, SdkError> {
    let mut insert = Insert::single_into(table.database_name());

    for input in input {
        match input? {
            CreateInputItem::Column(column, value) => insert.value(column.database_name(), value),
            CreateInputItem::DefaultValue(column) => insert.value(column.database_name(), default_value()),
        }
    }

    Ok(insert)
}
