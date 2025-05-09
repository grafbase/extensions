use grafbase_sdk::host_io::postgres::types::DatabaseValue;

use crate::ast::{Aliasable, Column, Comparable, Compare, ConditionTree, Function, Row, Select, SqlOp, Table, Values};

use super::{
    Alias, Case,
    compare::{JsonCompare, JsonType},
};
use std::borrow::Cow;

/// An expression that can be positioned in a query. Can be a single value or a
/// statement that is evaluated into a value.
#[derive(Debug, Clone, PartialEq)]
pub struct Expression<'a> {
    pub kind: ExpressionKind<'a>,
    pub alias: Option<Alias<'a>>,
}

impl<'a> Expression<'a> {
    pub fn value(value: DatabaseValue) -> Self {
        Self {
            kind: ExpressionKind::Parameterized(ParameterizedValue { value, enum_type: None }),
            alias: None,
        }
    }

    pub fn many_value(values: Vec<DatabaseValue>) -> Self {
        Self {
            kind: ExpressionKind::ManyParameterized(
                values
                    .into_iter()
                    .map(|value| ParameterizedValue { value, enum_type: None })
                    .collect(),
            ),
            alias: None,
        }
    }

    pub fn enum_value(value: DatabaseValue, enum_type: impl Into<Cow<'a, str>>) -> Self {
        Self {
            kind: ExpressionKind::Parameterized(ParameterizedValue {
                value,
                enum_type: Some(enum_type.into()),
            }),
            alias: None,
        }
    }
}

impl<'a> Expression<'a> {
    /// The type of the expression, dictates how it's implemented in the query.
    pub fn kind(&self) -> &ExpressionKind<'a> {
        &self.kind
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParameterizedValue<'a> {
    pub value: DatabaseValue,
    pub enum_type: Option<Cow<'a, str>>,
}

/// An expression we can compare and use in database queries.
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionKind<'a> {
    /// Anything that we must parameterize before querying
    Parameterized(ParameterizedValue<'a>),
    /// Multiple values that we must parameterize before querying.
    ManyParameterized(Vec<ParameterizedValue<'a>>),
    /// Will be rendered as-is to the SQL statement. Carefully escape, if needed.
    Raw(&'a str),
    /// Will be rendered as-is, quoted, to the SQL statement. Carefully escape, if needed.
    RawString(&'a str),
    /// A database column
    Column(Box<Column<'a>>),
    /// A database column
    Table(Box<Table<'a>>),
    /// Data in a row form, e.g. (1, 2, 3)
    Row(Row<'a>),
    /// A nested `SELECT` or `SELECT .. UNION` statement
    Selection(Box<Select<'a>>),
    /// A database function call
    Function(Box<Function<'a>>),
    /// A qualified asterisk to a table
    Asterisk(Option<Box<Table<'a>>>),
    /// An operation: sum, sub, mul or div.
    Op(Box<SqlOp<'a>>),
    /// A tree of expressions to evaluate from the deepest value to up
    ConditionTree(ConditionTree<'a>),
    /// A comparison expression
    Compare(Compare<'a>),
    /// A single value, column, row or a nested select
    Value(Box<Expression<'a>>),
    /// Multiple values
    Values(Values<'a>),
    /// A case expression
    Case(Case<'a>),
    /// DEFAULT keyword, e.g. for `INSERT INTO ... VALUES (..., DEFAULT, ...)`
    Default,
}

/// A quick alias to create a raw value expression.
pub fn raw(value: &str) -> Expression<'_> {
    Expression {
        kind: ExpressionKind::Raw(value),
        alias: None,
    }
}

pub fn raw_str(value: &str) -> Expression<'_> {
    Expression {
        kind: ExpressionKind::RawString(value),
        alias: None,
    }
}

/// A quick alias to create an asterisk to a table.
pub fn asterisk() -> Expression<'static> {
    Expression {
        kind: ExpressionKind::Asterisk(None),
        alias: None,
    }
}

/// A quick alias to create a default value expression.
pub fn default_value() -> Expression<'static> {
    Expression {
        kind: ExpressionKind::Default,
        alias: None,
    }
}

impl<'a> From<Function<'a>> for Expression<'a> {
    fn from(f: Function<'a>) -> Self {
        Expression {
            kind: ExpressionKind::Function(Box::new(f)),
            alias: None,
        }
    }
}

impl<'a> From<SqlOp<'a>> for Expression<'a> {
    fn from(p: SqlOp<'a>) -> Self {
        Expression {
            kind: ExpressionKind::Op(Box::new(p)),
            alias: None,
        }
    }
}

impl<'a> From<Values<'a>> for Expression<'a> {
    fn from(value: Values<'a>) -> Self {
        Expression {
            kind: ExpressionKind::Values(value),
            alias: None,
        }
    }
}

impl<'a> From<Row<'a>> for Expression<'a> {
    fn from(value: Row<'a>) -> Self {
        Expression {
            kind: ExpressionKind::Row(value),
            alias: None,
        }
    }
}

impl<'a> From<Table<'a>> for Expression<'a> {
    fn from(value: Table<'a>) -> Self {
        Self {
            kind: ExpressionKind::Table(Box::new(value)),
            alias: None,
        }
    }
}

impl<'a> From<ExpressionKind<'a>> for Expression<'a> {
    fn from(kind: ExpressionKind<'a>) -> Self {
        Self { kind, alias: None }
    }
}

impl<'a> From<Case<'a>> for Expression<'a> {
    fn from(value: Case<'a>) -> Self {
        Self {
            kind: ExpressionKind::Case(value),
            alias: None,
        }
    }
}

impl<'a> Aliasable<'a> for Expression<'a> {
    type Target = Expression<'a>;

    fn alias<T>(mut self, alias: T) -> Self::Target
    where
        T: Into<Alias<'a>>,
    {
        self.alias = Some(alias.into());
        self
    }
}

impl<'a> Comparable<'a> for Expression<'a> {
    fn equals<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        Compare::Equals(Box::new(self), Box::new(comparison.into()))
    }

    fn not_equals<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        Compare::NotEquals(Box::new(self), Box::new(comparison.into()))
    }

    fn is_not_distinct_from<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        Compare::IsNotDistinctFrom(Box::new(self), Box::new(comparison.into()))
    }

    fn less_than<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        Compare::LessThan(Box::new(self), Box::new(comparison.into()))
    }

    fn less_than_or_equals<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        Compare::LessThanOrEquals(Box::new(self), Box::new(comparison.into()))
    }

    fn greater_than<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        Compare::GreaterThan(Box::new(self), Box::new(comparison.into()))
    }

    fn greater_than_or_equals<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        Compare::GreaterThanOrEquals(Box::new(self), Box::new(comparison.into()))
    }

    fn in_selection<T>(self, selection: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        Compare::In(Box::new(self), Box::new(selection.into()))
    }

    fn not_in_selection<T>(self, selection: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        Compare::NotIn(Box::new(self), Box::new(selection.into()))
    }

    fn any_selection<T>(self, selection: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        Compare::AnySelection(Box::new(self), Box::new(selection.into()))
    }

    fn not_all_selection<T>(self, selection: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        Compare::NotAllSelection(Box::new(self), Box::new(selection.into()))
    }

    fn like<T>(self, pattern: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        Compare::Like(Box::new(self), Box::new(pattern.into()))
    }

    fn not_like<T>(self, pattern: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        Compare::NotLike(Box::new(self), Box::new(pattern.into()))
    }

    #[allow(clippy::wrong_self_convention)]
    fn is_null(self) -> Compare<'a> {
        Compare::Null(Box::new(self))
    }

    #[allow(clippy::wrong_self_convention)]
    fn is_not_null(self) -> Compare<'a> {
        Compare::NotNull(Box::new(self))
    }

    fn between<T, V>(self, left: T, right: V) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
        V: Into<Expression<'a>>,
    {
        Compare::Between(Box::new(self), Box::new(left.into()), Box::new(right.into()))
    }

    fn not_between<T, V>(self, left: T, right: V) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
        V: Into<Expression<'a>>,
    {
        Compare::NotBetween(Box::new(self), Box::new(left.into()), Box::new(right.into()))
    }

    fn compare_raw<T, V>(self, raw_comparator: T, right: V) -> Compare<'a>
    where
        T: Into<Cow<'a, str>>,
        V: Into<Expression<'a>>,
    {
        Compare::Raw(Box::new(self), raw_comparator.into(), Box::new(right.into()))
    }

    fn array_contains<T>(self, item: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        Compare::Json(JsonCompare::ArrayContains(Box::new(self), Box::new(item.into())))
    }

    fn array_contained<T>(self, item: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        Compare::Json(JsonCompare::ArrayContained(Box::new(self), Box::new(item.into())))
    }

    fn array_overlaps<T>(self, item: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        Compare::Json(JsonCompare::ArrayOverlaps(Box::new(self), Box::new(item.into())))
    }

    fn json_array_not_contains<T>(self, item: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        Compare::Json(JsonCompare::ArrayNotContains(Box::new(self), Box::new(item.into())))
    }

    fn json_type_equals<T>(self, json_type: T) -> Compare<'a>
    where
        T: Into<JsonType<'a>>,
    {
        Compare::Json(JsonCompare::TypeEquals(Box::new(self), json_type.into()))
    }

    fn json_type_not_equals<T>(self, json_type: T) -> Compare<'a>
    where
        T: Into<JsonType<'a>>,
    {
        Compare::Json(JsonCompare::TypeNotEquals(Box::new(self), json_type.into()))
    }

    fn any(self) -> Compare<'a> {
        Compare::Any(Box::new(self))
    }

    fn all(self) -> Compare<'a> {
        Compare::All(Box::new(self))
    }
}
