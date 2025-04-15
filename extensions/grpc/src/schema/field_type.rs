use super::*;
use std::{
    fmt::{self, Display as _},
    str::FromStr,
};

#[expect(clippy::unnecessary_lazy_evaluations)] // clippy is drunk
pub(in crate::schema) fn ingest_type(
    r#type: &str,
    messages_by_name: &HashMap<&str, MessageDefinitionId>,
    enums_by_name: &HashMap<&str, EnumDefinitionId>,
) -> Option<FieldType> {
    ScalarType::from_str(r#type)
        .ok()
        .map(FieldType::Scalar)
        .or_else(|| messages_by_name.get(r#type).map(|id| FieldType::Message(*id)))
        .or_else(|| enums_by_name.get(r#type).map(|id| FieldType::Enum(*id)))
        .or_else(|| {
            if !r#type.starts_with("map<") || !r#type.ends_with(">") {
                return None;
            };

            let r#type = r#type.trim_start_matches("map<").trim_end_matches(">");
            let (key_type, value_type) = r#type.split_once(',')?;

            let key_type = ingest_type(key_type.trim(), messages_by_name, enums_by_name)?;
            let value_type = ingest_type(value_type.trim(), messages_by_name, enums_by_name)?;

            Some(FieldType::Map(Box::new((key_type, value_type))))
        })
        .or_else(|| match r#type {
            ".google.protobuf.Duration" => Some(FieldType::Duration),
            ".google.protobuf.Timestamp" => Some(FieldType::Timestamp),
            ".google.protobuf.Empty" => Some(FieldType::Empty),
            ".google.protobuf.NullValue" => Some(FieldType::NullValue),
            ".google.protobuf.FieldMask" => Some(FieldType::FieldMask),

            ".google.protobuf.BoolValue" => Some(FieldType::Wrapper(ScalarType::Bool)),
            ".google.protobuf.BytesValue" => Some(FieldType::Wrapper(ScalarType::Bytes)),
            ".google.protobuf.DoubleValue" => Some(FieldType::Wrapper(ScalarType::Double)),
            ".google.protobuf.FloatValue" => Some(FieldType::Wrapper(ScalarType::Float)),
            ".google.protobuf.Int32Value" => Some(FieldType::Wrapper(ScalarType::Int32)),
            ".google.protobuf.Int64Value" => Some(FieldType::Wrapper(ScalarType::Int64)),
            ".google.protobuf.StringValue" => Some(FieldType::Wrapper(ScalarType::String)),
            ".google.protobuf.UInt32Value" => Some(FieldType::Wrapper(ScalarType::UInt32)),
            ".google.protobuf.UInt64Value" => Some(FieldType::Wrapper(ScalarType::UInt64)),

            _ => None,
        })
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum FieldType {
    Scalar(ScalarType),
    Message(MessageDefinitionId),
    Enum(EnumDefinitionId),
    Map(Box<(FieldType, FieldType)>),

    /// The BoolValue, BytesValue, etc. well-known types.
    /// See https://protobuf.dev/reference/protobuf/google.protobuf/#bool-value
    Wrapper(ScalarType),
    /// https://protobuf.dev/reference/protobuf/google.protobuf/#null-value
    NullValue,
    /// https://protobuf.dev/reference/protobuf/google.protobuf/#duration
    Duration,
    /// https://protobuf.dev/reference/protobuf/google.protobuf/#timestamp
    Timestamp,
    /// https://protobuf.dev/reference/protobuf/google.protobuf/#empty
    Empty,
    /// https://protobuf.dev/reference/protobuf/google.protobuf/#field-mask
    FieldMask,
}

impl FieldType {
    pub(crate) fn proto_name(&self, schema: &Schema) -> impl fmt::Display {
        display_fn(move |f| match self {
            FieldType::Scalar(scalar_type) => f.write_str(scalar_type.proto_name()),
            FieldType::Message(proto_message_id) => f.write_str(schema[*proto_message_id].name.as_str()),
            FieldType::Enum(proto_enum_id) => f.write_str(schema[*proto_enum_id].name.as_str()),
            FieldType::Map(kv) => {
                let (k, v) = kv.as_ref();
                f.write_str("map<")?;
                k.proto_name(schema).fmt(f)?;
                f.write_str(", ")?;
                v.proto_name(schema).fmt(f)?;
                f.write_str(">")
            }
            FieldType::Wrapper(scalar_type) => match scalar_type {
                ScalarType::Double => f.write_str(".google.protobuf.DoubleValue"),
                ScalarType::Float => f.write_str(".google.protobuf.FloatValue"),
                ScalarType::Int32 => f.write_str(".google.protobuf.Int32Value"),
                ScalarType::Int64 => f.write_str(".google.protobuf.Int64Value"),
                ScalarType::UInt32 => f.write_str(".google.protobuf.UInt32Value"),
                ScalarType::UInt64 => f.write_str(".google.protobuf.UInt64Value"),
                ScalarType::Sint32 => f.write_str(".google.protobuf.Sint32Value"),
                ScalarType::Sint64 => f.write_str(".google.protobuf.Sint64Value"),
                ScalarType::Fixed32 => f.write_str(".google.protobuf.Fixed32Value"),
                ScalarType::Fixed64 => f.write_str(".google.protobuf.Fixed64Value"),
                ScalarType::Sfixed32 => f.write_str(".google.protobuf.Sfixed32Value"),
                ScalarType::Sfixed64 => f.write_str(".google.protobuf.Sfixed64Value"),
                ScalarType::Bool => f.write_str(".google.protobuf.BoolValue"),
                ScalarType::String => f.write_str(".google.protobuf.StringValue"),
                ScalarType::Bytes => f.write_str(".google.protobuf.BytesValue"),
            },
            FieldType::NullValue => f.write_str(".google.protobuf.NullValue"),
            FieldType::Duration => f.write_str(".google.protobuf.Duration"),
            FieldType::Empty => f.write_str(".google.protobuf.Empty"),
            FieldType::FieldMask => f.write_str(".google.protobuf.FieldMask"),
            FieldType::Timestamp => f.write_str(".google.protobuf.Timestamp"),
        })
    }
}

fn display_fn(f: impl Fn(&mut fmt::Formatter<'_>) -> fmt::Result) -> impl fmt::Display {
    struct DisplayFn<T>(T);

    impl<T> fmt::Display for DisplayFn<T>
    where
        T: Fn(&mut fmt::Formatter<'_>) -> fmt::Result,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            (self.0)(f)
        }
    }

    DisplayFn(f)
}

/// See scalar value types table in [the reference](https://protobuf.dev/programming-guides/proto3/).
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum ScalarType {
    Double,
    Float,
    Int32,
    Int64,
    UInt32,
    UInt64,
    Sint32,
    Sint64,
    Fixed32,
    Fixed64,
    Sfixed32,
    Sfixed64,
    Bool,
    String,
    Bytes,
}

impl ScalarType {
    pub(crate) fn proto_name(&self) -> &'static str {
        match self {
            ScalarType::Double => "double",
            ScalarType::Float => "float",
            ScalarType::Int32 => "int32",
            ScalarType::Int64 => "int64",
            ScalarType::UInt32 => "uint32",
            ScalarType::UInt64 => "uint64",
            ScalarType::Sint32 => "sint32",
            ScalarType::Sint64 => "sint64",
            ScalarType::Fixed32 => "fixed32",
            ScalarType::Fixed64 => "fixed64",
            ScalarType::Sfixed32 => "sfixed32",
            ScalarType::Sfixed64 => "sfixed64",
            ScalarType::Bool => "bool",
            ScalarType::String => "string",
            ScalarType::Bytes => "bytes",
        }
    }
}

impl FromStr for ScalarType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "double" => Ok(ScalarType::Double),
            "float" => Ok(ScalarType::Float),
            "int32" => Ok(ScalarType::Int32),
            "int64" => Ok(ScalarType::Int64),
            "uint32" => Ok(ScalarType::UInt32),
            "uint64" => Ok(ScalarType::UInt64),
            "sint32" => Ok(ScalarType::Sint32),
            "sint64" => Ok(ScalarType::Sint64),
            "fixed32" => Ok(ScalarType::Fixed32),
            "fixed64" => Ok(ScalarType::Fixed64),
            "sfixed32" => Ok(ScalarType::Sfixed32),
            "sfixed64" => Ok(ScalarType::Sfixed64),
            "bool" => Ok(ScalarType::Bool),
            "string" => Ok(ScalarType::String),
            "bytes" => Ok(ScalarType::Bytes),
            _ => Err(()),
        }
    }
}
