use super::*;
use crate::schema;
use bytes::Buf as _;
use prost::encoding;

pub(super) fn decode_value(
    proto: &mut bytes::Bytes,
    wire_type: encoding::WireType,
    field_type: Option<&schema::FieldType>,
    schema: &schema::Schema,
    ctx: encoding::DecodeContext,
) -> Result<Option<Value>, prost::DecodeError> {
    match (wire_type, field_type) {
        (encoding::WireType::Varint, Some(schema::FieldType::Scalar(schema::ScalarType::Int32))) => {
            let mut value = Default::default();
            encoding::int32::merge(wire_type, &mut value, proto, ctx.clone())?;
            Ok(Some(Value::SignedInt(value.into())))
        }
        (encoding::WireType::Varint, Some(schema::FieldType::Scalar(schema::ScalarType::Int64))) => {
            let mut value = Default::default();
            encoding::int64::merge(wire_type, &mut value, proto, ctx.clone())?;

            Ok(Some(Value::SignedInt(value)))
        }
        (encoding::WireType::Varint, Some(schema::FieldType::Scalar(schema::ScalarType::UInt32))) => {
            let mut value = Default::default();
            encoding::uint32::merge(wire_type, &mut value, proto, ctx.clone())?;

            Ok(Some(Value::UnsignedInt(value.into())))
        }
        (encoding::WireType::Varint, Some(schema::FieldType::Scalar(schema::ScalarType::UInt64))) => {
            let mut value = Default::default();
            encoding::uint64::merge(wire_type, &mut value, proto, ctx.clone())?;

            Ok(Some(Value::UnsignedInt(value)))
        }
        (encoding::WireType::Varint, Some(schema::FieldType::Scalar(schema::ScalarType::Sint32))) => {
            let mut value = Default::default();
            encoding::sint32::merge(wire_type, &mut value, proto, ctx.clone())?;

            Ok(Some(Value::SignedInt(value.into())))
        }
        (encoding::WireType::Varint, Some(schema::FieldType::Scalar(schema::ScalarType::Sint64))) => {
            let mut value = Default::default();
            encoding::sint64::merge(wire_type, &mut value, proto, ctx.clone())?;

            Ok(Some(Value::SignedInt(value)))
        }
        (encoding::WireType::Varint, Some(schema::FieldType::Scalar(schema::ScalarType::Bool))) => {
            let mut value = Default::default();
            encoding::bool::merge(wire_type, &mut value, proto, ctx.clone())?;

            Ok(Some(Value::Bool(value)))
        }
        (encoding::WireType::Varint, Some(schema::FieldType::Enum(enum_id))) => {
            let enum_definition = &schema[*enum_id];
            let value_number = encoding::decode_varint(proto)?;

            let Some(value_name) = enum_definition
                .values
                .iter()
                .find(|value| u64::from(value.number) == value_number)
                .map(|value| bytes::Bytes::from(value.name.clone()))
            else {
                return Ok(None);
            };

            Ok(Some(Value::String(value_name)))
        }

        (encoding::WireType::Varint, _) => {
            prost::encoding::decode_varint(proto)?;

            Ok(None)
        }

        (encoding::WireType::LengthDelimited, Some(schema::FieldType::Scalar(schema::ScalarType::Int32))) => {
            let mut values = Vec::<i32>::new();

            encoding::int32::merge_repeated(encoding::WireType::LengthDelimited, &mut values, proto, ctx)?;

            Ok(Some(Value::List(
                values.into_iter().map(|i| Value::SignedInt(i.into())).collect(),
            )))
        }
        (encoding::WireType::LengthDelimited, Some(schema::FieldType::Scalar(schema::ScalarType::Int64))) => {
            let mut values = Vec::<i64>::new();

            encoding::int64::merge_repeated(encoding::WireType::LengthDelimited, &mut values, proto, ctx)?;

            Ok(Some(Value::List(values.into_iter().map(Value::SignedInt).collect())))
        }
        (encoding::WireType::LengthDelimited, Some(schema::FieldType::Scalar(schema::ScalarType::UInt32))) => {
            let mut values = Vec::<u32>::new();

            encoding::uint32::merge_repeated(encoding::WireType::LengthDelimited, &mut values, proto, ctx)?;

            Ok(Some(Value::List(
                values.into_iter().map(|i| Value::UnsignedInt(i.into())).collect(),
            )))
        }
        (encoding::WireType::LengthDelimited, Some(schema::FieldType::Scalar(schema::ScalarType::UInt64))) => {
            let mut values = Vec::<u64>::new();

            encoding::uint64::merge_repeated(encoding::WireType::LengthDelimited, &mut values, proto, ctx)?;

            Ok(Some(Value::List(values.into_iter().map(Value::UnsignedInt).collect())))
        }
        (encoding::WireType::LengthDelimited, Some(schema::FieldType::Scalar(schema::ScalarType::Sint32))) => {
            let mut values = Vec::<i32>::new();

            encoding::sint32::merge_repeated(encoding::WireType::LengthDelimited, &mut values, proto, ctx)?;

            Ok(Some(Value::List(
                values.into_iter().map(|i| Value::SignedInt(i.into())).collect(),
            )))
        }
        (encoding::WireType::LengthDelimited, Some(schema::FieldType::Scalar(schema::ScalarType::Sint64))) => {
            let mut values = Vec::<i64>::new();

            encoding::sint64::merge_repeated(encoding::WireType::LengthDelimited, &mut values, proto, ctx)?;

            Ok(Some(Value::List(values.into_iter().map(Value::SignedInt).collect())))
        }
        (encoding::WireType::LengthDelimited, Some(schema::FieldType::Scalar(schema::ScalarType::Bool))) => {
            let mut values = Vec::<bool>::new();

            encoding::bool::merge_repeated(encoding::WireType::LengthDelimited, &mut values, proto, ctx)?;

            Ok(Some(Value::List(values.into_iter().map(Value::Bool).collect())))
        }
        (encoding::WireType::LengthDelimited, Some(schema::FieldType::Scalar(schema::ScalarType::Fixed32))) => {
            let mut values = Vec::<u32>::new();

            encoding::fixed32::merge_repeated(encoding::WireType::LengthDelimited, &mut values, proto, ctx)?;

            Ok(Some(Value::List(
                values.into_iter().map(|i| Value::UnsignedInt(i.into())).collect(),
            )))
        }
        (encoding::WireType::LengthDelimited, Some(schema::FieldType::Scalar(schema::ScalarType::Sfixed32))) => {
            let mut values = Vec::<i32>::new();

            encoding::sfixed32::merge_repeated(encoding::WireType::LengthDelimited, &mut values, proto, ctx)?;

            Ok(Some(Value::List(
                values.into_iter().map(|i| Value::SignedInt(i.into())).collect(),
            )))
        }
        (encoding::WireType::LengthDelimited, Some(schema::FieldType::Scalar(schema::ScalarType::Fixed64))) => {
            let mut values = Vec::<u64>::new();

            encoding::fixed64::merge_repeated(encoding::WireType::LengthDelimited, &mut values, proto, ctx)?;

            Ok(Some(Value::List(values.into_iter().map(Value::UnsignedInt).collect())))
        }
        (encoding::WireType::LengthDelimited, Some(schema::FieldType::Scalar(schema::ScalarType::Sfixed64))) => {
            let mut values = Vec::<i64>::new();

            encoding::sfixed64::merge_repeated(encoding::WireType::LengthDelimited, &mut values, proto, ctx)?;

            Ok(Some(Value::List(values.into_iter().map(Value::SignedInt).collect())))
        }
        (encoding::WireType::LengthDelimited, Some(schema::FieldType::Scalar(schema::ScalarType::Float))) => {
            let mut values = Vec::<f32>::new();

            encoding::float::merge_repeated(encoding::WireType::LengthDelimited, &mut values, proto, ctx)?;

            Ok(Some(Value::List(values.into_iter().map(Value::Float).collect())))
        }
        (encoding::WireType::LengthDelimited, Some(schema::FieldType::Scalar(schema::ScalarType::Double))) => {
            let mut values = Vec::<f64>::new();

            encoding::double::merge_repeated(encoding::WireType::LengthDelimited, &mut values, proto, ctx)?;

            Ok(Some(Value::List(values.into_iter().map(Value::Double).collect())))
        }

        (encoding::WireType::LengthDelimited, Some(schema::FieldType::Message(message_id))) => {
            let len = encoding::decode_length_delimiter(&mut *proto)?;
            let mut message_bytes = proto.copy_to_bytes(len);

            decode_message(&mut message_bytes, &schema[*message_id], schema, ctx).map(Some)
        }
        (encoding::WireType::LengthDelimited, Some(schema::FieldType::Scalar(schema::ScalarType::String))) => {
            let mut value = String::new();
            prost::encoding::string::merge(wire_type, &mut value, &mut *proto, ctx)?;

            Ok(Some(Value::String(value.into())))
        }
        (encoding::WireType::LengthDelimited, Some(schema::FieldType::Scalar(schema::ScalarType::Bytes))) => {
            let mut value = Vec::new();
            prost::encoding::bytes::merge(wire_type, &mut value, &mut *proto, ctx)?;

            Ok(Some(Value::Bytes(value.into())))
        }

        (encoding::WireType::LengthDelimited, Some(schema::FieldType::Map(types))) => {
            let len = encoding::decode_length_delimiter(&mut *proto)?;
            let mut map_entry_bytes = proto.copy_to_bytes(len);

            let (key_type, value_type) = types.as_ref();

            let mut key_value: [Option<Value>; 2] = Default::default();

            // We expect two fields. https://protobuf.dev/programming-guides/encoding/#maps
            for _ in 0..2 {
                let (tag, wire_type) = encoding::decode_key(&mut map_entry_bytes)?;

                match tag {
                    1 => {
                        key_value[0] =
                            decode_value(&mut map_entry_bytes, wire_type, Some(key_type), schema, ctx.clone())?
                    }
                    2 => {
                        key_value[1] =
                            decode_value(&mut map_entry_bytes, wire_type, Some(value_type), schema, ctx.clone())?
                    }
                    other => return Err(prost::DecodeError::new(format!("Tag {other} where a map was expected"))),
                }
            }

            let [Some(key), Some(value)] = key_value else {
                return Err(prost::DecodeError::new("Missing key or value in map entry"));
            };

            Ok(Some(Value::Map(vec![(key, value)])))
        }

        (encoding::WireType::LengthDelimited, Some(schema::FieldType::Duration | schema::FieldType::Timestamp)) => {
            let len = encoding::decode_length_delimiter(&mut *proto)?;
            let mut message_bytes = proto.copy_to_bytes(len);

            let mut seconds = None;
            let mut nanos = None;

            for _ in 0..2 {
                let (tag, wire_type) = encoding::decode_key(&mut message_bytes)?;

                match tag {
                    1 => {
                        seconds = decode_value(
                            &mut message_bytes,
                            wire_type,
                            Some(&schema::FieldType::Scalar(schema::ScalarType::Int64)),
                            schema,
                            ctx.clone(),
                        )?
                        .and_then(|v| v.as_signed_int())
                    }
                    2 => {
                        nanos = decode_value(
                            &mut message_bytes,
                            wire_type,
                            Some(&schema::FieldType::Scalar(schema::ScalarType::Int32)),
                            schema,
                            ctx.clone(),
                        )?
                        .and_then(|v| v.as_signed_int())
                    }
                    _ => (),
                }
            }

            let Some(seconds) = seconds else {
                return Ok(Some(Value::Null));
            };

            let nanos = nanos.unwrap_or_default();

            let string = match field_type {
                Some(schema::FieldType::Duration) => {
                    let mut duration = format!("{seconds}.{nanos:09}");
                    while duration.ends_with("0") {
                        duration.pop();
                    }
                    duration.push('s');

                    duration
                }
                Some(schema::FieldType::Timestamp) => {
                    let Ok(ts) = jiff::Timestamp::new(seconds, nanos as i32) else {
                        return Err(prost::DecodeError::new("Invalid timestamp".to_owned()));
                    };
                    ts.to_string()
                }
                _ => unreachable!(),
            };

            Ok(Some(Value::String(string.into())))
        }

        (encoding::WireType::LengthDelimited, Some(schema::FieldType::Empty)) => {
            let len = prost::encoding::decode_length_delimiter(&mut *proto)?;
            proto.advance(len);

            Ok(Some(Value::Map(vec![])))
        }

        (encoding::WireType::LengthDelimited, Some(schema::FieldType::NullValue)) => {
            let len = prost::encoding::decode_length_delimiter(&mut *proto)?;
            proto.advance(len);

            Ok(Some(Value::Null))
        }

        (encoding::WireType::LengthDelimited, Some(schema::FieldType::Wrapper(inner))) => {
            let len = prost::encoding::decode_length_delimiter(&mut *proto)?;
            let mut message = proto.copy_to_bytes(len);

            if message.is_empty() {
                return Ok(Some(Value::Null));
            }

            let (_, wire_type) = encoding::decode_key(&mut message)?;

            decode_value(
                &mut message,
                wire_type,
                Some(&schema::FieldType::Scalar(*inner)),
                schema,
                ctx,
            )
        }

        (encoding::WireType::LengthDelimited, Some(schema::FieldType::FieldMask)) => {
            let len = prost::encoding::decode_length_delimiter(&mut *proto)?;

            if len == 0 {
                return Ok(Some(Value::Null));
            }

            let mut message = proto.copy_to_bytes(len);
            if let (1, encoding::WireType::LengthDelimited) = encoding::decode_key(&mut message)? {
                let mut value = String::new();
                prost::encoding::string::merge(wire_type, &mut value, &mut message, ctx)?;

                Ok(Some(Value::String(value.into())))
            } else {
                Ok(Some(Value::Null))
            }
        }

        (encoding::WireType::LengthDelimited, _) => {
            let len = prost::encoding::decode_length_delimiter(&mut *proto)?;
            proto.advance(len);

            Ok(None)
        }

        (encoding::WireType::ThirtyTwoBit, Some(schema::FieldType::Scalar(schema::ScalarType::Fixed32))) => {
            let mut value = Default::default();
            prost::encoding::fixed32::merge(wire_type, &mut value, &mut *proto, ctx)?;

            Ok(Some(Value::UnsignedInt(value.into())))
        }
        (encoding::WireType::ThirtyTwoBit, Some(schema::FieldType::Scalar(schema::ScalarType::Sfixed32))) => {
            let mut value = Default::default();
            prost::encoding::sfixed32::merge(wire_type, &mut value, &mut *proto, ctx)?;

            Ok(Some(Value::SignedInt(value.into())))
        }

        (encoding::WireType::SixtyFourBit, Some(schema::FieldType::Scalar(schema::ScalarType::Fixed64))) => {
            let mut value = Default::default();
            prost::encoding::fixed64::merge(wire_type, &mut value, &mut *proto, ctx)?;

            Ok(Some(Value::UnsignedInt(value)))
        }
        (encoding::WireType::SixtyFourBit, Some(schema::FieldType::Scalar(schema::ScalarType::Sfixed64))) => {
            let mut value = Default::default();
            prost::encoding::sfixed64::merge(wire_type, &mut value, &mut *proto, ctx)?;

            Ok(Some(Value::SignedInt(value)))
        }

        (encoding::WireType::ThirtyTwoBit, Some(schema::FieldType::Scalar(schema::ScalarType::Float))) => {
            let mut value = Default::default();
            prost::encoding::float::merge(wire_type, &mut value, &mut *proto, ctx)?;

            Ok(Some(Value::Float(value)))
        }
        (encoding::WireType::SixtyFourBit, Some(schema::FieldType::Scalar(schema::ScalarType::Double))) => {
            let mut value = Default::default();
            prost::encoding::double::merge(wire_type, &mut value, &mut *proto, ctx)?;

            Ok(Some(Value::Double(value)))
        }

        (encoding::WireType::ThirtyTwoBit, _) => {
            proto.advance(4);

            Ok(None)
        }
        (encoding::WireType::SixtyFourBit, _) => {
            proto.advance(8);

            Ok(None)
        }

        (encoding::WireType::StartGroup | encoding::WireType::EndGroup, _) => todo!("startgroup/endgroup"),
    }
}
