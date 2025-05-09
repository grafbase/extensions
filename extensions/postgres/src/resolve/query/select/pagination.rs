use enumflags2::BitFlags;
use grafbase_sdk::{
    SdkError,
    host_io::postgres::types::{DatabaseType, DatabaseValue},
};
use sql_ast::ast::{
    Alias, Aliasable, Column, Comparable, ConditionTree, EncodeFormat, Expression, Function, Joinable, Order,
    OrderDefinition, Orderable, Ordering, Select, Table, asterisk, cast, coalesce, count, encode, json_agg,
    json_build_array, json_build_object, raw, raw_str, row_number, row_to_json,
};

// Assuming these imports are available and provide necessary functionality
use crate::{
    context::selection_iterator::{
        TableSelection,
        collection_args::{CollectionArgs, CollectionOrdering},
    },
    resolve::{builder::SelectBuilder, query::select::SelectFlag},
};

pub fn build(builder: SelectBuilder<'_>, args: CollectionArgs) -> Result<Select<'_>, SdkError> {
    let selected_cte_alias = format!("Selected_{}", builder.table().client_name());
    let page_cte_alias = format!("Page_{}", builder.table().client_name());
    let page_info_cte_alias = format!("PageInfo_{}", builder.table().client_name());

    let selected_cte_select = build_outer_selection(&builder, &args);
    let limited_cte_select = build_limit_selection(&builder, &args, &selected_cte_alias);
    let page_info_select = build_page_info_selection(&args, &selected_cte_alias, &page_cte_alias);

    todo!()
}

/// Builds the outer selection query that forms the basis for pagination.
///
/// This function creates the first CTE (Common Table Expression) used in the pagination query,
/// which selects data from the main table and adds necessary metadata columns like row numbers
/// and cursor values for efficient pagination.
///
/// This selects one item extra over a possible limit, which we can use to inform the user if there's
/// more rows available.
///
/// This query also selects from the inner selection
fn build_outer_selection<'a>(builder: &SelectBuilder<'a>, args: &CollectionArgs) -> Select<'a> {
    let main_table_ref =
        Table::from((builder.table().schema(), builder.table().database_name())).alias(builder.table().client_name());

    let mut select = Select::from_table(main_table_ref);
    let mut cursor_payload_expressions: Vec<Expression<'_>> = Vec::new();

    for selection_item in builder.selection() {
        if let TableSelection::Column(col_select) = selection_item? {
            let (column_meta, expr) = col_select.into_expression(Some(builder.table().client_name().into()));
            select.value(expr.alias(column_meta.client_name()));
        }
    }

    let mut row_number = row_number();

    for (expr, order) in args.order_by().inner() {
        cursor_payload_expressions.push(expr.clone());

        let order_definition = OrderDefinition::from((expr, order));
        select.order_by(order_definition.clone());

        row_number = row_number.order_by(order_definition);
    }

    match (args.first(), args.last()) {
        (Some(limit), _) | (_, Some(limit)) => {
            select.limit(limit as u32 + 1);
        }
        _ => {}
    }

    let generated_cursor_expr = encode(
        cast(cast(json_build_array(cursor_payload_expressions), "text"), "bytea"),
        EncodeFormat::Base64,
    )
    .alias("cursor");

    select.value(generated_cursor_expr);
    select.value(Function::from(row_number).alias("row_number"));

    if let Some(filters) = builder.filter() {
        for filter_condition in filters {
            select.and_where(filter_condition?);
        }
    }

    select
}

/// Builds a query that limits results from the outer selection based on pagination parameters.
///
/// This selection limits the +1 from the outer selection, and is then used in the inner selection
/// as the table to select from.
fn build_limit_selection<'a>(
    builder: &SelectBuilder<'a>,
    args: &CollectionArgs,
    outer_selection_alias: &'a str,
) -> Select<'a> {
    let mut select = Select::from_table(outer_selection_alias);

    for selection_item in builder.selection() {
        if let TableSelection::Column(col_select) = selection_item? {
            let (column_meta, expr) = col_select.into_expression(Some(builder.table().client_name().into()));
            select.value(expr.alias(column_meta.client_name()));
        }
    }

    select.column("cursor");

    match (args.first(), args.last()) {
        (Some(limit), _) | (_, Some(limit)) => {
            let filter =
                Column::from("row_number").less_than_or_equals(Expression::value((limit as i32).into_bound_value(0)));

            select.and_where(filter);
        }
        _ => {}
    }

    for order in args.order_by().outer() {
        select.order_by(order);
    }

    select
}

/// Builds a query that retrieves pagination metadata for the collection.
///
/// This function creates a SELECT query that computes pagination information:
/// - `hasNextPage`: Determines if there are more results after the current page
/// - `hasPreviousPage`: Determines if there are results before the current page
/// - `startCursor`: The cursor of the first item in the current page
/// - `endCursor`: The cursor of the last item in the current page
///
/// # Arguments
///
/// * `args` - The collection arguments containing pagination parameters and ordering
/// * `limit` - The maximum number of items to return in a page
/// * `outer_selection_alias` - The alias of the CTE containing all candidate rows with cursors
/// * `limit_selection_alias` - The alias of the CTE containing the rows for the current page
///
/// # Returns
///
/// A SELECT query that computes pagination metadata for the collection.
fn build_page_info_selection<'a>(
    args: &CollectionArgs,
    outer_selection_alias: &'a str,
    limit_selection_alias: &'a str,
) -> Select<'a> {
    let mut outer_select = Select::default();

    outer_select.value({
        let expr = match args.first() {
            Some(limit) => {
                let mut select = Select::from_table(outer_selection_alias);
                let expr = Expression::from(count(asterisk()))
                    .greater_than(Expression::value((limit as i32).into_bound_value(0)));

                select.value(expr);
                Expression::from(select)
            }
            None => raw("false"),
        };

        expr.alias("hasNextPage")
    });

    outer_select.value({
        let expr = match args.last() {
            Some(limit) => {
                let mut select = Select::from_table(outer_selection_alias);
                let expr = Expression::from(count(asterisk()))
                    .greater_than(Expression::value((limit as i32).into_bound_value(0)));

                select.value(expr);
                Expression::from(select)
            }
            None => raw("false"),
        };

        expr.alias("hasPreviousPage")
    });

    outer_select.value({
        let mut select = Select::from_table(limit_selection_alias);

        select.limit(1);
        select.column("cursor");

        for order in args.order_by().outer() {
            select.order_by(order);
        }

        Expression::from(select).alias("startCursor")
    });

    outer_select.value({
        let mut select = Select::from_table(limit_selection_alias);

        select.limit(1);
        select.column("cursor");

        for (column, order) in args.order_by().outer() {
            let order = order.map(|o| o.reverse()).unwrap_or(Order::DescNullsLast);
            select.order_by((column, Some(order)));
        }

        Expression::from(select).alias("endCursor")
    });

    outer_select
}

fn build_inner_select<'a>(
    builder: &SelectBuilder<'a>,
    args: &CollectionArgs,
    limit_selection_alias: &'a str,
    page_info_alias: &'a str,
) -> Result<Select<'a>, SdkError> {
    // The innermost query of the select. All filters, ordering, limits etc. are defined here.
    let sql_table =
        Table::from((builder.table().schema(), builder.table().database_name())).alias(builder.table().database_name());

    let mut inner_nested = Select::from_table(sql_table);

    if let Some(filters) = builder.filter() {
        for filter in filters {
            inner_nested.and_where(filter?);
        }
    }

    for ordering in args.order_by().inner() {
        inner_nested.order_by(ordering.clone());
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

    for column in args.extra_columns() {
        json_select.column(column);
    }

    // SQL doesn't guarantee ordering if it's not defined in the query.
    // we'll reuse the nested ordering here.
    //
    // only order if we can paginate. the outer query of lookup
    // cannot paginate, so we return in the order of the client IDs
    for ordering in args.order_by().outer() {
        json_select.order_by(ordering);
    }

    let mut json_aggregation =
        Select::from_table(Table::from(json_select).alias(builder.table().database_name().to_string()));

    let column = Column::from((builder.table().database_name(), builder.field_name().to_string()));
    let cursor = Column::from((limit_selection_alias, "cursor"));

    // SQL doesn't guarantee ordering if it's not defined in the query.
    // we'll reuse the nested ordering here.
    let mut ordering = Ordering::default();

    for order in args.order_by().outer() {
        ordering.append(order.clone());
    }

    let json_obj = json_build_object([("node", Expression::from(column)), ("cursor", Expression::from(cursor))]);

    let json_agg = json_agg(json_obj, Some(ordering), false);
    let json_coalesce = coalesce([Expression::from(json_agg), raw("'[]'")]);

    let page_info = {
        let mut select = Select::from_table(page_info_alias);

        let has_next_page = Expression::from(Column::from((page_info_alias, "hasNextPage")));
        let has_previous_page = Expression::from(Column::from((page_info_alias, "hasPreviousPage")));
        let start_cursor = Expression::from(Column::from((page_info_alias, "startCursor")));
        let end_cursor = Expression::from(Column::from((page_info_alias, "endCursor")));

        select.value(json_build_object([
            ("hasNextPage", Expression::from(coalesce([has_next_page, raw("false")]))),
            (
                "hasPreviousPage",
                Expression::from(coalesce([has_previous_page, raw("false")])),
            ),
            ("startCursor", start_cursor),
            ("endCursor", end_cursor),
        ]));

        Expression::from(select)
    };

    let json_obj = json_build_object([
        ("edges", Expression::from(json_coalesce)),
        ("pageInfo", Expression::from(page_info)),
    ]);

    json_aggregation.value(json_obj.alias(builder.field_name().to_string()));

    Ok(json_aggregation)
}
