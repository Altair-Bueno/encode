//! A [JSON](https://www.json.org/json-en.html) encoder
//!
//! This example demonstrates how to implement [`Encodable`] using combinators
//! to simplify the implementation of encoders.
//!
//! Run the example with:
//!
//! ```sh
//! cargo run --example json
//! ```
use core::iter::repeat;
use encode::combinators::Iter;
use encode::combinators::Separated;
use encode::Encodable;
use encode::StrEncoder;
use std::collections::HashMap;

/// Our JSON data type.
#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
}

/// A helper combinator to encode a string as a JSON string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonString<S>(pub S);

impl<S: AsRef<str>, E: StrEncoder> Encodable<E> for JsonString<S> {
    type Error = E::Error;

    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        let s = self.0.as_ref();
        '"'.encode(encoder)?;
        for c in s.chars() {
            match c {
                '"' => r#"\""#.encode(encoder)?,
                '\\' => r#"\\"#.encode(encoder)?,
                '\x08' => r#"\b"#.encode(encoder)?,
                '\x0c' => r#"\f"#.encode(encoder)?,
                '\n' => r#"\n"#.encode(encoder)?,
                '\r' => r#"\r"#.encode(encoder)?,
                '\t' => r#"\t"#.encode(encoder)?,
                c if c.is_control() => format_args!("\\u{:04x}", c as u32).encode(encoder)?,
                c => c.encode(encoder)?,
            }
        }
        '"'.encode(encoder)
    }
}

/// Encodes JSON without any whitespace (compact format).
pub struct CompactJson<'a>(pub &'a Json);

impl<'a> CompactJson<'a> {
    pub fn new(json: &'a Json) -> Self {
        CompactJson(json)
    }
}

impl<'a, E: StrEncoder> Encodable<E> for CompactJson<'a> {
    type Error = E::Error;

    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        match self.0 {
            Json::Null => "null".encode(encoder),
            Json::Bool(true) => "true".encode(encoder),
            Json::Bool(false) => "false".encode(encoder),
            Json::Number(n) => {
                // JSON does not permit NaN or +/-infinity numeric literals.
                // Encode non-finite floating-point values as `null` so the
                // output remains valid JSON.
                if n.is_finite() {
                    format_args!("{n}").encode(encoder)
                } else {
                    "null".encode(encoder)
                }
            }
            Json::String(s) => JsonString(s).encode(encoder),
            // We use the `Separated` combinator to encode the iterator as a JSON array.
            Json::Array(a) => (
                '[',
                Separated::new(a.iter().map(CompactJson::new), ','),
                ']',
            )
                .encode(encoder),
            // Notice how we can use tuples to add the bracket prefix and suffix to the object.
            Json::Object(o) => (
                '{',
                Separated::new(
                    o.iter()
                        .map(|(k, v)| (JsonString(k), ':', CompactJson::new(v))),
                    ',',
                ),
                '}',
            )
                .encode(encoder),
        }
    }
}

/// Encodes JSON with configurable indentation (pretty format).
///
/// The `depth` field tracks the current nesting level; start at `0` via [`PrettyJson::new`].
/// The `indent` field controls what string is repeated per level (default: `"  "` for 2 spaces).
/// Set it to `"\t"` for tabs or `"    "` for 4-space indentation.
pub struct PrettyJson<'j, 'i> {
    pub json: &'j Json,
    pub depth: usize,
    pub indent: &'i str,
}

impl<'j, 'i> PrettyJson<'j, 'i> {
    pub fn new(json: &'j Json, indent: &'i str) -> Self {
        PrettyJson {
            json,
            depth: 0,
            indent,
        }
    }
}

impl<'j, 'i, E: StrEncoder> Encodable<E> for PrettyJson<'j, 'i> {
    type Error = E::Error;

    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        match self.json {
            Json::Array(a) if a.is_empty() => "[]".encode(encoder),
            Json::Object(o) if o.is_empty() => "{}".encode(encoder),
            Json::Array(a) => (
                "[\n",
                Separated::new(
                    a.iter().map(|json| {
                        (
                            Iter::new(repeat(self.indent).take(self.depth + 1)),
                            PrettyJson {
                                json,
                                depth: self.depth + 1,
                                indent: self.indent,
                            },
                        )
                    }),
                    ",\n",
                ),
                '\n',
                Iter::new(repeat(self.indent).take(self.depth)),
                ']',
            )
                .encode(encoder),
            // Notice how we can use tuples to add the bracket prefix and suffix to the object.
            Json::Object(o) => (
                "{\n",
                Separated::new(
                    o.iter().map(|(k, json)| {
                        (
                            Iter::new(repeat(self.indent).take(self.depth + 1)),
                            JsonString(k),
                            ": ",
                            PrettyJson {
                                json,
                                depth: self.depth + 1,
                                indent: self.indent,
                            },
                        )
                    }),
                    ",\n",
                ),
                '\n',
                Iter::new(repeat(self.indent).take(self.depth)),
                '}',
            )
                .encode(encoder),
            // Fall back to the compact encoder for these simpler values
            _ => CompactJson::new(self.json).encode(encoder),
        }
    }
}

// Notice how we can use our encoder to implement traits such as `Display`.
impl std::fmt::Display for Json {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            PrettyJson::new(self, "  ").encode(f)
        } else {
            CompactJson::new(self).encode(f)
        }
    }
}

fn main() -> Result<(), Box<dyn core::error::Error>> {
    let json = Json::Object(HashMap::from([
        ("name".into(), Json::String("John Doe".into())),
        ("age".into(), Json::Number(42.0)),
        ("is_student".into(), Json::Bool(false)),
        (
            "grades".into(),
            Json::Array(vec![
                Json::Number(100.0),
                Json::Number(90.0),
                Json::Number(80.0),
            ]),
        ),
        ("weird \n string\\\0".into(), Json::Null),
        (
            "nested".into(),
            Json::Array(vec![
                Json::Array(vec![]),
                Json::Array(vec![Json::Object(HashMap::from([
                    ("key".into(), Json::Null),
                    ("another".into(), Json::Array(vec![Json::Null])),
                ]))]),
                Json::Array(vec![]),
            ]),
        ),
    ]));
    println!("{:=^80}", "Compact");
    println!("{json}");
    println!("{:=^80}", "Pretty");
    println!("{json:#}");
    Ok(())
}

#[cfg(test)]
mod test {
    //! Tests for the JSON encoder. Do not include object tests as the order of
    //! the keys is not guaranteed for [`HashMap`].
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::enabled(Json::Bool(true), b"true" as &[u8])]
    #[case::disabled(Json::Bool(false), b"false")]
    fn assert_booleans_are_encoded_correctly(#[case] json: Json, #[case] expected: &[u8]) {
        let mut buf = Vec::new();
        CompactJson::new(&json).encode(&mut buf).unwrap();
        assert_eq!(&buf, expected);
    }

    #[test]
    fn assert_numbers_are_encoded_correctly() {
        let mut buf = Vec::new();
        CompactJson::new(&Json::Number(42.0))
            .encode(&mut buf)
            .unwrap();
        assert_eq!(&buf, b"42");
    }

    #[test]
    fn assert_strings_are_encoded_correctly() {
        let mut buf = Vec::new();
        CompactJson::new(&Json::String("Hello, World!".into()))
            .encode(&mut buf)
            .unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert_eq!(s, "\"Hello, World!\"");
    }

    #[rstest]
    #[case::non_empty(
        Json::Array(vec![Json::Number(1.0), Json::Number(2.0), Json::Number(3.0)]),
        b"[1,2,3]" as &[u8]
    )]
    #[case::empty(Json::Array(vec![]), b"[]")]
    fn assert_arrays_are_encoded_correctly(#[case] json: Json, #[case] expected: &[u8]) {
        let mut buf = Vec::new();
        CompactJson::new(&json).encode(&mut buf).unwrap();
        assert_eq!(&buf, expected);
    }

    #[rstest]
    #[case::non_empty(
        Json::Object(HashMap::from([("name".into(), Json::String("John Doe".into()))])),
        b"{\"name\":\"John Doe\"}" as &[u8]
    )]
    #[case::empty(Json::Object(HashMap::new()), b"{}")]
    fn assert_objects_are_encoded_correctly(#[case] json: Json, #[case] expected: &[u8]) {
        let mut buf = Vec::new();
        CompactJson::new(&json).encode(&mut buf).unwrap();
        assert_eq!(&buf, expected);
    }

    #[test]
    fn assert_nulls_are_encoded_correctly() {
        let mut buf = Vec::new();
        CompactJson::new(&Json::Null).encode(&mut buf).unwrap();
        assert_eq!(&buf, b"null");
    }

    #[test]
    fn assert_compact_display_uses_no_whitespace() {
        let json = Json::Array(vec![Json::Number(1.0), Json::Number(2.0)]);
        assert_eq!(format!("{json}"), "[1,2]");
    }

    #[test]
    fn assert_pretty_empty_array() {
        assert_eq!(format!("{:#}", Json::Array(vec![])), "[]");
    }

    #[test]
    fn assert_pretty_array() {
        let json = Json::Array(vec![Json::Number(1.0), Json::Number(2.0)]);
        assert_eq!(format!("{json:#}"), "[\n  1,\n  2\n]");
    }

    #[test]
    fn assert_pretty_nested_array() {
        let json = Json::Array(vec![Json::Array(vec![Json::Number(1.0)])]);
        assert_eq!(format!("{json:#}"), "[\n  [\n    1\n  ]\n]");
    }

    #[test]
    fn assert_pretty_empty_object() {
        assert_eq!(format!("{:#}", Json::Object(HashMap::new())), "{}");
    }

    #[test]
    fn assert_pretty_object() {
        let json = Json::Object(HashMap::from([("x".into(), Json::Number(1.0))]));
        assert_eq!(format!("{json:#}"), "{\n  \"x\": 1\n}");
    }

    #[test]
    fn assert_pretty_tabs() {
        let json = Json::Array(vec![Json::Number(1.0), Json::Number(2.0)]);
        let mut buf = String::new();
        PrettyJson {
            json: &json,
            depth: 0,
            indent: "\t",
        }
        .encode(&mut buf)
        .unwrap();
        assert_eq!(buf, "[\n\t1,\n\t2\n]");
    }

    #[test]
    fn assert_pretty_four_spaces() {
        let json = Json::Array(vec![Json::Number(1.0), Json::Number(2.0)]);
        let mut buf = String::new();
        PrettyJson {
            json: &json,
            depth: 0,
            indent: "    ",
        }
        .encode(&mut buf)
        .unwrap();
        assert_eq!(buf, "[\n    1,\n    2\n]");
    }
}
