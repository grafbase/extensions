use base64::Engine as _;
use serde::ser::{self, SerializeMap as _, SerializeSeq as _};

use crate::schema;

#[derive(PartialEq, Debug)]
pub(crate) enum Value {
    Bytes(bytes::Bytes),
    String(bytes::Bytes),
    SignedInt(i64),
    UnsignedInt(u64),
    Bool(bool),
    Float(f32),
    Double(f64),
    List(Vec<Value>),
    Map(Vec<(Value, Value)>),
    Null,
}

impl Value {
    pub(crate) fn as_signed_int(&self) -> Option<i64> {
        match self {
            Value::SignedInt(i) => Some(*i),
            _ => None,
        }
    }

    pub(super) fn is_str(&self, s: &str) -> bool {
        match self {
            Value::String(bytes) => bytes == s.as_bytes(),
            _ => false,
        }
    }
}

pub(crate) struct ProtoSerialize<'a> {
    pub(super) value: &'a Value,
    pub(super) schema: &'a schema::Schema,
}

impl<'a> ProtoSerialize<'a> {
    fn focus<'b>(&self, value: &'b Value) -> ProtoSerialize<'b>
    where
        'a: 'b,
    {
        ProtoSerialize {
            value,
            schema: self.schema,
        }
    }
}

impl ser::Serialize for ProtoSerialize<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match self.value {
            Value::String(s) => serializer.serialize_str(
                std::str::from_utf8(s)
                    .map_err(|err| ser::Error::custom(format!("String from protobuf is not valid UTF-8: {err}")))?,
            ),
            Value::Bytes(items) => {
                let encoded = base64::engine::general_purpose::URL_SAFE.encode(items);

                serializer.serialize_str(&encoded)
            }
            Value::SignedInt(i) => serializer.serialize_i64(*i),
            Value::UnsignedInt(i) => serializer.serialize_u64(*i),
            Value::Float(f) => serializer.serialize_f32(*f),
            Value::Double(d) => serializer.serialize_f64(*d),
            Value::Bool(b) => serializer.serialize_bool(*b),
            Value::List(values) => {
                let mut seq = serializer.serialize_seq(Some(values.len()))?;

                for value in values {
                    seq.serialize_element(&self.focus(value))?;
                }

                seq.end()
            }
            Value::Map(items) => {
                let mut map = serializer.serialize_map(Some(items.len()))?;

                for (key, value) in items {
                    map.serialize_entry(&self.focus(key), &self.focus(value))?;
                }

                map.end()
            }
            Value::Null => serializer.serialize_unit(),
        }
    }
}
