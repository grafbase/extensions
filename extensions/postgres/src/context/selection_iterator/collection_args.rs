use std::collections::BTreeMap;

use grafbase_database_definition::{DatabaseDefinition, TableColumnWalker, TableWalker};
use grafbase_sdk::SdkError;

use sql_ast::ast::{Aliasable, Column, Order, OrderDefinition};

#[derive(Clone, Default)]
pub struct CollectionOrdering<'a> {
    inner: Vec<(TableColumnWalker<'a>, Option<Order>)>,
    outer: Vec<(String, Option<Order>)>,
}

impl<'a> CollectionOrdering<'a> {
    pub fn inner(&self) -> impl ExactSizeIterator<Item = (TableColumnWalker<'a>, Option<Order>)> + '_ {
        self.inner.iter().map(|(column, order)| (*column, *order))
    }

    pub fn outer(&self) -> impl ExactSizeIterator<Item = OrderDefinition<'static>> + '_ {
        self.outer.iter().map(|(column, order)| {
            let column = Column::from(column.clone());
            (column.into(), *order)
        })
    }
}

/// Argument defining a relay-style GraphQL collection.
#[derive(Clone)]
pub struct CollectionArgs<'a> {
    first: Option<u64>,
    last: Option<u64>,
    before: Option<String>,
    after: Option<String>,
    order_by: CollectionOrdering<'a>,
    extra_columns: Vec<(TableColumnWalker<'a>, Column<'static>)>,
}

#[derive(Default, Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionParameters {
    pub first: Option<u64>,
    pub last: Option<u64>,
    pub before: Option<String>,
    pub after: Option<String>,
    #[serde(default)]
    pub order_by: Vec<OrderParameter>,
}

#[derive(Debug, Default, Clone, serde::Deserialize)]
pub struct OrderParameter {
    #[serde(flatten)]
    pub field: BTreeMap<String, OrderDirection>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderDirection {
    Asc,
    Desc,
}

impl<'a> CollectionArgs<'a> {
    pub(crate) fn new(
        database_definition: &'a DatabaseDefinition,
        table: TableWalker<'a>,
        mut params: CollectionParameters,
    ) -> Result<Self, SdkError> {
        if let (Some(_), Some(_)) = (params.first, params.last) {
            return Err(SdkError::from("first and last parameters can't be both defined"));
        }

        let constraint = table
            .implicit_ordering_key()
            .expect("tables at this point must have at least one unique constraint");

        for column in constraint.columns() {
            if params
                .order_by
                .iter()
                .any(|v| v.field.contains_key(column.table_column().client_name()))
            {
                continue;
            }

            params.order_by.push({
                let mut map = BTreeMap::new();
                map.insert(column.table_column().client_name().to_string(), OrderDirection::Asc);

                OrderParameter { field: map }
            });
        }

        // ordering the innermost query
        let mut order_by = CollectionOrdering::default();

        // extra columns we have to select (based on ordering)
        let mut extra_columns = Vec::new();

        for mut value in params.order_by {
            let Some((field, direction)) = value.field.pop_first() else {
                continue;
            };

            // For `last` to work, we must reverse the order of the inner query.
            let inner_direction = match direction {
                OrderDirection::Desc if params.last.is_some() => Order::AscNullsFirst,
                OrderDirection::Desc => Order::DescNullsFirst,
                _ if params.last.is_some() => Order::DescNullsFirst,
                _ => Order::AscNullsFirst,
            };

            // and then reverse the order again for the outer query.
            let outer_direction = match inner_direction {
                Order::DescNullsFirst if params.last.is_some() => Order::AscNullsFirst,
                Order::AscNullsFirst if params.last.is_some() => Order::DescNullsFirst,
                _ => inner_direction,
            };

            let column = database_definition
                .find_column_for_client_field(&field, table.id())
                .ok_or_else(|| {
                    SdkError::from(format!(
                        "ordering type {} with field{}, which does not exist",
                        table.client_name(),
                        &field
                    ))
                })?;

            let sql_column = Column::from((table.database_name().to_string(), column.client_name().to_string()));

            // We must name our order columns for them to be visible in the order by statement of the
            // outer queries.
            let alias = format!("{}_{}", table.database_name(), column.database_name());
            extra_columns.push((column, sql_column.clone().alias(alias.clone())));

            order_by.inner.push((column, Some(inner_direction)));
            order_by.outer.push((alias, Some(outer_direction)));
        }

        Ok(Self {
            first: params.first,
            last: params.last,
            before: params.before,
            after: params.after,
            order_by,
            extra_columns,
        })
    }

    /// Select the first N items. An example GraphQL definition: `userCollection(first: N)`.
    pub(crate) fn first(&self) -> Option<u64> {
        self.first
    }

    /// Select the last N items. An example GraphQL definition: `userCollection(last: N)`.
    pub(crate) fn last(&self) -> Option<u64> {
        self.last
    }

    /// Returns the cursor value for fetching the page before the specified cursor.
    ///
    /// This corresponds to the `before` parameter in GraphQL pagination that is used
    /// to fetch a page of items that come before a specific cursor value.
    pub(crate) fn before(&self) -> Option<&str> {
        self.before.as_deref()
    }

    /// Returns the cursor value for fetching the page after the specified cursor.
    ///
    /// This corresponds to the `after` parameter in GraphQL pagination that is used
    /// to fetch a page of items that come after a specific cursor value.
    pub(crate) fn after(&self) -> Option<&str> {
        self.after.as_deref()
    }

    /// Returns the cursor value, prioritizing "before" over "after" if both are present.
    ///
    /// This is a convenience method that returns either the "before" cursor or the "after" cursor,
    /// whichever is defined. If both are defined, "before" is returned.
    pub(crate) fn cursor(&self) -> Option<&str> {
        self.before().or_else(|| self.after())
    }

    /// Defines the ordering of the collection. The first item in a tuple is the ordering for the innermost
    /// query, and the second one of all the outer queries. An example GraphQL definition:
    /// `userCollection(orderBy: [{ name: DESC }])`.
    pub(crate) fn order_by(&self) -> &CollectionOrdering<'a> {
        &self.order_by
    }

    /// A set of extra columns needing to select in the collecting query. Needed to handle the ordering of the outer
    /// layers.
    pub(crate) fn extra_columns(&self) -> impl ExactSizeIterator<Item = (TableColumnWalker<'a>, Column<'static>)> + '_ {
        self.extra_columns.clone().into_iter()
    }
}
