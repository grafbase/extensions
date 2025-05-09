use std::borrow::Cow;

use crate::context::{
    PageInfo, filter::FilterIterator, order::LookupOrderIterator, selection_iterator::SelectionIterator,
};
use grafbase_database_definition::{RelationWalker, TableWalker};

/// A builder for building a PostgreSQL `SELECT` statement.
pub struct SelectBuilder<'a> {
    table: TableWalker<'a>,
    selection: SelectionIterator<'a>,
    filter: Option<FilterIterator<'a>>,
    field_name: Cow<'static, str>,
    relation: Option<RelationWalker<'a>>,
    lookup_order: Option<LookupOrderIterator<'a>>,
}

impl<'a> SelectBuilder<'a> {
    /// Starting from the given table, select the fields in the iterator
    /// and name the selection with `field_name`.
    pub fn new(
        table: TableWalker<'a>,
        selection: SelectionIterator<'a>,
        field_name: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            table,
            selection,
            filter: None,
            field_name: field_name.into(),
            relation: None,
            lookup_order: None,
        }
    }

    /// Returns whether the selection includes a cursor.
    pub fn needs_cursor(&self) -> bool {
        self.selection.needs_cursor()
    }

    /// Returns whether the selection includes a cursor.
    pub fn selects_cursor(&self) -> bool {
        self.selection.selects_cursor()
    }

    /// Returns `true` if the selection includes edges.
    pub fn selects_edges(&self) -> bool {
        self.selection.selects_edges()
    }

    /// Returns `true` if the selection includes nodes.
    pub fn selects_nodes(&self) -> bool {
        self.selection.selects_nodes()
    }

    /// Retrieves the pagination information if it exists in the selection.
    pub fn page_info(&self) -> Option<PageInfo> {
        self.selection.page_info()
    }

    /// Adds a `WHERE` clause to the statement.
    pub fn set_filter(&mut self, filter: FilterIterator<'a>) {
        self.filter = Some(filter);
    }

    /// Marks the query as a selection for a relation.
    pub fn set_relation(&mut self, relation: RelationWalker<'a>) {
        self.relation = Some(relation);
    }

    /// Sets the order of an entity lookup.
    pub fn set_lookup_order(&mut self, lookup: LookupOrderIterator<'a>) {
        self.lookup_order = Some(lookup);
    }

    /// The name of the table we're selecting from.
    pub fn table(&self) -> TableWalker<'a> {
        self.table
    }

    /// The selected fields from the user.
    pub fn selection(&self) -> SelectionIterator<'a> {
        self.selection.clone()
    }

    /// How we name the result of this query. Set to `root` if generating the main query,
    /// and to the name of the relation field if creating a select for a join.
    pub fn field_name(&'a self) -> &'a str {
        &self.field_name
    }

    /// The `WHERE` statement for this select.
    pub fn filter(&self) -> Option<FilterIterator<'a>> {
        self.filter.clone()
    }

    /// The order of an entity lookup, if set.
    pub fn lookup_order(&self) -> Option<LookupOrderIterator<'a>> {
        self.lookup_order.clone()
    }

    /// If selecting for a join, this should have the definition of the relation we're
    /// currently on.
    pub fn relation(&self) -> Option<RelationWalker<'a>> {
        self.relation
    }
}
