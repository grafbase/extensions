use std::{borrow::Cow, str::FromStr};

use crate::schema::{self, FieldType, ScalarType};
use base64::Engine as _;
use prost::encoding;
use serde::{Deserialize as _, de};

pub(crate) struct FieldVisitor<'a> {
    pub(super) schema: &'a schema::Schema,
    pub(super) field_type: &'a schema::FieldType,
    pub(super) tag: u32,
    pub(super) out: &'a mut Vec<u8>,
}

impl<'de> de::DeserializeSeed<'de> for FieldVisitor<'_> {
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        match self.field_type {
            FieldType::Scalar(_)
            | FieldType::Wrapper(_)
            | FieldType::NullValue
            | FieldType::Duration
            | FieldType::FieldMask
            | FieldType::Timestamp
            | FieldType::Empty => deserializer.deserialize_any(self),
            FieldType::Message(message_id) => {
                let mut submessage = Vec::new();

                super::MessageDeserialize {
                    schema: self.schema,
                    message_definition: &self.schema[*message_id],
                    out: &mut submessage,
                }
                .deserialize(deserializer)?;

                encoding::encode_key(self.tag, encoding::WireType::LengthDelimited, self.out);
                encoding::encode_varint(submessage.len() as u64, self.out);
                self.out.extend_from_slice(&submessage);

                Ok(())
            }
            FieldType::Enum(enum_definition_id) => {
                let value_str = Cow::<'_, str>::deserialize(deserializer)?;

                let enum_definition = &self.schema[*enum_definition_id];
                let value = enum_definition
                    .values
                    .iter()
                    .find(|v| v.name == value_str)
                    .ok_or_else(|| {
                        de::Error::invalid_value(
                            de::Unexpected::Str(&value_str),
                            &format!("A value of the {} enum", enum_definition.name).as_str(),
                        )
                    })?;

                encoding::encode_key(self.tag, encoding::WireType::Varint, self.out);
                encoding::encode_varint(value.number.into(), self.out);

                Ok(())
            }
            FieldType::Map(map_definition) => {
                let (key_type, value_type) = map_definition.as_ref();

                super::map::MapDeserialize {
                    schema: self.schema,
                    key_type,
                    value_type,
                    out: self.out,
                    tag: self.tag,
                }
                .deserialize(deserializer)?;

                Ok(())
            }
        }
    }
}

impl<'de> de::Visitor<'de> for FieldVisitor<'_> {
    type Value = ();

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a {} value", self.field_type.proto_name(self.schema))
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match self.field_type {
            FieldType::Scalar(ScalarType::Bool) => {
                encoding::bool::encode(self.tag, &v, self.out);
                Ok(())
            }
            FieldType::Wrapper(ScalarType::Bool) => encode_message(self.tag, self.out, |out| {
                encoding::bool::encode(1, &v, out);
                Ok(())
            }),
            _ => Err(de::Error::invalid_type(de::Unexpected::Bool(v), &self)),
        }
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match self.field_type {
            FieldType::Scalar(ScalarType::Int32) => {
                encoding::int32::encode(self.tag, &(v as i32), self.out);
                Ok(())
            }
            FieldType::Scalar(ScalarType::Int64) => {
                encoding::int64::encode(self.tag, &v, self.out);
                Ok(())
            }
            FieldType::Scalar(ScalarType::Sint32) => {
                encoding::sint32::encode(self.tag, &(v as i32), self.out);
                Ok(())
            }
            FieldType::Scalar(ScalarType::Sint64) => {
                encoding::sint64::encode(self.tag, &v, self.out);
                Ok(())
            }
            FieldType::Scalar(ScalarType::Sfixed32) => {
                encoding::sfixed32::encode(self.tag, &(v as i32), self.out);
                Ok(())
            }
            FieldType::Scalar(ScalarType::Sfixed64) => {
                encoding::sfixed64::encode(self.tag, &v, self.out);
                Ok(())
            }
            FieldType::Scalar(ScalarType::Double) => {
                encoding::double::encode(self.tag, &(v as f64), self.out);
                Ok(())
            }
            FieldType::Scalar(ScalarType::Float) => {
                encoding::float::encode(self.tag, &(v as f32), self.out);
                Ok(())
            }
            FieldType::Wrapper(ScalarType::Int32) => encode_message(self.tag, self.out, |out| {
                encoding::int32::encode(1, &(v as i32), out);
                Ok(())
            }),
            FieldType::Wrapper(ScalarType::Int64) => encode_message(self.tag, self.out, |out| {
                encoding::int64::encode(1, &v, out);
                Ok(())
            }),
            FieldType::Wrapper(ScalarType::UInt32) => encode_message(self.tag, self.out, |out| {
                encoding::uint32::encode(1, &(v as u32), out);
                Ok(())
            }),
            FieldType::Wrapper(ScalarType::UInt64) => encode_message(self.tag, self.out, |out| {
                encoding::uint64::encode(1, &(v as u64), out);
                Ok(())
            }),

            _ => Err(de::Error::invalid_type(de::Unexpected::Signed(v), &self)),
        }
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match self.field_type {
            FieldType::Scalar(ScalarType::Int32) => {
                encoding::uint32::encode(self.tag, &(v as u32), self.out);
                Ok(())
            }
            FieldType::Scalar(ScalarType::Int64) => {
                encoding::uint64::encode(self.tag, &v, self.out);
                Ok(())
            }
            FieldType::Scalar(ScalarType::Sint32) => {
                encoding::sint32::encode(self.tag, &(v as i32), self.out);
                Ok(())
            }
            FieldType::Scalar(ScalarType::Sint64) => {
                encoding::sint64::encode(self.tag, &(v as i64), self.out);
                Ok(())
            }
            FieldType::Scalar(ScalarType::Sfixed32) => {
                encoding::sfixed32::encode(self.tag, &(v as i32), self.out);
                Ok(())
            }
            FieldType::Scalar(ScalarType::Sfixed64) => {
                encoding::sfixed64::encode(self.tag, &(v as i64), self.out);
                Ok(())
            }
            FieldType::Scalar(ScalarType::Fixed32) => {
                encoding::fixed32::encode(self.tag, &(v as u32), self.out);
                Ok(())
            }
            FieldType::Scalar(ScalarType::Fixed64) => {
                encoding::fixed64::encode(self.tag, &v, self.out);
                Ok(())
            }
            FieldType::Scalar(ScalarType::Double) => {
                encoding::double::encode(self.tag, &(v as f64), self.out);
                Ok(())
            }
            FieldType::Scalar(ScalarType::Float) => {
                encoding::float::encode(self.tag, &(v as f32), self.out);
                Ok(())
            }
            FieldType::Wrapper(ScalarType::Int32) => encode_message(self.tag, self.out, |out| {
                encoding::int32::encode(1, &(v as i32), out);
                Ok(())
            }),
            FieldType::Wrapper(ScalarType::Int64) => encode_message(self.tag, self.out, |out| {
                encoding::int64::encode(1, &(v as i64), out);
                Ok(())
            }),
            FieldType::Wrapper(ScalarType::UInt32) => encode_message(self.tag, self.out, |out| {
                encoding::uint32::encode(1, &(v as u32), out);
                Ok(())
            }),
            FieldType::Wrapper(ScalarType::UInt64) => encode_message(self.tag, self.out, |out| {
                encoding::uint64::encode(1, &v, out);
                Ok(())
            }),

            _ => Err(de::Error::invalid_type(de::Unexpected::Unsigned(v), &self)),
        }
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match self.field_type {
            FieldType::Scalar(ScalarType::Double) => {
                encoding::double::encode(self.tag, &v, self.out);
                Ok(())
            }
            FieldType::Scalar(ScalarType::Float) => {
                encoding::float::encode(self.tag, &(v as f32), self.out);
                Ok(())
            }
            FieldType::Wrapper(ScalarType::Float) => encode_message(self.tag, self.out, |out| {
                encoding::float::encode(1, &(v as f32), out);
                Ok(())
            }),
            FieldType::Wrapper(ScalarType::Double) => encode_message(self.tag, self.out, |out| {
                encoding::double::encode(1, &v, out);
                Ok(())
            }),

            _ => Err(de::Error::invalid_type(de::Unexpected::Float(v), &self)),
        }
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match self.field_type {
            FieldType::Scalar(ScalarType::String) => {
                encoding::encode_key(self.tag, encoding::WireType::LengthDelimited, self.out);

                encoding::encode_varint(v.len() as u64, self.out);

                self.out.extend_from_slice(v.as_bytes());

                Ok(())
            }
            FieldType::Duration => encode_message(self.tag, self.out, |out| {
                // see https://github.com/protocolbuffers/protobuf/blob/85039a7438c3163ba4962d7359f0f1463b391975/src/google/protobuf/duration.proto#L102
                if !v.ends_with("s") {
                    return Err(de::Error::custom("Invalid duration string"));
                }

                let v = v.trim_end_matches('s');

                let Some((seconds, nanos)) = v.split_once('.') else {
                    return Err(de::Error::custom("Invalid duration string"));
                };

                let seconds = seconds
                    .parse::<i64>()
                    .map_err(|_| de::Error::custom("Invalid duration string"))?;

                let mut nanoseconds = 0i32;

                for (idx, char) in nanos.chars().take(8).enumerate() {
                    let Some(digit) = char.to_digit(10) else {
                        return Err(de::Error::custom("Invalid duration string"));
                    };

                    nanoseconds += digit as i32 * 10i32.pow(8 - idx as u32);
                }

                encoding::int64::encode(1, &seconds, out);
                encoding::int32::encode(2, &nanoseconds, out);

                Ok(())
            }),
            FieldType::FieldMask => encode_message(self.tag, self.out, |out| {
                encoding::encode_key(1, encoding::WireType::LengthDelimited, out);
                encoding::encode_varint(v.len() as u64, out);
                out.extend_from_slice(v.as_bytes());

                Ok(())
            }),
            FieldType::Timestamp => encode_message(self.tag, self.out, |out| {
                let ts = jiff::Timestamp::from_str(v)
                    .map_err(|err| de::Error::custom(format!("Invalid Timestamp string: {err}")))?;

                encoding::int64::encode(1, &ts.as_second(), out);
                encoding::int32::encode(2, &ts.subsec_nanosecond(), out);

                Ok(())
            }),
            FieldType::Wrapper(ScalarType::String) => encode_message(self.tag, self.out, |out| {
                encoding::encode_key(1, encoding::WireType::LengthDelimited, out);
                encoding::encode_varint(v.len() as u64, out);
                out.extend_from_slice(v.as_bytes());
                Ok(())
            }),

            FieldType::Scalar(ScalarType::Bytes) => {
                encoding::encode_key(self.tag, encoding::WireType::LengthDelimited, self.out);
                let decoded = base64::engine::general_purpose::URL_SAFE
                    .decode(v)
                    .or_else(|_| base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(v))
                    .or_else(|_| base64::engine::general_purpose::STANDARD.decode(v))
                    .or_else(|_| base64::engine::general_purpose::STANDARD_NO_PAD.decode(v))
                    .map_err(|err| de::Error::custom(format!("Invalid base64 string: {err}")))?;

                encoding::encode_varint(decoded.len() as u64, self.out);

                self.out.extend_from_slice(&decoded);

                Ok(())
            }
            FieldType::Wrapper(ScalarType::Bytes) => encode_message(self.tag, self.out, |out| {
                encoding::encode_key(1, encoding::WireType::LengthDelimited, out);

                let decoded = base64::engine::general_purpose::URL_SAFE
                    .decode(v)
                    .or_else(|_| base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(v))
                    .or_else(|_| base64::engine::general_purpose::STANDARD.decode(v))
                    .or_else(|_| base64::engine::general_purpose::STANDARD_NO_PAD.decode(v))
                    .map_err(|err| de::Error::custom(format!("Invalid base64 string: {err}")))?;

                encoding::encode_varint(decoded.len() as u64, out);

                out.extend_from_slice(&decoded);

                Ok(())
            }),

            _ => Err(de::Error::invalid_type(de::Unexpected::Str(v), &self)),
        }
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match self.field_type {
            FieldType::Scalar(ScalarType::Bytes) => {
                encoding::encode_key(self.tag, encoding::WireType::LengthDelimited, self.out);
                encoding::encode_varint(v.len() as u64, self.out);
                self.out.extend_from_slice(v);

                Ok(())
            }
            FieldType::Wrapper(ScalarType::Bytes) => encode_message(self.tag, self.out, |out| {
                encoding::encode_key(1, encoding::WireType::LengthDelimited, out);
                encoding::encode_varint(v.len() as u64, out);
                out.extend_from_slice(v);
                Ok(())
            }),

            _ => Err(de::Error::invalid_type(de::Unexpected::Bytes(v), &self)),
        }
    }

    fn visit_map<A>(self, _map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        encoding::encode_key(self.tag, encoding::WireType::LengthDelimited, self.out);
        encoding::encode_varint(0, self.out);
        Ok(())
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match self.field_type {
            FieldType::NullValue | FieldType::Wrapper(_) => encode_message(self.tag, self.out, |_| Ok(())),
            _ => Err(de::Error::invalid_type(de::Unexpected::Unit, &self)),
        }
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match self.field_type {
            FieldType::NullValue | FieldType::Wrapper(_) => encode_message(self.tag, self.out, |_| Ok(())),
            _ => Err(de::Error::invalid_type(de::Unexpected::Option, &self)),
        }
    }
}

fn encode_message<E>(tag: u32, out: &mut Vec<u8>, f: impl FnOnce(&mut Vec<u8>) -> Result<(), E>) -> Result<(), E> {
    let mut submessage = Vec::new();
    f(&mut submessage)?;

    encoding::encode_key(tag, encoding::WireType::LengthDelimited, out);
    encoding::encode_varint(submessage.len() as u64, out);
    out.extend_from_slice(&submessage);
    Ok(())
}
