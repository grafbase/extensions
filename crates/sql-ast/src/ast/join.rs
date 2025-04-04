use crate::ast::{ConditionTree, Table};

/// The `JOIN` table and conditions.
#[derive(Debug, PartialEq, Clone)]
pub struct JoinData<'a> {
    pub(crate) table: Table<'a>,
    pub(crate) conditions: ConditionTree<'a>,
    pub(crate) lateral: bool,
}

impl<'a> JoinData<'a> {
    /// Implement a join with no conditions.
    pub fn all_from(table: impl Into<Table<'a>>) -> Self {
        Self {
            table: table.into(),
            conditions: ConditionTree::NoCondition,
            lateral: false,
        }
    }

    /// Join as lateral join.
    pub fn lateral(&mut self) {
        self.lateral = true;
    }
}

impl<'a, T> From<T> for JoinData<'a>
where
    T: Into<Table<'a>>,
{
    fn from(table: T) -> Self {
        Self::all_from(table)
    }
}

/// A representation of a `JOIN` statement.
#[derive(Debug, PartialEq, Clone)]
pub enum Join<'a> {
    /// Implements an `INNER JOIN` with given `JoinData`.
    Inner(JoinData<'a>),
    /// Implements an `LEFT JOIN` with given `JoinData`.
    Left(JoinData<'a>),
    /// Implements an `RIGHT JOIN` with given `JoinData`.
    Right(JoinData<'a>),
    /// Implements an `FULL JOIN` with given `JoinData`.
    Full(JoinData<'a>),
    /// Implements a `CROSS JOIN` with given `JoinData`.
    Cross(JoinData<'a>),
}

/// An item that can be joined.
pub trait Joinable<'a> {
    /// Add the `JOIN` conditions.
    fn on<T>(self, conditions: T) -> JoinData<'a>
    where
        T: Into<ConditionTree<'a>>;
}

impl<'a, U> Joinable<'a> for U
where
    U: Into<Table<'a>>,
{
    fn on<T>(self, conditions: T) -> JoinData<'a>
    where
        T: Into<ConditionTree<'a>>,
    {
        JoinData {
            table: self.into(),
            conditions: conditions.into(),
            lateral: false,
        }
    }
}

impl<'a> Joinable<'a> for JoinData<'a> {
    fn on<T>(self, conditions: T) -> JoinData<'a>
    where
        T: Into<ConditionTree<'a>>,
    {
        let conditions = match self.conditions {
            ConditionTree::NoCondition => conditions.into(),
            cond => cond.and(conditions.into()),
        };

        JoinData {
            table: self.table,
            conditions,
            lateral: false,
        }
    }
}
