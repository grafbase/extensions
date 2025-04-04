use crate::ast::{ConditionTree, Expression};

/// `AND`, `OR` and `NOT` conjunctive implementations.
pub trait Conjunctive<'a> {
    /// Builds an `AND` condition having `self` as the left leaf and `other` as the right.
    fn and<E>(self, other: E) -> ConditionTree<'a>
    where
        E: Into<Expression<'a>>;

    /// Builds an `OR` condition having `self` as the left leaf and `other` as the right.
    fn or<E>(self, other: E) -> ConditionTree<'a>
    where
        E: Into<Expression<'a>>;

    /// Builds a `NOT` condition having `self` as the condition.
    fn not(self) -> ConditionTree<'a>;
}

impl<'a, T> Conjunctive<'a> for T
where
    T: Into<Expression<'a>>,
{
    fn and<E>(self, other: E) -> ConditionTree<'a>
    where
        E: Into<Expression<'a>>,
    {
        ConditionTree::And(vec![self.into(), other.into()])
    }

    fn or<E>(self, other: E) -> ConditionTree<'a>
    where
        E: Into<Expression<'a>>,
    {
        ConditionTree::Or(vec![self.into(), other.into()])
    }

    fn not(self) -> ConditionTree<'a> {
        ConditionTree::not(self.into())
    }
}
