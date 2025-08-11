use std::fmt;

use field_selection_map::{Path, PathSegment, SelectedObjectField, SelectedValue};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct SimpleIs {
    pub(crate) fields: Vec<SimpleIsField>,
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct SimpleIsField {
    pub(crate) input_field_name: String,
    pub(crate) output_field_name: String,
}

impl fmt::Display for SimpleIs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("{")?;

        for (i, field) in self.fields.iter().enumerate() {
            if i > 0 {
                f.write_str(",")?;
            }
            f.write_str(" ")?;
            f.write_str(&field.output_field_name)?;
            f.write_str(": ")?;
            f.write_str(&field.input_field_name)?;
        }

        if !self.fields.is_empty() {
            f.write_str(" ")?;
        }
        f.write_str("}")
    }
}

const UNACCEPTABLE_IS_ERROR: &str = r#"
Failed to parse `is:` argument to `derive`. The argument must be a string of the following shape: "{ target_field_name: source_field_name }".
"#;

pub(crate) fn extract_is(is: &str) -> anyhow::Result<SimpleIs> {
    let map = field_selection_map::SelectedValue::try_from(is)
        .map_err(|err| anyhow::anyhow!("Failed to parse as a field selection map: {err}"))?;
    let [field_selection_map::SelectedValueEntry::Object { path: None, object }] = map.alternatives.as_slice() else {
        anyhow::bail!(UNACCEPTABLE_IS_ERROR);
    };

    let mut fields = Vec::with_capacity(4);

    for field in &object.fields {
        let field = extract_field(field)?;

        fields.push(field);
    }

    Ok(SimpleIs { fields })
}

fn extract_field(
    field_selection_map::SelectedObjectField { key, value }: &SelectedObjectField,
) -> anyhow::Result<SimpleIsField> {
    let Some(SelectedValue { alternatives }) = value else {
        anyhow::bail!(UNACCEPTABLE_IS_ERROR);
    };

    let [field_selection_map::SelectedValueEntry::Path(Path { ty: None, segments })] = alternatives.as_slice() else {
        anyhow::bail!(UNACCEPTABLE_IS_ERROR);
    };

    let [PathSegment { ty: None, field }] = segments.as_slice() else {
        anyhow::bail!(UNACCEPTABLE_IS_ERROR);
    };

    Ok(SimpleIsField {
        input_field_name: (*field).to_owned(),
        output_field_name: (*key).to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_is_basic_success() {
        let is = r#"{ target: source }"#;
        let result = extract_is(is);
        insta::assert_debug_snapshot!(result, @r#"
        Ok(
            SimpleIs {
                fields: [
                    SimpleIsField {
                        input_field_name: "source",
                        output_field_name: "target",
                    },
                ],
            },
        )
        "#);
    }

    #[test]
    fn extract_is_with_underscores() {
        let is = r#"{ output_field: input_field }"#;
        let result = extract_is(is);
        insta::assert_debug_snapshot!(result, @r#"
        Ok(
            SimpleIs {
                fields: [
                    SimpleIsField {
                        input_field_name: "input_field",
                        output_field_name: "output_field",
                    },
                ],
            },
        )
        "#);
    }

    #[test]
    fn extract_is_with_camel_case() {
        let is = r#"{ targetFieldName: sourceFieldName }"#;
        let result = extract_is(is);
        insta::assert_debug_snapshot!(result, @r#"
        Ok(
            SimpleIs {
                fields: [
                    SimpleIsField {
                        input_field_name: "sourceFieldName",
                        output_field_name: "targetFieldName",
                    },
                ],
            },
        )
        "#);
    }

    #[test]
    fn extract_is_composite() {
        let is = r#"{ target1: source1 target2: source2 }"#;
        let result = extract_is(is);
        insta::assert_debug_snapshot!(result, @r#"
        Ok(
            SimpleIs {
                fields: [
                    SimpleIsField {
                        input_field_name: "source1",
                        output_field_name: "target1",
                    },
                    SimpleIsField {
                        input_field_name: "source2",
                        output_field_name: "target2",
                    },
                ],
            },
        )
        "#);
    }

    #[test]
    fn extract_is_fails_on_invalid_field_selection_map() {
        let is = r#"{ target: source"#; // Missing closing brace
        let result = extract_is(is);
        insta::assert_debug_snapshot!(result, @r#"
        Err(
            "Failed to parse as a field selection map: { target: source\n                ^\ninvalid object",
        )
        "#);
    }

    #[test]
    fn extract_is_fails_on_non_object() {
        let is = r#""not an object""#;
        let result = extract_is(is);
        insta::assert_debug_snapshot!(result, @r#"
        Err(
            "Failed to parse as a field selection map: \"not an object\"\n^\ninvalid name",
        )
        "#);
    }

    #[test]
    fn extract_is_fails_on_array() {
        let is = r#"[{ target: source }]"#;
        let result = extract_is(is);
        insta::assert_debug_snapshot!(result, @r#"
        Err(
            "\nFailed to parse `is:` argument to `derive`. The argument must be a string of the following shape: \"{ target_field_name: source_field_name }\".\n",
        )
        "#);
    }

    #[test]
    fn extract_is_fails_on_empty_object() {
        let is = r#"{}"#;
        let result = extract_is(is);
        insta::assert_debug_snapshot!(result, @r#"
        Ok(
            SimpleIs {
                fields: [],
            },
        )
        "#);
    }

    #[test]
    fn extract_is_fails_on_multiple_fields_with_commas() {
        let is = r#"{ target1: source1, target2: source2 }"#;
        let result = extract_is(is);
        insta::assert_debug_snapshot!(result, @r#"
        Err(
            "Failed to parse as a field selection map: { target1: source1, target2: source2 }\n                  ^\ninvalid object",
        )
        "#);
    }

    #[test]
    fn extract_is_fails_on_non_string_value() {
        let is = r#"{ target: 123 }"#;
        let result = extract_is(is);
        insta::assert_debug_snapshot!(result, @r#"
        Err(
            "Failed to parse as a field selection map: { target: 123 }\n        ^\ninvalid object",
        )
        "#);
    }

    #[test]
    fn extract_is_fails_on_object_value() {
        let is = r#"{ target: { nested: value } }"#;
        let result = extract_is(is);
        insta::assert_debug_snapshot!(result, @r#"
        Err(
            "\nFailed to parse `is:` argument to `derive`. The argument must be a string of the following shape: \"{ target_field_name: source_field_name }\".\n",
        )
        "#);
    }

    #[test]
    fn extract_is_fails_on_complex_path() {
        let is = r#"{ target: source.field }"#;
        let result = extract_is(is);
        insta::assert_debug_snapshot!(result, @r#"
        Err(
            "\nFailed to parse `is:` argument to `derive`. The argument must be a string of the following shape: \"{ target_field_name: source_field_name }\".\n",
        )
        "#);
    }

    #[test]
    fn extract_is_succeeds_on_null_keyword() {
        let is = r#"{ target: null }"#;
        let result = extract_is(is);
        insta::assert_debug_snapshot!(result, @r#"
        Ok(
            SimpleIs {
                fields: [
                    SimpleIsField {
                        input_field_name: "null",
                        output_field_name: "target",
                    },
                ],
            },
        )
        "#);
    }
}
