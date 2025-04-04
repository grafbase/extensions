use super::ExpressionKind;
use crate::ast::{Column, ConditionTree, Expression};
use std::borrow::Cow;

/// For modeling comparison expressions.
#[derive(Debug, Clone, PartialEq)]
pub enum Compare<'a> {
    /// `left = right`
    Equals(Box<Expression<'a>>, Box<Expression<'a>>),
    /// `left <> right`
    NotEquals(Box<Expression<'a>>, Box<Expression<'a>>),
    /// `left < right`
    LessThan(Box<Expression<'a>>, Box<Expression<'a>>),
    /// `left <= right`
    LessThanOrEquals(Box<Expression<'a>>, Box<Expression<'a>>),
    /// `left > right`
    GreaterThan(Box<Expression<'a>>, Box<Expression<'a>>),
    /// `left >= right`
    GreaterThanOrEquals(Box<Expression<'a>>, Box<Expression<'a>>),
    /// `left IN (..)`
    In(Box<Expression<'a>>, Box<Expression<'a>>),
    /// `left NOT IN (..)`
    NotIn(Box<Expression<'a>>, Box<Expression<'a>>),
    /// `left = ANY (..)`
    AnySelection(Box<Expression<'a>>, Box<Expression<'a>>),
    /// `left <> ALL (..)`
    NotAllSelection(Box<Expression<'a>>, Box<Expression<'a>>),
    /// `left LIKE %..%`
    Like(Box<Expression<'a>>, Box<Expression<'a>>),
    /// `left NOT LIKE %..%`
    NotLike(Box<Expression<'a>>, Box<Expression<'a>>),
    /// `value IS NULL`
    Null(Box<Expression<'a>>),
    /// `value IS NOT NULL`
    NotNull(Box<Expression<'a>>),
    /// `value` BETWEEN `left` AND `right`
    Between(Box<Expression<'a>>, Box<Expression<'a>>, Box<Expression<'a>>),
    /// `value` NOT BETWEEN `left` AND `right`
    NotBetween(Box<Expression<'a>>, Box<Expression<'a>>, Box<Expression<'a>>),
    /// Raw comparator, allows to use an operator `left <raw> right` as is,
    /// without visitor transformation in between.
    Raw(Box<Expression<'a>>, Cow<'a, str>, Box<Expression<'a>>),
    /// All json related comparators
    Json(JsonCompare<'a>),
    /// ANY (`left`)
    Any(Box<Expression<'a>>),
    /// ALL (`left`)
    All(Box<Expression<'a>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum JsonCompare<'a> {
    ArrayOverlaps(Box<Expression<'a>>, Box<Expression<'a>>),
    ArrayContains(Box<Expression<'a>>, Box<Expression<'a>>),
    ArrayContained(Box<Expression<'a>>, Box<Expression<'a>>),
    ArrayNotContains(Box<Expression<'a>>, Box<Expression<'a>>),
    TypeEquals(Box<Expression<'a>>, JsonType<'a>),
    TypeNotEquals(Box<Expression<'a>>, JsonType<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum JsonType<'a> {
    Array,
    Object,
    String,
    Number,
    Boolean,
    Null,
    ColumnRef(Box<Column<'a>>),
}

impl<'a> From<Column<'a>> for JsonType<'a> {
    fn from(col: Column<'a>) -> Self {
        JsonType::ColumnRef(Box::new(col))
    }
}

impl<'a> From<Compare<'a>> for ConditionTree<'a> {
    fn from(cmp: Compare<'a>) -> Self {
        ConditionTree::single(Expression::from(cmp))
    }
}

impl<'a> From<Compare<'a>> for Expression<'a> {
    fn from(cmp: Compare<'a>) -> Self {
        Expression {
            kind: ExpressionKind::Compare(cmp),
            alias: None,
        }
    }
}

/// An item that can be compared against other values in the database.
pub trait Comparable<'a> {
    /// Tests if both sides are the same value.
    fn equals<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>;

    /// Tests if both sides are not the same value.
    fn not_equals<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>;

    /// Tests if the left side is smaller than the right side.
    fn less_than<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>;

    /// Tests if the left side is smaller than the right side or the same.
    fn less_than_or_equals<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>;

    /// Tests if the left side is bigger than the right side.
    fn greater_than<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>;

    /// Tests if the left side is bigger than the right side or the same.
    fn greater_than_or_equals<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>;

    /// Tests if the left side is included in the right side collection.
    fn in_selection<T>(self, selection: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>;

    /// Tests if the left side is equal to any element on the right side collection.
    /// Corresponds to `left = ANY (right)`.
    fn any_selection<T>(self, selection: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>;

    /// Tests if the left side is not equal to any element on the right side collection.
    /// Corresponds to `left <> ALL (right)`.
    fn not_all_selection<T>(self, selection: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>;

    /// Tests if the left side is not included in the right side collection.
    fn not_in_selection<T>(self, selection: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>;

    /// Tests if the left side includes the right side string.
    fn like<T>(self, pattern: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>;

    /// Tests if the left side does not include the right side string.
    fn not_like<T>(self, pattern: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>;

    /// Tests if the left side is `NULL`.
    #[allow(clippy::wrong_self_convention)]
    fn is_null(self) -> Compare<'a>;

    /// Tests if the left side is not `NULL`.
    #[allow(clippy::wrong_self_convention)]
    fn is_not_null(self) -> Compare<'a>;

    /// Tests if the value is between two given values.
    fn between<T, V>(self, left: T, right: V) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
        V: Into<Expression<'a>>;

    /// Tests if the value is not between two given values.
    fn not_between<T, V>(self, left: T, right: V) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
        V: Into<Expression<'a>>;

    /// Tests if the array overlaps with another array.
    fn array_overlaps<T>(self, item: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>;

    /// Tests if the array contains another array.
    fn array_contains<T>(self, item: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>;

    /// Tests if the JSON array contains a value.
    fn array_contained<T>(self, item: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>;

    /// Tests if the JSON array does not contain a value.
    fn json_array_not_contains<T>(self, item: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>;

    /// Tests if the JSON array starts with a value.
    fn json_array_begins_with<T>(self, item: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>;

    /// Tests if the JSON array does not start with a value.
    fn json_array_not_begins_with<T>(self, item: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>;

    /// Tests if the JSON array ends with a value.
    fn json_array_ends_into<T>(self, item: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>;

    /// Tests if the JSON array does not end with a value.
    fn json_array_not_ends_into<T>(self, item: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>;

    /// Tests if the JSON value is of a certain type.
    fn json_type_equals<T>(self, json_type: T) -> Compare<'a>
    where
        T: Into<JsonType<'a>>;

    /// Tests if the JSON value is not of a certain type.
    fn json_type_not_equals<T>(self, json_type: T) -> Compare<'a>
    where
        T: Into<JsonType<'a>>;

    /// Matches at least one elem of a list of values.
    fn any(self) -> Compare<'a>;

    /// Matches all elem of a list of values.
    fn all(self) -> Compare<'a>;

    /// Compares two expressions with a custom operator.
    fn compare_raw<T, V>(self, raw_comparator: T, right: V) -> Compare<'a>
    where
        T: Into<Cow<'a, str>>,
        V: Into<Expression<'a>>;
}

impl<'a, U> Comparable<'a> for U
where
    U: Into<Column<'a>>,
{
    fn equals<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: Expression<'a> = col.into();

        val.equals(comparison)
    }

    fn not_equals<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: Expression<'a> = col.into();
        val.not_equals(comparison)
    }

    fn less_than<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: Expression<'a> = col.into();
        val.less_than(comparison)
    }

    fn less_than_or_equals<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: Expression<'a> = col.into();
        val.less_than_or_equals(comparison)
    }

    fn greater_than<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: Expression<'a> = col.into();
        val.greater_than(comparison)
    }

    fn greater_than_or_equals<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: Expression<'a> = col.into();
        val.greater_than_or_equals(comparison)
    }

    fn in_selection<T>(self, selection: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: Expression<'a> = col.into();
        val.in_selection(selection)
    }

    fn not_in_selection<T>(self, selection: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: Expression<'a> = col.into();
        val.not_in_selection(selection)
    }

    fn like<T>(self, pattern: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: Expression<'a> = col.into();
        val.like(pattern)
    }

    fn not_like<T>(self, pattern: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: Expression<'a> = col.into();
        val.not_like(pattern)
    }

    #[allow(clippy::wrong_self_convention)]
    fn is_null(self) -> Compare<'a> {
        let col: Column<'a> = self.into();
        let val: Expression<'a> = col.into();
        val.is_null()
    }

    #[allow(clippy::wrong_self_convention)]
    fn is_not_null(self) -> Compare<'a> {
        let col: Column<'a> = self.into();
        let val: Expression<'a> = col.into();
        val.is_not_null()
    }

    fn between<T, V>(self, left: T, right: V) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
        V: Into<Expression<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: Expression<'a> = col.into();
        val.between(left, right)
    }

    fn not_between<T, V>(self, left: T, right: V) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
        V: Into<Expression<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: Expression<'a> = col.into();
        val.not_between(left, right)
    }

    fn compare_raw<T, V>(self, raw_comparator: T, right: V) -> Compare<'a>
    where
        T: Into<Cow<'a, str>>,
        V: Into<Expression<'a>>,
    {
        let left: Column<'a> = self.into();
        let left: Expression<'a> = left.into();
        let right: Expression<'a> = right.into();

        left.compare_raw(raw_comparator.into(), right)
    }

    fn array_overlaps<T>(self, item: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: Expression<'a> = col.into();

        val.array_overlaps(item)
    }

    fn array_contains<T>(self, item: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: Expression<'a> = col.into();

        val.array_contains(item)
    }

    fn array_contained<T>(self, item: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: Expression<'a> = col.into();

        val.array_contained(item)
    }

    fn json_array_not_contains<T>(self, item: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: Expression<'a> = col.into();

        val.json_array_not_contains(item)
    }

    fn json_array_begins_with<T>(self, item: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: Expression<'a> = col.into();

        val.json_array_begins_with(item)
    }

    fn json_array_not_begins_with<T>(self, item: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: Expression<'a> = col.into();

        val.json_array_not_begins_with(item)
    }

    fn json_array_ends_into<T>(self, item: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: Expression<'a> = col.into();

        val.json_array_ends_into(item)
    }

    fn json_array_not_ends_into<T>(self, item: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: Expression<'a> = col.into();

        val.json_array_not_ends_into(item)
    }

    fn json_type_equals<T>(self, json_type: T) -> Compare<'a>
    where
        T: Into<JsonType<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: Expression<'a> = col.into();

        val.json_type_equals(json_type)
    }

    fn json_type_not_equals<T>(self, json_type: T) -> Compare<'a>
    where
        T: Into<JsonType<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: Expression<'a> = col.into();

        val.json_type_not_equals(json_type)
    }

    fn any(self) -> Compare<'a> {
        let col: Column<'a> = self.into();
        let val: Expression<'a> = col.into();

        val.any()
    }

    fn all(self) -> Compare<'a> {
        let col: Column<'a> = self.into();
        let val: Expression<'a> = col.into();

        val.all()
    }

    fn any_selection<T>(self, selection: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: Expression<'a> = col.into();
        val.any_selection(selection)
    }

    fn not_all_selection<T>(self, selection: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let col: Column<'a> = self.into();
        let val: Expression<'a> = col.into();
        val.not_all_selection(selection)
    }
}
