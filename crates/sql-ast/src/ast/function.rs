mod aggregate_to_string;
mod average;
mod cast;
mod coalesce;
mod concat;
mod count;
mod encode;
mod json_agg;
mod json_build_object;
mod json_extract;
mod json_extract_array;
mod json_unquote;
mod lower;
mod maximum;
mod minimum;
mod row_number;
mod row_to_json;
mod sum;
mod to_jsonb;
mod unnest;
mod upper;

pub use aggregate_to_string::*;
pub use average::*;
pub use cast::*;
pub use coalesce::*;
pub use concat::*;
pub use count::*;
pub use encode::*;
pub use json_agg::*;
pub use json_build_object::*;
pub use json_extract::*;
pub(crate) use json_extract_array::*;
pub use json_unquote::*;
pub use lower::*;
pub use maximum::*;
pub use minimum::*;
pub use row_number::*;
pub use row_to_json::*;
pub use sum::*;
pub use to_jsonb::*;
pub use unnest::*;
pub use upper::*;

use super::Aliasable;
use std::borrow::Cow;

/// A database function definition
#[derive(Debug, Clone, PartialEq)]
pub struct Function<'a> {
    pub(crate) typ_: FunctionType<'a>,
    pub(crate) alias: Option<Cow<'a, str>>,
}

impl Function<'_> {
    pub fn returns_json(&self) -> bool {
        matches!(
            self.typ_,
            FunctionType::RowToJson(_)
                | FunctionType::JsonExtract(_)
                | FunctionType::JsonExtractLastArrayElem(_)
                | FunctionType::JsonExtractFirstArrayElem(_)
                | FunctionType::ToJsonb(_)
        )
    }
}

/// A database function type
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FunctionType<'a> {
    Count(Count<'a>),
    Cast(Cast<'a>),
    AggregateToString(AggregateToString<'a>),
    Average(Average<'a>),
    Sum(Sum<'a>),
    Lower(Lower<'a>),
    Upper(Upper<'a>),
    Minimum(Minimum<'a>),
    Maximum(Maximum<'a>),
    Coalesce(Coalesce<'a>),
    Concat(Concat<'a>),
    JsonExtract(JsonExtract<'a>),
    JsonExtractLastArrayElem(JsonExtractLastArrayElem<'a>),
    JsonExtractFirstArrayElem(JsonExtractFirstArrayElem<'a>),
    JsonUnquote(JsonUnquote<'a>),
    RowToJson(RowToJson<'a>),
    ToJsonb(ToJsonb<'a>),
    JsonbAgg(JsonbAgg<'a>),
    Encode(Encode<'a>),
    JsonBuildObject(JsonBuildObject<'a>),
    Unnest(Unnest<'a>),
}

impl<'a> Aliasable<'a> for Function<'a> {
    type Target = Function<'a>;

    fn alias<T>(mut self, alias: T) -> Self::Target
    where
        T: Into<Cow<'a, str>>,
    {
        self.alias = Some(alias.into());
        self
    }
}
