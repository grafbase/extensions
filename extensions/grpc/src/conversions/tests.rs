use super::MessageDeserialize;
use crate::{directives, schema};
use serde::de::DeserializeSeed as _;

#[test]
fn roundtrip() {
    let schema = schema::Schema::new(
        vec![],
        vec![directives::ProtoMessageDefinition {
            name: "TestMessage".to_owned(),
            fields: vec![
                directives::ProtoField {
                    name: "name".to_owned(),
                    r#type: "string".to_owned(),
                    repeated: false,
                    number: 3,
                },
                directives::ProtoField {
                    name: "gpgPublicKey".to_owned(),
                    r#type: "bytes".to_owned(),
                    repeated: false,
                    number: 4,
                },
                directives::ProtoField {
                    name: "numberInt32".to_owned(),
                    r#type: "int32".to_owned(),
                    repeated: false,
                    number: 7,
                },
                directives::ProtoField {
                    name: "numberInt64".to_owned(),
                    r#type: "int64".to_owned(),
                    repeated: false,
                    number: 8,
                },
                directives::ProtoField {
                    name: "friends".to_owned(),
                    r#type: "TestMessage".to_owned(),
                    repeated: true,
                    number: 9,
                },
                directives::ProtoField {
                    name: "contacts".to_owned(),
                    r#type: "map<string, TestMessage>".to_owned(),
                    repeated: false,
                    number: 29,
                },
                directives::ProtoField {
                    name: "numberSInt32".to_owned(),
                    r#type: "sint32".to_owned(),
                    repeated: false,
                    number: 10,
                },
                directives::ProtoField {
                    name: "numberSInt64".to_owned(),
                    r#type: "sint64".to_owned(),
                    repeated: false,
                    number: 12,
                },
                directives::ProtoField {
                    name: "numberSFixed32".to_owned(),
                    r#type: "sfixed32".to_owned(),
                    repeated: true,
                    number: 13,
                },
                directives::ProtoField {
                    name: "numberSFixed64".to_owned(),
                    r#type: "sfixed64".to_owned(),
                    repeated: true,
                    number: 14,
                },
                directives::ProtoField {
                    name: "numberFixed32".to_owned(),
                    r#type: "fixed32".to_owned(),
                    repeated: true,
                    number: 15,
                },
                directives::ProtoField {
                    name: "numberFixed64".to_owned(),
                    r#type: "fixed64".to_owned(),
                    repeated: true,
                    number: 16,
                },
                directives::ProtoField {
                    name: "favoriteColor".to_owned(),
                    r#type: "Color".to_owned(),
                    repeated: false,
                    number: 17,
                },
                directives::ProtoField {
                    name: "nullValue".to_owned(),
                    r#type: ".google.protobuf.NullValue".to_owned(),
                    repeated: false,
                    number: 18,
                },
                directives::ProtoField {
                    name: "boolValue".to_owned(),
                    r#type: ".google.protobuf.BoolValue".to_owned(),
                    repeated: false,
                    number: 19,
                },
                directives::ProtoField {
                    name: "stringValue".to_owned(),
                    r#type: ".google.protobuf.StringValue".to_owned(),
                    repeated: false,
                    number: 20,
                },
                directives::ProtoField {
                    name: "bytesValue".to_owned(),
                    r#type: ".google.protobuf.BytesValue".to_owned(),
                    repeated: false,
                    number: 21,
                },
                directives::ProtoField {
                    name: "int32Value".to_owned(),
                    r#type: ".google.protobuf.Int32Value".to_owned(),
                    repeated: false,
                    number: 22,
                },
                directives::ProtoField {
                    name: "int64Value".to_owned(),
                    r#type: ".google.protobuf.Int64Value".to_owned(),
                    repeated: false,
                    number: 23,
                },
                directives::ProtoField {
                    name: "uint32Value".to_owned(),
                    r#type: ".google.protobuf.UInt32Value".to_owned(),
                    repeated: false,
                    number: 24,
                },
                directives::ProtoField {
                    name: "uint64Value".to_owned(),
                    r#type: ".google.protobuf.UInt64Value".to_owned(),
                    repeated: false,
                    number: 25,
                },
                directives::ProtoField {
                    name: "floatValue".to_owned(),
                    r#type: ".google.protobuf.FloatValue".to_owned(),
                    repeated: false,
                    number: 26,
                },
                directives::ProtoField {
                    name: "doubleValue".to_owned(),
                    r#type: ".google.protobuf.DoubleValue".to_owned(),
                    repeated: false,
                    number: 27,
                },
                directives::ProtoField {
                    name: "duration".to_owned(),
                    r#type: ".google.protobuf.Duration".to_owned(),
                    repeated: false,
                    number: 30,
                },
                directives::ProtoField {
                    name: "timestamp".to_owned(),
                    r#type: ".google.protobuf.Timestamp".to_owned(),
                    repeated: false,
                    number: 31,
                },
                directives::ProtoField {
                    name: "fieldMask".to_owned(),
                    r#type: ".google.protobuf.FieldMask".to_owned(),
                    repeated: false,
                    number: 32,
                },
                directives::ProtoField {
                    name: "empty".to_owned(),
                    r#type: ".google.protobuf.Empty".to_owned(),
                    repeated: false,
                    number: 33,
                },
            ],
        }],
        vec![
            directives::ProtoEnumDefinition {
                name: "Temperature".to_owned(),
                values: vec![
                    directives::ProtoEnumValueDefinition {
                        name: "LUKEWARM".to_owned(),
                        number: 0,
                    },
                    directives::ProtoEnumValueDefinition {
                        name: "HOT".to_owned(),
                        number: 1,
                    },
                    directives::ProtoEnumValueDefinition {
                        name: "COLD".to_owned(),
                        number: 2,
                    },
                    directives::ProtoEnumValueDefinition {
                        name: "FREEZING".to_owned(),
                        number: 3,
                    },
                ],
            },
            directives::ProtoEnumDefinition {
                name: "Color".to_owned(),
                values: vec![
                    directives::ProtoEnumValueDefinition {
                        name: "RED".to_owned(),
                        number: 0,
                    },
                    directives::ProtoEnumValueDefinition {
                        name: "GREEN".to_owned(),
                        number: 1,
                    },
                    directives::ProtoEnumValueDefinition {
                        name: "BLUE".to_owned(),
                        number: 2,
                    },
                    directives::ProtoEnumValueDefinition {
                        name: "YELLOW".to_owned(),
                        number: 3,
                    },
                ],
            },
        ],
    )
    .unwrap();
    let message = schema.get_message("TestMessage").unwrap();

    let input = r#"
{
  "name": "george george",
  "gpgPublicKey": "bG9sCg==",
  "numberInt32": 24601,
  "nullValue": null,
  "boolValue": true,
  "stringValue": "abcd",
  "bytesValue": "a2VrZWtla2UK",
  "int32Value": -16,
  "int64Value": -16192843734,
  "uint32Value": 16,
  "uint64Value": 16192843734124,
  "floatValue": 3.14,
  "doubleValue": -3.14,
  "duration": "1.03002s",
  "timestamp": "1972-01-01T10:00:20.021Z",
  "fieldMask": "f.fooBar,h",
  "empty": {},
  "friends": [
    {
      "name": "alice",
      "numberSInt32": 24605,
      "numberSInt64": -24606,
      "numberInt64": 24607,
      "doubleValue": null,
      "contacts": {
        "fred": {
          "favoriteColor": "BLUE"
        },
        "jane": {
          "favoriteColor": "RED"
        }
      }
    },
    {
      "name": "bob",
      "numberFixed32": [
        1,
        2,
        3
      ],
      "numberFixed64": [
        4,
        5,
        6
      ],
      "numberSFixed32": [
        7,
        8,
        -9
      ],
      "numberSFixed64": [
        10,
        11,
        12
      ]
    }
  ]
}
        "#;

    let mut out = Vec::new();
    let mut deserializer = serde_json::Deserializer::from_str(input);

    let _: serde_json::Value = serde_json::from_str(input).unwrap();

    MessageDeserialize {
        schema: &schema,
        message_definition: message,
        out: &mut out,
    }
    .deserialize(&mut deserializer)
    .unwrap();

    let bytes = bytes::Bytes::from(out);

    let serialized =
        serde_json::to_string_pretty(&super::output::MessageSerialize::new(&bytes, message, &schema)).unwrap();

    pretty_assertions::assert_eq!(input.trim(), serialized);
}

#[test]
fn packed_repeated_fields() {
    // From docs example: https://protobuf.dev/programming-guides/encoding/#packed.
    let message = b"\x32\x06\x03\x8e\x02\x9e\xa7\x05";

    let schema = schema::Schema::new(
        vec![],
        vec![directives::ProtoMessageDefinition {
            name: "Test5".to_owned(),
            fields: vec![directives::ProtoField {
                name: "f".to_owned(),
                number: 6,
                repeated: true,
                r#type: "int32".to_owned(),
            }],
        }],
        vec![],
    )
    .unwrap();

    let bytes = bytes::Bytes::from_static(message);
    let message = schema.get_message("Test5").unwrap();

    let serialized =
        serde_json::to_string_pretty(&super::output::MessageSerialize::new(&bytes, message, &schema)).unwrap();

    insta::assert_snapshot!(serialized);
}
