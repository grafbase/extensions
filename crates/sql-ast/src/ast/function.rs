mod aggregate_to_string;
mod array_position;
mod average;
mod cast;
mod coalesce;
mod concat;
mod convert_from;
mod count;
mod decode;
mod encode;
mod json_agg;
mod json_build_array;
mod json_build_object;
mod json_extract;
mod json_extract_array;
mod json_unquote;
mod lower;
mod maximum;
mod minimum;
mod replace;
mod row_number;
mod row_to_json;
mod sum;
mod to_jsonb;
mod unnest;
mod upper;

pub use aggregate_to_string::*;
pub use array_position::*;
pub use average::*;
pub use cast::*;
pub use coalesce::*;
pub use concat::*;
pub use convert_from::*;
pub use count::*;
pub use decode::*;
pub use encode::*;
pub use json_agg::*;
pub use json_build_array::*;
pub use json_build_object::*;
pub use json_extract::*;
pub use json_extract_array::*;
pub use json_unquote::*;
pub use lower::*;
pub use maximum::*;
pub use minimum::*;
pub use replace::*;
pub use row_number::*;
pub use row_to_json::*;
pub use sum::*;
pub use to_jsonb::*;
pub use unnest::*;
pub use upper::*;

use super::{Alias, Aliasable};

/// A database function definition
#[derive(Debug, Clone, PartialEq)]
pub struct Function<'a> {
    pub(crate) r#type: FunctionType<'a>,
    pub(crate) alias: Option<Alias<'a>>,
}

impl Function<'_> {
    pub fn returns_json(&self) -> bool {
        matches!(
            self.r#type,
            FunctionType::RowToJson(_)
                | FunctionType::JsonExtract(_)
                | FunctionType::JsonExtractArrayElem(_)
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
    JsonExtractArrayElem(JsonExtractArrayElem<'a>),
    JsonUnquote(JsonUnquote<'a>),
    RowToJson(RowToJson<'a>),
    ToJsonb(ToJsonb<'a>),
    JsonAgg(JsonAgg<'a>),
    Encode(Encode<'a>),
    JsonBuildObject(JsonBuildObject<'a>),
    Unnest(Unnest<'a>),
    ArrayPosition(ArrayPosition<'a>),
    JsonBuildArray(JsonBuildArray<'a>),
    RowNumber(RowNumber<'a>),
    Decode(Decode<'a>),
    ConvertFrom(ConvertFrom<'a>),
    Replace(Replace<'a>),
}

impl<'a> Aliasable<'a> for Function<'a> {
    type Target = Function<'a>;

    fn alias<T>(mut self, alias: T) -> Self::Target
    where
        T: Into<Alias<'a>>,
    {
        self.alias = Some(alias.into());
        self
    }
}
