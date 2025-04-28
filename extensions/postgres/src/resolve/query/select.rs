use enumflags2::{BitFlags, bitflags};
use grafbase_sdk::SdkError;
use sql_ast::ast::{
    Aliasable, Column, Comparable, ConditionTree, Expression, Joinable, Ordering, Select, Table, coalesce, json_agg,
    json_build_object, raw, raw_str, row_to_json,
};

use crate::{context::selection_iterator::TableSelection, resolve::builder::SelectBuilder};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[bitflags]
#[repr(u8)]
pub enum SelectFlag {
    Nested = 0b0001,
    Pagination = 0b0010,
}

/// Builds the outermost query of the selection. Gathers all the data from the nested
/// queries into a JSON array, which is serialized in the database.
///
/// [example query](https://gist.github.com/pimeys/a7535acb0922fa432562539f5d8123c3)
pub fn build(builder: SelectBuilder<'_>, flags: BitFlags<SelectFlag>) -> Result<Select<'_>, SdkError> {
    // The innermost query of the select. All filters, ordering, limits etc. are defined here.
    let sql_table =
        Table::from((builder.table().schema(), builder.table().database_name())).alias(builder.table().database_name());

    let mut inner_nested = Select::from_table(sql_table);

    if let Some(filters) = builder.filter() {
        for filter in filters {
            inner_nested.and_where(filter?);
        }
    }

    if let Some(args) = builder.collection_args() {
        // only order if we can paginate. the outer query of lookup
        // cannot paginate, so we return in the order of the client IDs
        if flags.contains(SelectFlag::Pagination) {
            for ordering in args.order_by().inner() {
                inner_nested.order_by(ordering.clone());
            }
        }

        if let Some(limit) = args.first() {
            inner_nested.limit(limit as u32); // we load one extra for pagination
        }

        // There's no `LAST` in PostgreSQL, so we limit the inner selection which is ordered in an opposite way,
        // and re-order it in the outer query.
        if let Some(limit) = args.last() {
            inner_nested.limit(limit as u32); // we load one extra for pagination
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
                let (column, expr) = select.into_expression(Some(builder.table().client_name().into()));
                collecting_select.value(expr.alias(column.client_name()));
            }
            TableSelection::ColumnUnnest(unnest) => {
                let (column, nested) = unnest.into_select(None);
                collecting_select.value(Expression::from(nested).alias(column.client_name()));
            }
            // m:1, 1:1
            TableSelection::JoinUnique(relation, selection) => {
                let client_field_name = relation.client_field_name();
                collecting_select.column(client_field_name.clone());

                let mut builder = SelectBuilder::new(relation.referenced_table(), selection, client_field_name.clone());
                builder.set_relation(relation);

                // recurse
                let flags = flags | SelectFlag::Nested | SelectFlag::Pagination;
                let mut join_data = Table::from(build(builder, flags)?)
                    .alias(client_field_name)
                    .on(ConditionTree::single(raw("true")));

                join_data.lateral();
                collecting_select.left_join(join_data);
            }
            // 1:m
            TableSelection::JoinMany(relation, selection, args) => {
                let client_field_name = relation.client_field_name();
                collecting_select.column(client_field_name.clone());

                let mut builder = SelectBuilder::new(relation.referenced_table(), selection, client_field_name.clone());
                builder.set_collection_args(args);
                builder.set_relation(relation);

                // recurse
                let flags = flags | SelectFlag::Nested | SelectFlag::Pagination;
                let mut join_data = Table::from(build(builder, flags)?)
                    .alias(client_field_name)
                    .on(ConditionTree::single(raw("true")));

                join_data.lateral();
                collecting_select.left_join(join_data);
            }
        }
    }

    let mut json_select = Select::from_table(Table::from(collecting_select).alias(builder.table().database_name()));
    json_select.value(row_to_json(builder.table().database_name(), false).alias(builder.field_name().to_string()));

    if flags.contains(SelectFlag::Nested) {
        json_select.value(raw_str("todo").alias("cursor"));
        json_select.value(raw_str("todo").alias("start_cursor"));
        json_select.value(raw_str("todo").alias("end_cursor"));
    }

    match builder.collection_args() {
        Some(args) => {
            for column in args.extra_columns() {
                json_select.column(column);
            }

            // SQL doesn't guarantee ordering if it's not defined in the query.
            // we'll reuse the nested ordering here.
            //
            // only order if we can paginate. the outer query of lookup
            // cannot paginate, so we return in the order of the client IDs
            if flags.contains(SelectFlag::Pagination) {
                for ordering in args.order_by().outer() {
                    json_select.order_by(ordering);
                }
            }

            let mut json_aggregation =
                Select::from_table(Table::from(json_select).alias(builder.table().database_name().to_string()));

            let column = Column::from((builder.table().database_name(), builder.field_name().to_string()));

            if flags.contains(SelectFlag::Pagination) {
                // SQL doesn't guarantee ordering if it's not defined in the query.
                // we'll reuse the nested ordering here.
                let mut ordering = Ordering::default();

                for order in args.order_by().outer() {
                    ordering.append(order.clone());
                }

                let json_obj = json_build_object([("node", Expression::from(column)), ("cursor", raw_str("todo"))]);

                let json_agg = json_agg(json_obj, Some(ordering), false);
                let json_coalesce = coalesce([Expression::from(json_agg), raw("'[]'")]);

                let page_info = json_build_object([
                    ("hasNextPage", raw("false")),
                    ("hasNextPage", raw("false")),
                    ("hasPreviousPage", raw("false")),
                    ("startCursor", raw_str("todo")),
                    ("endCursor", raw_str("todo")),
                ]);

                let json_obj = json_build_object([
                    ("edges", Expression::from(json_coalesce)),
                    ("pageInfo", Expression::from(page_info)),
                ]);

                json_aggregation.value(json_obj.alias(builder.field_name().to_string()));

                Ok(json_aggregation)
            } else {
                let json_agg = json_agg(Expression::from(column), None, false);
                let json_coalesce = coalesce([Expression::from(json_agg), raw("'[]'")]);

                json_aggregation.value(json_coalesce.alias(builder.field_name().to_string()));

                Ok(json_aggregation)
            }
        }
        None => Ok(json_select),
    }
}
