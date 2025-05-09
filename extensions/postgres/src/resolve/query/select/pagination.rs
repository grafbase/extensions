use std::collections::HashSet;

use crate::{
    context::{
        PageInfo,
        selection_iterator::{TableSelection, collection_args::CollectionArgs},
    },
    resolve::builder::SelectBuilder,
};
use grafbase_database_definition::TableColumnWalker;
use grafbase_sdk::{SdkError, host_io::postgres::types::DatabaseType};
use sql_ast::ast::{
    Aliasable, Column, CommonTableExpression, Comparable, ConditionTree, EncodeFormat, Expression, Function, Joinable,
    Order, OrderDefinition, Ordering, Select, Table, asterisk, cast, coalesce, convert_from, count, decode, encode,
    json_agg, json_build_array, json_build_object, json_extract_array_elem, raw, row_number, row_to_json,
};

/// The name of the Common Table Expression (CTE) that contains the decoded cursor.
const DECODED_CURSOR: &str = "DecodedCursor";

/// The name of the Common Table Expression (CTE) that contains extracted values from the cursor.
const CURSOR_VALUES: &str = "CursorValues";

/// The name of the Common Table Expression (CTE) that contains all filtered rows before pagination.
/// This CTE selects N+1 rows, where N is the limit.
const FILTERED: &str = "Filtered";

/// The name of the Common Table Expression (CTE) that contains the rows after pagination limits are applied.
/// This CTE selects N rows from the "Filtered" CTE, where N is the limit.
const NODES: &str = "Nodes";

/// The name of the Common Table Expression (CTE) that contains pagination metadata.
const PAGE_INFO: &str = "PageInfo";

/// Builds a pagination query, selecting multiple rows from the database, generating a cursor
/// for each row, and returning a page info object for pagination purposes.
///
/// This query is used if the GraphQL query expects data in edges/node, pageInfo types.
///
/// The query is divided into multiple common table expressions (CTE):
///
/// - "DecodedCursor": If the user sends a cursor in before/after argument, the CTE decodes the base64-encoded cursor.
/// - "CursorValues": If the user sends a cursor in before/after argument, the CTE extracts the values from the decoded cursor.
/// - "Filtered": The CTE selects all rows from the table that match the filter criteria. This query loads one extra row,
///   which is used to determine if there are more rows to load in pagination.
/// - "Nodes": The actual nodes in the result set. Loads N elements from "Filtered".
/// - "PageInfo": The CTE contains pagination metadata.
///
/// Every edge has a node, and a possible cursor. The cursor is base64-encoded JSON array, which holds the values
/// of each column that is included in the order by clause.
pub fn build<'a>(builder: SelectBuilder<'a>, args: CollectionArgs<'a>) -> Result<Select<'a>, SdkError> {
    let mut select = build_final_select(&builder, &args)?;

    if let Some(cursor) = args.cursor() {
        select.with(CommonTableExpression::new(
            DECODED_CURSOR,
            build_decoded_cursor_cte(cursor),
        ));

        select.with(CommonTableExpression::new(
            CURSOR_VALUES,
            build_cursor_values_cte(&args),
        ));
    }

    select.with(CommonTableExpression::new(
        FILTERED,
        build_filtered_cte(&builder, &args)?,
    ));

    select.with(CommonTableExpression::new(NODES, build_nodes_cte(&builder, &args)?));

    if let Some(page_info) = builder.page_info() {
        select.with(CommonTableExpression::new(
            PAGE_INFO,
            build_page_info_cte(&builder, &args, page_info),
        ));
    }

    Ok(select)
}

/// Builds a SQL selection that transforms a base64-encoded cursor string into a JSON value.
fn build_decoded_cursor_cte(cursor: &str) -> Select<'static> {
    let cursor = Expression::value(cursor.to_string().into_bound_value(0));
    let cursor = decode(cursor, EncodeFormat::Base64);
    let cursor = cast(convert_from(cursor, "UTF8"), "jsonb");

    let mut select = Select::default();
    select.value(Expression::from(cursor).alias("val"));

    select
}

/// Builds a SQL SELECT statement that extracts values from a decoded cursor.
///
/// This function creates a query that extracts each field from the cursor's JSON array,
/// casting each value to the appropriate database type based on the ordering columns.
/// The extracted values are named according to their corresponding database column names.
fn build_cursor_values_cte<'a>(args: &CollectionArgs<'a>) -> Select<'a> {
    let mut select = Select::from_table(DECODED_CURSOR);
    let cursor_column = Column::from((DECODED_CURSOR, "val"));

    for (i, (column, _)) in args.order_by().inner().enumerate() {
        let expr = json_extract_array_elem(cursor_column.clone(), i);
        let expr = cast(expr, column.database_type().as_str());

        select.value(expr.alias(column.database_name()));
    }

    select
}

/// Builds the outer selection query that forms the basis for pagination.
///
/// This function creates the first CTE (Common Table Expression) used in the pagination query,
/// which selects data from the main table and adds necessary metadata columns like row numbers
/// and cursor values for efficient pagination.
///
/// This selects one item extra over a possible limit, which we can use to inform the user if there's
/// more rows available.
fn build_filtered_cte<'a>(builder: &SelectBuilder<'a>, args: &CollectionArgs<'a>) -> Result<Select<'a>, SdkError> {
    let main_table_ref =
        Table::from((builder.table().schema(), builder.table().database_name())).alias(builder.table().client_name());

    let mut select = Select::from_table(main_table_ref);

    if args.cursor().is_some() {
        select.and_from(CURSOR_VALUES);
    }

    let mut cursor_payload_expressions: Vec<Expression<'_>> = Vec::new();

    attach_selection(builder, args, &mut select)?;
    attach_cursor_filters(args, &mut select);

    if let Some(relation) = builder.relation() {
        for (left, right) in relation.referencing_columns().zip(relation.referenced_columns()) {
            let left_column = Column::from((left.table().client_name(), left.database_name()));
            let right_column = Column::from((right.table().client_name(), right.database_name()));

            select.and_where(left_column.equals(right_column));
        }
    }

    let mut row_number = row_number();

    for (column, order) in args.order_by().inner() {
        let expr = Expression::from(Column::from((column.table().client_name(), column.database_name())));
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

    if builder.needs_cursor() {
        let generated_cursor_expr = encode(
            cast(cast(json_build_array(cursor_payload_expressions), "text"), "bytea"),
            EncodeFormat::Base64,
        )
        .alias("cursor");

        select.value(generated_cursor_expr);
    }

    select.value(Function::from(row_number).alias("row_number"));

    if let Some(filters) = builder.filter() {
        for filter_condition in filters {
            select.and_where(filter_condition?);
        }
    }

    Ok(select)
}

/// Builds a query that limits results from the outer selection based on pagination parameters.
///
/// This selection limits the +1 from the outer selection, and is then used in the inner selection
/// as the table to select from.
fn build_nodes_cte<'a>(builder: &SelectBuilder<'a>, args: &CollectionArgs<'a>) -> Result<Select<'a>, SdkError> {
    let table = Table::from(FILTERED).alias(builder.table().client_name());
    let mut select = Select::from_table(table);

    attach_selection(builder, args, &mut select)?;

    if builder.needs_cursor() {
        select.column(Column::from((builder.table().client_name(), "cursor")).alias("cursor"));
    }

    match (args.first(), args.last()) {
        (Some(limit), _) | (_, Some(limit)) => {
            let filter =
                Column::from("row_number").less_than_or_equals(Expression::value((limit as i32).into_bound_value(0)));

            select.and_where(filter);
        }
        _ => {}
    }

    for (column, order) in args.order_by().inner() {
        let column = Expression::from(Column::from((column.table().client_name(), column.database_name())));
        select.order_by((column, order));
    }

    Ok(select)
}

/// Builds a query that retrieves pagination metadata for the collection.
fn build_page_info_cte<'a>(builder: &SelectBuilder<'a>, args: &CollectionArgs<'a>, page_info: PageInfo) -> Select<'a> {
    let mut outer_select = Select::default();

    if page_info.selects_has_next_page() {
        outer_select.value({
            let expr = match args.first() {
                Some(limit) => {
                    let mut select = Select::from_table(FILTERED);
                    let expr = Expression::from(count(asterisk()))
                        .greater_than(Expression::value((limit as i32).into_bound_value(0)));

                    select.value(expr);
                    Expression::from(select)
                }
                None => raw("false"),
            };

            expr.alias("hasNextPage")
        });
    }

    if page_info.selects_has_previous_page() {
        outer_select.value({
            let expr = match args.last() {
                Some(limit) => {
                    let mut select = Select::from_table(FILTERED);
                    let expr = Expression::from(count(asterisk()))
                        .greater_than(Expression::value((limit as i32).into_bound_value(0)));

                    select.value(expr);
                    Expression::from(select)
                }
                None => raw("false"),
            };

            expr.alias("hasPreviousPage")
        });
    }

    if page_info.selects_start_cursor() {
        outer_select.value({
            let table = Table::from(NODES).alias(builder.table().client_name());
            let mut select = Select::from_table(table);

            select.limit(1);
            select.column("cursor");

            for (column, order) in args.order_by().inner() {
                let column = Expression::from(Column::from((column.table().client_name(), column.database_name())));
                let order = order.unwrap_or(Order::AscNullsFirst);

                select.order_by((column, Some(order)));
            }

            Expression::from(select).alias("startCursor")
        });
    }

    if page_info.selects_end_cursor() {
        outer_select.value({
            let table = Table::from(NODES).alias(builder.table().client_name());
            let mut select = Select::from_table(table);

            select.limit(1);
            select.column("cursor");

            for (column, order) in args.order_by().inner() {
                let column = Expression::from(Column::from((column.table().client_name(), column.database_name())));
                let order = order.map(|o| o.reverse()).unwrap_or(Order::DescNullsLast);

                select.order_by((column, Some(order)));
            }

            Expression::from(select).alias("endCursor")
        });
    }

    outer_select
}

/// Builds the final selection query that combines all CTEs into the complete pagination result.
fn build_final_select<'a>(builder: &SelectBuilder<'a>, args: &CollectionArgs<'a>) -> Result<Select<'a>, SdkError> {
    let mut json_aggregation = if builder.selects_edges() {
        let edges_select = build_edges_select(builder, args)?;
        Select::from_table(Table::from(edges_select).alias(builder.table().client_name()))
    } else {
        let mut select = Select::default();
        select.value(Expression::value(1.into_bound_value(0)));

        Select::from_table(select)
    };

    let mut outer_selected_objects = Vec::new();

    // SQL doesn't guarantee ordering if it's not defined in the query.
    // we'll reuse the nested ordering here.
    let mut ordering = Ordering::default();

    for order in args.order_by().outer() {
        ordering.append(order.clone());
    }

    if builder.selects_edges() {
        let mut inner_selected_objects = Vec::new();
        if builder.selects_cursor() {
            let cursor = Column::from((builder.table().client_name(), "cursor"));
            inner_selected_objects.push(("cursor", Expression::from(cursor)));
        }

        if builder.selects_nodes() {
            let column = Column::from((builder.table().client_name(), builder.field_name().to_string()));
            inner_selected_objects.push(("node", Expression::from(column)));
        }

        let json_obj = json_build_object(inner_selected_objects);
        let json_agg = json_agg(json_obj, Some(ordering), false);
        let json_coalesce = coalesce([Expression::from(json_agg), raw("'[]'")]);

        outer_selected_objects.push(("edges", Expression::from(json_coalesce)));
    }

    if let Some(page_info) = builder.page_info() {
        let mut select = Select::from_table(PAGE_INFO);

        let mut page_info_selection = Vec::new();

        if page_info.selects_has_next_page() {
            page_info_selection.push((
                "hasNextPage",
                Expression::from(Column::from((PAGE_INFO, "hasNextPage"))),
            ));
        }

        if page_info.selects_has_previous_page() {
            page_info_selection.push((
                "hasPreviousPage",
                Expression::from(Column::from((PAGE_INFO, "hasPreviousPage"))),
            ));
        }

        if page_info.selects_start_cursor() {
            page_info_selection.push((
                "startCursor",
                Expression::from(Column::from((PAGE_INFO, "startCursor"))),
            ));
        }

        if page_info.selects_end_cursor() {
            page_info_selection.push(("endCursor", Expression::from(Column::from((PAGE_INFO, "endCursor")))));
        }

        select.value(json_build_object(page_info_selection));
        outer_selected_objects.push(("pageInfo", Expression::from(select)));
    };

    let json_obj = json_build_object(outer_selected_objects);
    json_aggregation.value(json_obj.alias(builder.field_name().to_string()));

    Ok(json_aggregation)
}

/// Builds a SQL selection for the "edges" part of a GraphQL connection.
fn build_edges_select<'a>(builder: &SelectBuilder<'a>, args: &CollectionArgs<'a>) -> Result<Select<'a>, SdkError> {
    let sql_table = Table::from(NODES).alias(builder.table().client_name());
    let mut inner_nested = Select::from_table(sql_table);

    for (column, order) in args.order_by().inner() {
        let column = Expression::from(Column::from((column.table().client_name(), column.database_name())));
        inner_nested.order_by((column, order));
    }

    let mut collecting_select = Select::from_table(Table::from(inner_nested).alias(builder.table().client_name()));

    if builder.needs_cursor() {
        collecting_select.column((builder.table().client_name(), "cursor"));
    }

    let mut selected_columns = HashSet::new();

    for selection in builder.selection() {
        match selection? {
            TableSelection::Column(select) => {
                let (column, expr) = select.into_expression(Some(builder.table().client_name().into()));

                selected_columns.insert(column.id());
                collecting_select.value(expr.alias(column.client_name()));
            }
            TableSelection::ColumnUnnest(unnest) => {
                let (column, nested) = unnest.into_select(Some(NODES.into()));

                selected_columns.insert(column.id());
                collecting_select.value(Expression::from(nested).alias(column.client_name()));
            }
            // m:1, 1:1
            TableSelection::JoinUnique(relation, selection) => {
                let client_field_name = relation.client_field_name();
                collecting_select.column(client_field_name.clone());

                let mut builder = SelectBuilder::new(relation.referenced_table(), selection, client_field_name.clone());
                builder.set_relation(relation);

                // recurse
                let mut join_data = Table::from(super::unique::build(builder)?)
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
                builder.set_relation(relation);

                // recurse
                let mut join_data = Table::from(build(builder, args)?)
                    .alias(client_field_name)
                    .on(ConditionTree::single(raw("true")));

                join_data.lateral();
                collecting_select.left_join(join_data);
            }
        }
    }

    for (column, _) in args.extra_columns() {
        if !selected_columns.insert(column.id()) {
            continue;
        }

        let column = Column::from((builder.table().client_name(), column.database_name()));
        collecting_select.column(column);
    }

    let mut json_select = Select::from_table(Table::from(collecting_select).alias(builder.table().client_name()));
    json_select.value(row_to_json(builder.table().client_name(), false).alias(builder.field_name().to_string()));

    if builder.needs_cursor() {
        json_select.column((builder.table().client_name(), "cursor"));
    }

    for (_, column) in args.extra_columns() {
        json_select.column(column.table(builder.table().client_name()));
    }

    for ordering in args.order_by().outer() {
        json_select.order_by(ordering);
    }

    Ok(json_select)
}

/// Adds the selected columns to the SQL select statement based on the provided builder.
///
/// This function processes the selection items from the builder and adds them to the select statement.
/// It tracks already selected columns in a HashSet to avoid duplicates.
fn attach_selection<'a>(
    builder: &SelectBuilder<'a>,
    args: &CollectionArgs<'a>,
    select: &mut Select<'a>,
) -> Result<(), SdkError> {
    let mut selected_columns = HashSet::new();

    for selection_item in builder.selection() {
        match selection_item? {
            TableSelection::Column(col_select) => {
                let (column_meta, expr) = col_select.into_expression(Some(builder.table().client_name().into()));

                if selected_columns.insert(column_meta.id()) {
                    select.value(expr.alias(column_meta.database_name()));
                }
            }
            TableSelection::JoinUnique(relation, _) => {
                for column in relation.referencing_columns() {
                    if selected_columns.insert(column.id()) {
                        let column = Column::from((builder.table().client_name(), column.database_name()))
                            .alias(column.database_name());

                        select.column(column);
                    }
                }
            }
            _ => {}
        }
    }

    for (column, _) in args.extra_columns() {
        if selected_columns.insert(column.id()) {
            let column =
                Column::from((builder.table().client_name(), column.database_name())).alias(column.database_name());

            select.column(column);
        }
    }

    Ok(())
}

/// Attaches cursor-based filtering conditions to the SQL select statement.
///
/// This function adds WHERE conditions to implement cursor-based pagination,
/// handling both forward pagination with 'after' cursors and backward pagination
/// with 'before' cursors. The function applies the appropriate comparison operators
/// based on the cursor direction to ensure the correct rows are selected.
fn attach_cursor_filters<'a>(args: &CollectionArgs<'a>, select: &mut Select<'a>) {
    match (args.after(), args.before()) {
        (Some(_), None) => {
            let order_columns = args.order_by().inner().collect::<Vec<_>>();

            if let Some(condition) = build_cursor_conditions(&order_columns, true) {
                select.and_where(condition);
            }
        }
        (None, Some(_)) => {
            let order_columns = args.order_by().inner().collect::<Vec<_>>();

            if let Some(condition) = build_cursor_conditions(&order_columns, false) {
                select.and_where(condition);
            }
        }
        _ => {}
    }
}

/// Builds SQL conditions for cursor-based pagination based on ordering columns.
///
/// This function creates the SQL conditions needed to implement cursor-based pagination
/// by comparing table column values with cursor values. It handles both forward pagination
/// (with 'after' cursor) and backward pagination (with 'before' cursor).
///
/// The function constructs a complex condition tree that correctly handles:
/// - Multiple ordering columns with proper precedence
/// - Both ascending and descending order for each column
/// - NULL values in both table columns and cursor values
fn build_cursor_conditions<'a>(
    order_columns: &[(TableColumnWalker<'a>, Option<Order>)],
    is_after_cursor: bool,
) -> Option<ConditionTree<'a>> {
    if order_columns.is_empty() {
        return None;
    }

    let mut all_conditions = Vec::new();

    // Process each column in the ordering
    for i in 0..order_columns.len() {
        let (col, order_opt) = &order_columns[i];
        let order = if is_after_cursor {
            order_opt.unwrap_or(Order::AscNullsFirst)
        } else {
            order_opt.unwrap_or(Order::DescNullsLast)
        };

        // Get expressions for current column
        let col_expr = Expression::from(Column::from((col.table().client_name(), col.database_name())));
        let cursor_expr = Column::from((CURSOR_VALUES, col.database_name()));
        let is_cursor_null = Expression::from(cursor_expr.clone()).is_null();
        let is_col_null = col_expr.clone().is_null();

        // For the first column, or independent conditions for each column
        let primary_condition = if !order.ascends() {
            // For DESC order (e.g., when using 'before' cursor)
            ConditionTree::or(
                // (col < cursor_val AND cursor_val IS NOT NULL)
                ConditionTree::and(
                    ConditionTree::single(col_expr.clone().less_than(cursor_expr.clone())),
                    ConditionTree::not(is_cursor_null.clone()),
                ),
                // OR (cursor_val IS NULL)
                ConditionTree::single(is_cursor_null.clone()),
            )
        } else {
            // For ASC order
            ConditionTree::or(
                // (col > cursor_val AND cursor_val IS NOT NULL)
                ConditionTree::and(
                    ConditionTree::single(col_expr.clone().greater_than(cursor_expr.clone())),
                    ConditionTree::not(is_cursor_null.clone()),
                ),
                // OR (cursor_val IS NULL)
                ConditionTree::single(is_cursor_null.clone()),
            )
        };

        // If this is the first column, just add the primary condition
        if i == 0 {
            all_conditions.push(primary_condition);
            continue;
        }

        // For subsequent columns, we need to build conditions that:
        // 1. All previous columns are equal (using IS NOT DISTINCT FROM to handle NULLs)
        // 2. The current column has the comparison

        // First, build equality conditions for all previous columns
        let mut equality_conditions = Vec::new();

        (0..i).for_each(|j| {
            let (prev_col, _) = &order_columns[j];
            let prev_expr = Expression::from(Column::from((prev_col.table().client_name(), prev_col.database_name())));
            let prev_cursor = Column::from((CURSOR_VALUES, prev_col.database_name()));

            // Use IS NOT DISTINCT FROM to properly handle NULL equality
            let equality = ConditionTree::single(prev_expr.is_not_distinct_from(prev_cursor));
            equality_conditions.push(equality);
        });

        // For 2nd+ columns, add secondary condition for when current column values differ
        let comparative_condition = if !order.ascends() {
            // For DESC order of the current column
            ConditionTree::or(
                // (col < cursor_val AND col IS NOT NULL AND cursor_val IS NOT NULL)
                ConditionTree::and(
                    ConditionTree::single(col_expr.clone().less_than(cursor_expr.clone())),
                    ConditionTree::and(
                        ConditionTree::not(is_col_null.clone()),
                        ConditionTree::not(is_cursor_null.clone()),
                    ),
                ),
                // OR (col IS NOT NULL AND cursor_val IS NULL)
                ConditionTree::and(ConditionTree::not(is_col_null), is_cursor_null),
            )
        } else {
            // For ASC order of the current column
            ConditionTree::or(
                // (col > cursor_val AND col IS NOT NULL AND cursor_val IS NOT NULL)
                ConditionTree::and(
                    ConditionTree::single(col_expr.clone().greater_than(cursor_expr.clone())),
                    ConditionTree::and(
                        ConditionTree::not(is_col_null.clone()),
                        ConditionTree::not(is_cursor_null.clone()),
                    ),
                ),
                // OR (col IS NOT NULL AND cursor_val IS NULL)
                ConditionTree::and(ConditionTree::not(is_col_null), is_cursor_null),
            )
        };

        // Combine equality conditions with the comparative condition
        let mut combined = comparative_condition;
        for eq_condition in equality_conditions {
            combined = ConditionTree::and(eq_condition, combined);
        }

        all_conditions.push(combined);
    }

    // Combine all conditions with OR
    if all_conditions.is_empty() {
        return None;
    }

    let mut final_condition = all_conditions.pop().unwrap();
    while let Some(condition) = all_conditions.pop() {
        final_condition = ConditionTree::or(condition, final_condition);
    }

    Some(final_condition)
}
