use crate::ast::{Column, Expression};

/// Defines ordering for an `ORDER BY` statement.
pub type OrderDefinition<'a> = (Expression<'a>, Option<Order>);

/// A list of definitions for the `ORDER BY` statement.
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Ordering<'a>(pub Vec<OrderDefinition<'a>>);

impl<'a> Ordering<'a> {
    pub fn append(&mut self, value: OrderDefinition<'a>) {
        self.0.push(value);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// The ordering direction
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Order {
    /// Ascending
    Asc,
    /// Descending
    Desc,
    /// Ascending Nulls First
    AscNullsFirst,
    /// Ascending Nulls Last
    AscNullsLast,
    /// Descending Nulls First
    DescNullsFirst,
    /// Descending Nulls Last
    DescNullsLast,
}

impl Order {
    /// Reverses the direction of the ordering.
    pub fn reverse(self) -> Self {
        match self {
            Order::Asc => Order::Desc,
            Order::Desc => Order::Asc,
            Order::AscNullsFirst => Order::DescNullsFirst,
            Order::AscNullsLast => Order::DescNullsLast,
            Order::DescNullsFirst => Order::AscNullsFirst,
            Order::DescNullsLast => Order::AscNullsLast,
        }
    }

    /// Returns `true` if the ordering direction is ascending.
    ///
    /// This function determines if an `Order` variant represents an ascending order,
    /// which includes `Asc`, `AscNullsFirst`, and `AscNullsLast`.
    pub fn ascends(self) -> bool {
        matches!(self, Order::Asc | Order::AscNullsFirst | Order::AscNullsLast)
    }

    /// Returns `true` if null values should be ordered first in the result.
    ///
    /// This function determines if an `Order` variant represents an ordering that
    /// places null values before non-null values, which includes `Desc`, `AscNullsFirst`,
    /// and `DescNullsFirst`.
    pub fn nulls_first(&self) -> bool {
        matches!(self, Order::Desc | Order::AscNullsFirst | Order::DescNullsFirst)
    }
}

/// An item that can be used in the `ORDER BY` statement
pub trait Orderable<'a>
where
    Self: Sized,
{
    /// Order by `self` in the given order
    fn order(self, order: Option<Order>) -> OrderDefinition<'a>;

    /// Change the order to `ASC`
    fn ascend(self) -> OrderDefinition<'a> {
        self.order(Some(Order::Asc))
    }

    /// Change the order to `DESC`
    fn descend(self) -> OrderDefinition<'a> {
        self.order(Some(Order::Desc))
    }

    /// Change the order to `ASC NULLS FIRST`
    fn ascend_nulls_first(self) -> OrderDefinition<'a> {
        self.order(Some(Order::AscNullsFirst))
    }

    /// Change the order to `ASC NULLS LAST`
    fn ascend_nulls_last(self) -> OrderDefinition<'a> {
        self.order(Some(Order::AscNullsLast))
    }

    /// Change the order to `DESC NULLS FIRST`
    fn descend_nulls_first(self) -> OrderDefinition<'a> {
        self.order(Some(Order::DescNullsFirst))
    }

    /// Change the order to `ASC NULLS LAST`
    fn descend_nulls_last(self) -> OrderDefinition<'a> {
        self.order(Some(Order::DescNullsLast))
    }
}

/// Convert the value into an order definition with order item and direction
pub trait IntoOrderDefinition<'a> {
    fn into_order_definition(self) -> OrderDefinition<'a>;
}

impl<'a> IntoOrderDefinition<'a> for &'a str {
    fn into_order_definition(self) -> OrderDefinition<'a> {
        let column: Column<'a> = self.into();
        (column.into(), None)
    }
}

impl<'a> IntoOrderDefinition<'a> for Column<'a> {
    fn into_order_definition(self) -> OrderDefinition<'a> {
        (self.into(), None)
    }
}

impl<'a> IntoOrderDefinition<'a> for OrderDefinition<'a> {
    fn into_order_definition(self) -> OrderDefinition<'a> {
        self
    }
}

impl<'a> Orderable<'a> for Column<'a> {
    fn order(self, order: Option<Order>) -> OrderDefinition<'a> {
        (self.into(), order)
    }
}

impl<'a> Orderable<'a> for &'a str {
    fn order(self, order: Option<Order>) -> OrderDefinition<'a> {
        let column: Column<'a> = self.into();
        column.order(order)
    }
}

impl<'a> Orderable<'a> for (&'a str, &'a str) {
    fn order(self, order: Option<Order>) -> OrderDefinition<'a> {
        let column: Column<'a> = self.into();
        column.order(order)
    }
}
