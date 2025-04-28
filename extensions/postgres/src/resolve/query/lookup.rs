use enumflags2::BitFlags;
use grafbase_database_definition::{RelationWalker, TableColumnWalker};
use grafbase_sdk::{SdkError, host_io::postgres::types::DatabaseValue};
use indexmap::{IndexMap, map::Entry};
use serde_json::Value;
use sql_ast::ast::{
    Alias, Aliasable, Case, Column, Comparable, ConditionTree, Expression, Joinable, Orderable, Ordering, Select,
    Table, cast, coalesce, json_agg, raw, raw_str, row_to_json, unnest,
};

use crate::{
    context::{
        order::OrderKind,
        selection_iterator::{SelectionIterator, TableSelection, collection_args::CollectionArgs},
    },
    resolve::builder::SelectBuilder,
};

pub fn build(builder: SelectBuilder<'_>) -> Result<Select<'_>, SdkError> {
    let lookup_order = builder
        .lookup_order()
        .ok_or_else(|| SdkError::from("Missing lookup IDs for query generation"))?;

    if lookup_order.is_empty() {
        let mut empty_select = Select::default();
        empty_select.value(sql_ast::ast::cast(raw_str("[]"), "JSON").alias(builder.field_name().to_string()));
        return Ok(empty_select);
    }

    let mut input_order_alias = Alias::new("input_order");
    let mut column_values: IndexMap<TableColumnWalker<'_>, Vec<Value>> = IndexMap::new();
    let mut lookup_columns = Vec::new();

    for order in lookup_order {
        match order {
            OrderKind::Single(column, value) => match column_values.entry(column) {
                Entry::Occupied(mut occupied_entry) => occupied_entry.get_mut().push(value),
                Entry::Vacant(vacant_entry) => {
                    lookup_columns.push(column);
                    input_order_alias.push_column(column.database_name());
                    vacant_entry.insert(vec![value]);
                }
            },
            OrderKind::Composite(items) => {
                for (column, value) in items {
                    match column_values.entry(column) {
                        Entry::Occupied(mut occupied_entry) => occupied_entry.get_mut().push(value),
                        Entry::Vacant(vacant_entry) => {
                            lookup_columns.push(column);
                            input_order_alias.push_column(column.database_name());
                            vacant_entry.insert(vec![value]);
                        }
                    }
                }
            }
        }
    }

    input_order_alias.push_column("ord");

    let mut unnest_values = Vec::new();

    for (column, values) in column_values {
        unnest_values.push(DatabaseValue::from_json_input(
            Value::Array(values),
            column.database_type(),
            true,
        )?);
    }

    let unnest = unnest(Expression::many_value(unnest_values), true).alias(input_order_alias);

    let main_table_ref =
        Table::from((builder.table().schema(), builder.table().database_name())).alias(builder.table().client_name());

    // LEFT JOIN MainTable ON input_order.id = MainTable.pk
    let order_join = lookup_columns.iter().fold(ConditionTree::NoCondition, |acc, col| {
        let order_column = Column::from(("input_order", col.database_name()));
        let table_column = Column::from((builder.table().client_name(), col.database_name()));

        acc.and(order_column.equals(table_column))
    });

    // --- Step 3: Base Select and LATERAL Joins ---
    // Start the select from UNNEST, join the main data table
    let mut main_select = Select::from_table(Expression::from(unnest));
    main_select.left_join(main_table_ref.on(order_join));

    // Keep track of columns/expressions needed for the final JSON object `q`.
    // Stores (JSON_key_name, SQL_expression_for_value)
    let mut columns_for_json = Vec::new();

    for selection in builder.selection() {
        match selection? {
            // Base columns from the main table (e.g., User.id, User.name)
            TableSelection::Column(select) => {
                let (column, _) = select.into_expression(None); // We only need column names here
                // Reference the column from the aliased main table (e.g., "User"."name")
                let col_expr = Column::from((builder.table().client_name(), column.database_name()));
                columns_for_json.push((column.client_name().to_string(), col_expr.into()));
            }
            // Nested object/array fetched via LATERAL JOIN (e.g., blogs, author)
            TableSelection::JoinUnique(relation, nested_selection) => {
                inject_relation(
                    &mut main_select,
                    &mut columns_for_json,
                    relation,
                    nested_selection,
                    None,
                )?;
            }
            TableSelection::JoinMany(relation, nested_selection, args) => {
                inject_relation(
                    &mut main_select,
                    &mut columns_for_json,
                    relation,
                    nested_selection,
                    Some(args),
                )?;
            }
            TableSelection::ColumnUnnest(unnest) => {
                // Handle array unnesting if required in the final object structure.
                // This might involve adding a subquery result similar to lateral joins.
                let (column, nested_select) = unnest.into_select(None);
                columns_for_json.push((column.client_name().to_string(), Expression::from(nested_select)));
            }
        }
    }

    // --- Step 4: Build CASE expression for result_json ---

    // Build the subquery for `ROW_TO_JSON(subquery)`
    let mut sub_select = Select::default();
    if columns_for_json.is_empty() {
        // If no columns were selected, ROW_TO_JSON would fail or produce unintended results.
        // Handle this case, maybe by selecting a dummy value or adjusting logic.
        // For now, let's select 'null' to represent an empty object perhaps.
        sub_select.value(cast(raw_str("null"), "JSON")); // Adjust as needed
    } else {
        for (alias, expr) in columns_for_json {
            sub_select.value(expr.alias(alias));
        }
    }

    // `(SELECT row_to_json(q)::jsonb FROM (SELECT ...) q)`
    let row_to_json_expr = cast(row_to_json("subquery", false), "JSONB");

    let mut json_builder_select = Select::from_table(Table::from(sub_select).alias("subquery"));
    json_builder_select.value(row_to_json_expr);

    // `CASE WHEN MainTable.pk IS NOT NULL THEN (json_builder_select) ELSE 'null'::jsonb END AS result_json`
    let case_expr = lookup_columns.iter().fold(ConditionTree::NoCondition, |acc, col| {
        let expr = Column::from((builder.table().client_name(), col.database_name())).is_not_null();
        acc.and(expr)
    });

    let case_expr = Case::builder()
        .when(case_expr, json_builder_select)
        .r#else(cast(raw("null"), "JSONB"));

    // Add the CASE expression and the ordinality column to the main select's values
    let ord_col = Column::from(("input_order", "ord")); // "input_order"."ord"
    main_select.value(Expression::from(case_expr).alias("result_json"));
    main_select.value(ord_col.clone()); // Select ord for final ordering

    // --- Step 5: Final Aggregation ---
    // `SELECT COALESCE(jsonb_agg(result_json ORDER BY ord), '[]'::jsonb) AS "root"`
    let final_data_table = Table::from(main_select).alias("final_data");
    let mut final_select = Select::from_table(final_data_table);

    let result_json_col = Column::from(("final_data", "result_json"));
    let final_ord_col = Column::from(("final_data", "ord")); // Get 'ord' from the final_data alias

    // `jsonb_agg(result_json ORDER BY ord)`
    // Ensure `sql_ast::ast::functions::jsonb_agg` exists and returns an `Aggregated` struct
    // supporting `.order_by(Orderable)`.
    let jsonb_agg_expr = json_agg(result_json_col, Some(Ordering(vec![final_ord_col.order(None)])), false);

    // `COALESCE(..., '[]'::jsonb)`
    let coalesce_expr = coalesce([Expression::from(jsonb_agg_expr), cast(raw_str("[]"), "JSON").into()]);

    final_select.value(coalesce_expr.alias(builder.field_name().to_string()));

    Ok(final_select)
}

fn inject_relation<'a>(
    main_select: &mut Select<'a>,
    columns_for_json: &mut Vec<(String, Expression<'a>)>,
    relation: RelationWalker<'a>,
    nested_selection: SelectionIterator<'a>,
    args: Option<CollectionArgs>,
) -> Result<(), SdkError> {
    let client_field_name = relation.client_field_name();
    let join_alias = client_field_name;

    let mut join_builder = SelectBuilder::new(relation.referenced_table(), nested_selection, join_alias.clone());
    join_builder.set_relation(relation);

    if let Some(args) = args {
        join_builder.set_collection_args(args);
    }

    let nested_select = super::select::build(join_builder, BitFlags::all())?;

    let mut join_data = Table::from(nested_select)
        .alias(join_alias.clone())
        .on(ConditionTree::single(raw("true")));

    join_data.lateral();

    main_select.left_join(join_data);

    columns_for_json.push((
        join_alias.to_string(), // Alias in the final JSON object (e.g., "blogs")
        Column::from((join_alias.clone(), join_alias)).into(), // Column from lateral join result
    ));

    Ok(())
}
