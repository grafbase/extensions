use super::compare::JsonType;
use crate::ast::{Comparable, Compare, Expression};
use std::borrow::Cow;

/// A collection of values surrounded by parentheses.
#[derive(Debug, Default, Clone)]
pub struct Row<'a> {
    pub values: Vec<Expression<'a>>,
}

impl<'a> Row<'a> {
    pub fn new() -> Self {
        Row { values: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Row {
            values: Vec::with_capacity(capacity),
        }
    }

    pub fn pop(&mut self) -> Option<Expression<'a>> {
        self.values.pop()
    }

    pub fn push<T>(&mut self, value: T)
    where
        T: Into<Expression<'a>>,
    {
        self.values.push(value.into());
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl<'a> IntoIterator for Row<'a> {
    type Item = Expression<'a>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl<'a, T> From<Vec<T>> for Row<'a>
where
    T: Into<Expression<'a>>,
{
    fn from(vector: Vec<T>) -> Row<'a> {
        let mut row = Row::with_capacity(vector.len());

        for v in vector.into_iter() {
            row.push(v.into());
        }

        row
    }
}

impl<'a, A> From<(A,)> for Row<'a>
where
    A: Into<Expression<'a>>,
{
    fn from((val,): (A,)) -> Self {
        let mut row = Row::with_capacity(1);
        row.push(val);
        row
    }
}

impl<'a, A, B> From<(A, B)> for Row<'a>
where
    A: Into<Expression<'a>>,
    B: Into<Expression<'a>>,
{
    fn from(vals: (A, B)) -> Self {
        let mut row = Row::with_capacity(2);

        row.push(vals.0);
        row.push(vals.1);

        row
    }
}

impl<'a, A, B, C> From<(A, B, C)> for Row<'a>
where
    A: Into<Expression<'a>>,
    B: Into<Expression<'a>>,
    C: Into<Expression<'a>>,
{
    fn from(vals: (A, B, C)) -> Self {
        let mut row = Row::with_capacity(3);

        row.push(vals.0);
        row.push(vals.1);
        row.push(vals.2);

        row
    }
}

impl<'a, A, B, C, D> From<(A, B, C, D)> for Row<'a>
where
    A: Into<Expression<'a>>,
    B: Into<Expression<'a>>,
    C: Into<Expression<'a>>,
    D: Into<Expression<'a>>,
{
    fn from(vals: (A, B, C, D)) -> Self {
        let mut row = Row::with_capacity(4);

        row.push(vals.0);
        row.push(vals.1);
        row.push(vals.2);
        row.push(vals.3);

        row
    }
}

impl<'a, A, B, C, D, E> From<(A, B, C, D, E)> for Row<'a>
where
    A: Into<Expression<'a>>,
    B: Into<Expression<'a>>,
    C: Into<Expression<'a>>,
    D: Into<Expression<'a>>,
    E: Into<Expression<'a>>,
{
    fn from(vals: (A, B, C, D, E)) -> Self {
        let mut row = Row::with_capacity(5);

        row.push(vals.0);
        row.push(vals.1);
        row.push(vals.2);
        row.push(vals.3);
        row.push(vals.4);

        row
    }
}

impl<'a> Comparable<'a> for Row<'a> {
    fn equals<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let value: Expression<'a> = self.into();
        value.equals(comparison)
    }

    fn not_equals<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let value: Expression<'a> = self.into();
        value.not_equals(comparison)
    }

    fn is_not_distinct_from<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let value: Expression<'a> = self.into();
        value.is_not_distinct_from(comparison)
    }

    fn less_than<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let value: Expression<'a> = self.into();
        value.less_than(comparison)
    }

    fn less_than_or_equals<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let value: Expression<'a> = self.into();
        value.less_than_or_equals(comparison)
    }

    fn greater_than<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let value: Expression<'a> = self.into();
        value.greater_than(comparison)
    }

    fn greater_than_or_equals<T>(self, comparison: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let value: Expression<'a> = self.into();
        value.greater_than_or_equals(comparison)
    }

    fn in_selection<T>(self, selection: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let value: Expression<'a> = self.into();
        value.in_selection(selection)
    }

    fn not_in_selection<T>(self, selection: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let value: Expression<'a> = self.into();
        value.not_in_selection(selection)
    }

    fn any_selection<T>(self, selection: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let value: Expression<'a> = self.into();
        value.any_selection(selection)
    }

    fn not_all_selection<T>(self, selection: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let value: Expression<'a> = self.into();
        value.not_all_selection(selection)
    }

    fn like<T>(self, pattern: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let value: Expression<'a> = self.into();
        value.like(pattern)
    }

    fn not_like<T>(self, pattern: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let value: Expression<'a> = self.into();
        value.not_like(pattern)
    }

    #[allow(clippy::wrong_self_convention)]
    fn is_null(self) -> Compare<'a> {
        let value: Expression<'a> = self.into();
        value.is_null()
    }

    #[allow(clippy::wrong_self_convention)]
    fn is_not_null(self) -> Compare<'a> {
        let value: Expression<'a> = self.into();
        value.is_not_null()
    }

    fn between<T, V>(self, left: T, right: V) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
        V: Into<Expression<'a>>,
    {
        let value: Expression<'a> = self.into();
        value.between(left, right)
    }

    fn not_between<T, V>(self, left: T, right: V) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
        V: Into<Expression<'a>>,
    {
        let value: Expression<'a> = self.into();
        value.not_between(left, right)
    }

    fn compare_raw<T, V>(self, raw_comparator: T, right: V) -> Compare<'a>
    where
        T: Into<Cow<'a, str>>,
        V: Into<Expression<'a>>,
    {
        let value: Expression<'a> = self.into();
        value.compare_raw(raw_comparator, right)
    }

    /// Tests if the array overlaps with another array.
    fn array_overlaps<T>(self, item: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let value: Expression<'a> = self.into();

        value.array_overlaps(item)
    }

    fn array_contains<T>(self, item: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let value: Expression<'a> = self.into();

        value.array_contains(item)
    }

    fn array_contained<T>(self, item: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let value: Expression<'a> = self.into();

        value.array_contained(item)
    }

    fn json_array_not_contains<T>(self, item: T) -> Compare<'a>
    where
        T: Into<Expression<'a>>,
    {
        let value: Expression<'a> = self.into();

        value.json_array_not_contains(item)
    }

    fn json_type_equals<T>(self, json_type: T) -> Compare<'a>
    where
        T: Into<JsonType<'a>>,
    {
        let value: Expression<'a> = self.into();

        value.json_type_equals(json_type)
    }

    fn json_type_not_equals<T>(self, json_type: T) -> Compare<'a>
    where
        T: Into<JsonType<'a>>,
    {
        let value: Expression<'a> = self.into();

        value.json_type_not_equals(json_type)
    }

    fn any(self) -> Compare<'a> {
        let value: Expression<'a> = self.into();

        value.any()
    }

    fn all(self) -> Compare<'a> {
        let value: Expression<'a> = self.into();

        value.all()
    }
}
