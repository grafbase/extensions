use std::borrow::Cow;

use ramhorns::{Content, Section, encoding::Encoder, traits::ContentSequence};
use serde_json::Value;

use crate::TemplateEscaping;

pub(crate) struct JsonContent<'a> {
    pub value: Cow<'a, Value>,
    pub escaping: Option<TemplateEscaping>,
}

impl<'a> JsonContent<'a> {
    fn get<'s, 'b>(&'s self, name: &str) -> Option<JsonContent<'b>>
    where
        'a: 'b,
        's: 'b,
    {
        if name == "." {
            return Some(JsonContent {
                value: Cow::Borrowed(self.value.as_ref()),
                escaping: self.escaping,
            });
        }
        name.split('.')
            .try_fold(self.value.as_ref(), |parent, key| {
                parent.as_object().and_then(|obj| obj.get(key))
            })
            .map(|value| JsonContent {
                value: Cow::Borrowed(value),
                escaping: self.escaping,
            })
    }
}

fn urlencode(s: &str) -> impl std::fmt::Display + '_ {
    use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};

    // Urlencode char encoding set. Only the characters in the unreserved set don't
    // have any special purpose in any part of a URI and can be safely left
    // unencoded as specified in https://tools.ietf.org/html/rfc3986.html#section-2.3
    const URLENCODE_STRICT_SET: &percent_encoding::AsciiSet =
        &NON_ALPHANUMERIC.remove(b'_').remove(b'.').remove(b'-').remove(b'~');

    utf8_percent_encode(s, URLENCODE_STRICT_SET)
}

impl Content for JsonContent<'_> {
    fn is_truthy(&self) -> bool {
        match self.value.as_ref() {
            Value::Null => false,
            Value::Bool(b) => b.is_truthy(),
            Value::Number(n) => match n.as_i128() {
                Some(i) => i.is_truthy(),
                None => match n.as_f64() {
                    Some(f) => f.is_truthy(),
                    None => false,
                },
            },
            Value::String(s) => s.is_truthy(),
            Value::Array(a) => !a.is_empty(),
            Value::Object(o) => !o.is_empty(),
        }
    }

    fn capacity_hint(&self, _tpl: &ramhorns::Template<'_>) -> usize {
        match self.value.as_ref() {
            Value::Null => 4,
            Value::Bool(_) => 5,
            Value::Number(n) => {
                let n = n.as_f64().unwrap();
                if n.is_finite() { 24 } else { 64 }
            }
            Value::String(s) => s.len(),
            Value::Array(v) => v.len() * 2,
            Value::Object(o) => o.len() * 2,
        }
    }

    fn render_escaped<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        match self.value.as_ref() {
            Value::Null => encoder.write_unescaped("null"),
            Value::Bool(b) => encoder.write_unescaped(if *b { "true" } else { "false" }),
            Value::Number(n) => encoder.format_unescaped(n),
            Value::String(s) => match self.escaping {
                Some(TemplateEscaping::Json) => {
                    let s = serde_json::to_string(s).unwrap();
                    encoder.write_unescaped(&s)
                }
                Some(TemplateEscaping::Url) => {
                    encoder.format_unescaped(urlencode(s))?;
                    Ok(())
                }
                _ => encoder.write_unescaped(s),
            },
            Value::Array(a) => match self.escaping {
                Some(TemplateEscaping::Url) => {
                    let s = serde_json::to_string(a).unwrap();
                    encoder.format_unescaped(urlencode(&s))?;
                    Ok(())
                }
                Some(TemplateEscaping::Json) => encoder.write_unescaped(&serde_json::to_string(a).unwrap()),
                None => Ok(()),
            },
            Value::Object(o) => match self.escaping {
                Some(TemplateEscaping::Url) => {
                    let s = serde_json::to_string(o).unwrap();
                    encoder.format_unescaped(urlencode(&s))?;
                    Ok(())
                }
                Some(TemplateEscaping::Json) => encoder.write_unescaped(&serde_json::to_string(o).unwrap()),
                None => Ok(()),
            },
        }
    }

    fn render_unescaped<E: Encoder>(&self, encoder: &mut E) -> Result<(), E::Error> {
        match self.value.as_ref() {
            Value::Null => encoder.write_unescaped("null"),
            Value::Bool(b) => encoder.write_unescaped(if *b { "true" } else { "false" }),
            Value::Number(n) => encoder.format_unescaped(n),
            Value::String(s) => encoder.write_unescaped(s),
            Value::Array(a) => encoder.write_unescaped(&serde_json::to_string(a).unwrap()),
            Value::Object(o) => encoder.write_unescaped(&serde_json::to_string(o).unwrap()),
        }
    }

    fn render_section<C, E>(&self, section: Section<'_, C>, encoder: &mut E) -> Result<(), E::Error>
    where
        C: ContentSequence,
        E: Encoder,
    {
        match self.value.as_ref() {
            Value::Array(list) => ramhorns::render_indexed_content_section(
                list.iter().map(|value| JsonContent {
                    value: Cow::Borrowed(value),
                    escaping: self.escaping,
                }),
                section,
                encoder,
            ),
            Value::Object(o) => {
                // if is_truthy() is false
                if o.is_empty() {
                    Ok(())
                } else {
                    section.with(self).render(encoder)
                }
            }
            _ => section.render(encoder),
        }
    }

    fn render_field_escaped<E>(&self, _: u64, name: &str, encoder: &mut E) -> Result<bool, E::Error>
    where
        E: Encoder,
    {
        match self.get(name) {
            Some(v) => v.render_escaped(encoder).map(|_| true),
            None => Ok(false),
        }
    }

    fn render_field_unescaped<E>(&self, _: u64, name: &str, encoder: &mut E) -> Result<bool, E::Error>
    where
        E: Encoder,
    {
        match self.get(name) {
            Some(v) => v.render_unescaped(encoder).map(|_| true),
            None => Ok(false),
        }
    }

    fn render_field_section<C, E>(
        &self,
        _: u64,
        name: &str,
        section: Section<'_, C>,
        encoder: &mut E,
    ) -> Result<bool, E::Error>
    where
        C: ContentSequence,
        E: Encoder,
    {
        match self.get(name) {
            Some(v) => v.render_section(section, encoder).map(|_| true),
            None => Ok(false),
        }
    }
}
