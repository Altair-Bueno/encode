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
use encode::combinators::Separated;
use encode::Encodable;
use encode::EncodableSize;
use encode::Encoder;
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

impl<S: AsRef<str>, E: Encoder> Encodable<E> for JsonString<S> {
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

impl<E: Encoder> Encodable<E> for Json {
    type Error = E::Error;

    #[inline]
    fn encode(&self, encoder: &mut E) -> Result<(), Self::Error> {
        match self {
            // Strings and characters can be encoded as is.
            Json::Null => "null".encode(encoder),
            Json::Bool(true) => "true".encode(encoder),
            Json::Bool(false) => "false".encode(encoder),
            // We use format_args! for numbers to avoid the overhead of allocating a string.
            Json::Number(n) => format_args!("{n}").encode(encoder),
            // We use the custom `JsonString` combinator to scape the string according to the JSON spec.
            Json::String(s) => JsonString(s).encode(encoder),
            // We use the `Separated` combinator to encode the iterator (&Vec<Json>) as a JSON array.
            Json::Array(a) => ('[', Separated::new(a, ','), ']').encode(encoder),
            // Notice how we can use tuples to add the bracket prefix and suffix to the object.
            Json::Object(o) => (
                '{',
                Separated::new(o.iter().map(|(k, v)| (JsonString(k), ':', v)), ','),
                '}',
            )
                .encode(encoder),
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
    ]));
    // We can use the SizeEncoder to calculate the size of the final JSON string.
    let size = json.encoded_size()?;
    println!("Expected JSON size: {}", size);

    // We can also encode the JSON string into a buffer, like a Vec<u8> or &mut [u8].
    let mut buf = Vec::with_capacity(size);
    json.encode(&mut buf)?;
    let s = String::from_utf8(buf).expect("Our encoder always produces valid UTF-8");

    println!("{s}");
    assert_eq!(
        s.len(),
        size,
        "The size of the encoded JSON string should match the calculated size"
    );
    Ok(())
}

#[cfg(test)]
mod test {
    //! Tests for the JSON encoder. Do not include object tests as the order of
    //! the keys is not guaranteed for [`HashMap`].
    use super::*;

    #[test]
    fn assert_booleans_are_encoded_correctly() {
        let mut buf = Vec::new();
        Json::Bool(true).encode(&mut buf).unwrap();
        assert_eq!(&buf, b"true");
        buf.clear();
        Json::Bool(false).encode(&mut buf).unwrap();
        assert_eq!(&buf, b"false");
    }

    #[test]
    fn assert_numbers_are_encoded_correctly() {
        let mut buf = Vec::new();
        Json::Number(42.0).encode(&mut buf).unwrap();
        assert_eq!(&buf, b"42");
    }

    #[test]
    fn assert_strings_are_encoded_correctly() {
        let mut buf = Vec::new();
        Json::String("Hello, World!".into())
            .encode(&mut buf)
            .unwrap();
        let s = String::from_utf8(buf).unwrap();
        assert_eq!(s, "\"Hello, World!\"");
    }

    #[test]
    fn assert_arrays_are_encoded_correctly() {
        let mut buf = Vec::new();
        Json::Array(vec![
            Json::Number(1.0),
            Json::Number(2.0),
            Json::Number(3.0),
        ])
        .encode(&mut buf)
        .unwrap();
        assert_eq!(&buf, b"[1,2,3]");
    }

    #[test]
    fn assert_empty_arrays_are_encoded_correctly() {
        let mut buf = Vec::new();
        Json::Array(vec![]).encode(&mut buf).unwrap();
        assert_eq!(&buf, b"[]");
    }

    #[test]
    fn assert_objects_are_encoded_correctly() {
        let mut buf = Vec::new();
        Json::Object(HashMap::from([(
            "name".into(),
            Json::String("John Doe".into()),
        )]))
        .encode(&mut buf)
        .unwrap();
        assert_eq!(&buf, b"{\"name\":\"John Doe\"}");
    }

    #[test]
    fn assert_empty_objects_are_encoded_correctly() {
        let mut buf = Vec::new();
        Json::Object(HashMap::new()).encode(&mut buf).unwrap();
        assert_eq!(&buf, b"{}");
    }

    #[test]
    fn assert_nulls_are_encoded_correctly() {
        let mut buf = Vec::new();
        Json::Null.encode(&mut buf).unwrap();
        assert_eq!(&buf, b"null");
    }
}
