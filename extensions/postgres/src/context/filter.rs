mod complex;
mod simple;

pub(super) use complex::MultipleFilterIterator;
use grafbase_sdk::SdkError;
pub(super) use simple::UniqueFilterIterator;

use sql_ast::ast::ConditionTree;

#[derive(Clone)]
pub enum FilterIterator<'a> {
    Unique(UniqueFilterIterator<'a>),
    Multiple(MultipleFilterIterator<'a>),
}

impl<'a> Iterator for FilterIterator<'a> {
    type Item = Result<ConditionTree<'a>, SdkError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            FilterIterator::Unique(iterator) => match iterator.next() {
                Some(Ok(condition)) => Some(Ok(ConditionTree::from(condition))),
                Some(Err(err)) => Some(Err(err)),
                None => None,
            },
            FilterIterator::Multiple(iterator) => iterator.next(),
        }
    }
}
