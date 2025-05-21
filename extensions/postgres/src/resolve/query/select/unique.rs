use std::borrow::Cow;

use grafbase_sdk::SdkError;
use sql_ast::ast::{
    Aliasable, Column, Comparable, ConditionTree, Expression, Joinable, Select, Table, raw, row_to_json,
};

use crate::{context::selection_iterator::TableSelection, resolve::builder::SelectBuilder};

/// Builds the outermost query of the selection. Gathers all the data from the nested
/// queries into a JSON array, which is serialized in the database.
///
/// [example query](https://gist.github.com/pimeys/a7535acb0922fa432562539f5d8123c3)
pub fn build(builder: SelectBuilder<'_>) -> Result<Select<'_>, SdkError> {
    // The innermost query of the select. All filters, ordering, limits etc. are defined here.
    let sql_table =
        Table::from((builder.table().schema(), builder.table().database_name())).alias(builder.table().database_name());

    let mut inner_nested = Select::from_table(sql_table);

    if let Some(filters) = builder.filter() {
        for filter in filters {
            inner_nested.and_where(filter?);
        }
    }

    if let Some(relation) = builder.relation() {
        for (left, right) in relation.referencing_columns().zip(relation.referenced_columns()) {
            let left_column = Column::from((left.table().client_name(), left.database_name()));
            let right_column = Column::from((right.table().database_name(), right.database_name()));

            inner_nested.and_where(left_column.equals(right_column));
        }
    }

    // The middle query of the selection. Collects nested data from joins, and combines it with the main
    // query. Returns all rows as JSON objects.
    let mut collecting_select = Select::from_table(Table::from(inner_nested).alias(builder.table().client_name()));

    for selection in builder.selection() {
        match selection? {
            TableSelection::Column(select) => {
                let (column, expr, alias) = select.into_expression(Some(builder.table().client_name().into()));
                let alias = alias.unwrap_or_else(|| column.client_name());

                collecting_select.value(expr.alias(alias));
            }
            TableSelection::ColumnUnnest(unnest) => {
                let (column, nested, alias) = unnest.into_select(None);
                let alias = alias.unwrap_or_else(|| column.client_name());

                collecting_select.value(Expression::from(nested).alias(alias));
            }
            // m:1, 1:1
            TableSelection::JoinUnique(relation, selection, alias) => {
                let client_field_name = alias
                    .map(Cow::Borrowed)
                    .unwrap_or_else(|| relation.client_field_name().into());

                collecting_select.column(client_field_name.to_string());

                let mut builder =
                    SelectBuilder::new(relation.referenced_table(), selection, client_field_name.to_string());
                builder.set_relation(relation);

                // recurse
                let mut join_data = Table::from(build(builder)?)
                    .alias(client_field_name)
                    .on(ConditionTree::single(raw("true")));

                join_data.lateral();
                collecting_select.left_join(join_data);
            }
            // 1:m
            TableSelection::JoinMany(relation, selection, args, alias) => {
                let client_field_name = alias
                    .map(Cow::Borrowed)
                    .unwrap_or_else(|| relation.client_field_name().into());

                collecting_select.column(client_field_name.to_string());

                let mut builder =
                    SelectBuilder::new(relation.referenced_table(), selection, client_field_name.to_string());
                builder.set_relation(relation);

                // recurse
                let mut join_data = Table::from(super::pagination::build(builder, args)?)
                    .alias(client_field_name)
                    .on(ConditionTree::single(raw("true")));

                join_data.lateral();
                collecting_select.left_join(join_data);
            }
        }
    }

    let mut json_select = Select::from_table(Table::from(collecting_select).alias(builder.table().database_name()));
    json_select.value(row_to_json(builder.table().database_name(), false).alias(builder.field_name().to_string()));

    Ok(json_select)
}
