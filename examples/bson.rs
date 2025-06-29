//! A [BSON](https://bsonspec.org/spec.html) encoder
//!
//! This example demonstrates how to implement [`Encodable`] with custom
//! errors, and how combinators can be used to simplify the implementation of
//! encoders.
//!
//! Run the example with:
//!
//! ```sh
//! cargo run --example bson
//! ```

use core::convert::Infallible;
use core::num::TryFromIntError;

use encode::combinators::{FromError, Iter, LengthPrefix, LE};
use encode::encoders::InsufficientSpace;
use encode::{Encodable, EncodableSize};

/// A BSON encoding error.
///
/// This error type is used when encoding BSON fails. Because BSON has a maximum
/// size of 2^31 - 1 bytes, we need to handle the case where the encoded BSON
/// document is too large to fit in a 32-bit signed integer. Additionally, we
/// need to handle the case where there is not enough space in the buffer to
/// encode the BSON document, as well as any other errors that may occur during
/// encoding.
#[derive(Debug, Clone, PartialEq)]
pub enum BsonError {
    TooLarge(TryFromIntError),
    InsufficientSpace(InsufficientSpace),
}

impl core::error::Error for BsonError {}
impl core::fmt::Display for BsonError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            BsonError::TooLarge(_) => write!(f, "failed to encode BSON because it is too large"),
            BsonError::InsufficientSpace(err) => core::fmt::Display::fmt(err, f),
        }
    }
}
impl From<TryFromIntError> for BsonError {
    fn from(err: TryFromIntError) -> Self {
        BsonError::TooLarge(err)
    }
}
impl From<InsufficientSpace> for BsonError {
    fn from(err: InsufficientSpace) -> Self {
        BsonError::InsufficientSpace(err)
    }
}
impl From<Infallible> for BsonError {
    fn from(_: Infallible) -> Self {
        unreachable!("infallible cannot be constructed")
    }
}

/// A BSON document.
#[derive(Debug, Clone, PartialEq)]
pub struct BsonDocument {
    /// The elements of the BSON document.
    pub e_list: Vec<BsonElement>,
}

impl<Encoder> Encodable<Encoder> for BsonDocument
where
    Encoder: encode::ByteEncoder,
    BsonError: From<Encoder::Error>,
{
    type Error = BsonError;

    fn encode(&self, encoder: &mut Encoder) -> Result<(), Self::Error> {
        let document = (
            Iter::new(&self.e_list),
            // u8 uses `BaseEncoder::Error`, so we need to convert it to our BsonError
            // for our combinators to work.
            FromError::<_, Self::Error>::new(0u8),
        );
        // We cannot use LengthPrefix here because we need to encode the size of
        // the document including the size field itself.
        let size = document.encoded_size()? + (i32::BITS / 8) as usize;
        LE::<i32>::try_from(size)?.encode(encoder)?;
        document.encode(encoder)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BsonElement {
    /// The name of the BSON element.
    pub e_value: String,
    /// The variant of the BSON element.
    pub variant: BsonElementVariant,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BsonElementVariant {
    Double(f64),
    String(String),
    Document(BsonDocument),
    Array(BsonDocument),
    Binary { subtype: u8, data: Vec<u8> },
    Undefined,
    ObjectId([u8; 12]),
    Boolean(bool),
    DateTime(i64),
    Null,
    Regex(String, String),
    DBPointer(String, [u8; 12]),
    JavaScriptCode(String),
    Symbol(String),
    JavaScriptCodeWithScope(String, BsonDocument),
    Int32(i32),
    Int64(i64),
    Timestamp(u64),
    Decimal128([u8; 16]), // We don't have `f128` yet, so we use a byte array.
    MinKey,
    MaxKey,
}

impl<Encoder> Encodable<Encoder> for BsonElement
where
    Encoder: encode::ByteEncoder,
    BsonError: From<Encoder::Error>,
{
    type Error = BsonError;

    fn encode(&self, encoder: &mut Encoder) -> Result<(), Self::Error> {
        let e_name = (&self.e_value, 0u8);

        match &self.variant {
            // When all elements have the same error type, we can use a tuple
            // to encode them all at once.
            BsonElementVariant::Double(x) => (1i8, e_name, LE::new(*x)).encode(encoder)?,
            // However, when the error types are different, we need to either
            // transform the error type using `FromError` or encode them separately.
            BsonElementVariant::String(x) => {
                2i8.encode(encoder)?;
                e_name.encode(encoder)?;
                BsonString(x).encode(encoder)?;
            }
            BsonElementVariant::Document(x) => {
                3i8.encode(encoder)?;
                e_name.encode(encoder)?;
                x.encode(encoder)?;
            }
            BsonElementVariant::Array(x) => {
                4i8.encode(encoder)?;
                e_name.encode(encoder)?;
                x.encode(encoder)?;
            }
            BsonElementVariant::Binary { subtype, data } => {
                (5i8, e_name, LE::<i32>::try_from(data.len())?, subtype, data).encode(encoder)?
            }
            BsonElementVariant::Undefined => (6i8, e_name).encode(encoder)?,
            BsonElementVariant::ObjectId(x) => (7i8, e_name, x).encode(encoder)?,
            BsonElementVariant::Boolean(x) => (8i8, e_name, x).encode(encoder)?,
            BsonElementVariant::DateTime(x) => (9i8, e_name, LE::new(*x)).encode(encoder)?,
            BsonElementVariant::Null => (10i8, e_name).encode(encoder)?,
            BsonElementVariant::Regex(x, y) => (11i8, e_name, x, 0u8, y, 0u8).encode(encoder)?,
            BsonElementVariant::DBPointer(x, y) => {
                (12i8, e_name).encode(encoder)?;
                BsonString(x).encode(encoder)?;
                y.encode(encoder)?;
            }
            BsonElementVariant::JavaScriptCode(x) => (13i8, e_name, x).encode(encoder)?,
            BsonElementVariant::Symbol(x) => {
                (14i8, e_name).encode(encoder)?;
                BsonString(x).encode(encoder)?;
            }
            BsonElementVariant::JavaScriptCodeWithScope(x, y) => {
                (15i8, e_name).encode(encoder)?;
                let content = (BsonString(x), y);
                // We cannot use LengthPrefix here because we need to encode the size of
                // the content including the size field itself.
                let len = content.encoded_size()? + (i32::BITS / 8) as usize;
                LE::<i32>::try_from(len)?.encode(encoder)?;
                content.encode(encoder)?;
            }
            BsonElementVariant::Int32(x) => (16i8, e_name, LE::new(*x)).encode(encoder)?,
            BsonElementVariant::Timestamp(x) => (17i8, e_name, LE::new(*x)).encode(encoder)?,
            BsonElementVariant::Int64(x) => (18i8, e_name, LE::new(*x)).encode(encoder)?,
            BsonElementVariant::Decimal128(x) => (19i8, e_name, x).encode(encoder)?,
            BsonElementVariant::MinKey => (-1i8, e_name).encode(encoder)?,
            BsonElementVariant::MaxKey => (127i8, e_name).encode(encoder)?,
        };
        Ok(())
    }
}

struct BsonString<S>(S);

impl<S, Encoder> Encodable<Encoder> for BsonString<S>
where
    S: AsRef<str>,
    Encoder: encode::ByteEncoder,
    BsonError: From<Encoder::Error>,
{
    type Error = BsonError;

    fn encode(&self, encoder: &mut Encoder) -> Result<(), Self::Error> {
        LengthPrefix::<_, LE<i32>, BsonError>::new((self.0.as_ref(), 0u8)).encode(encoder)?;
        Ok(())
    }
}

fn main() -> Result<(), BsonError> {
    let document = BsonDocument {
        e_list: vec![
            BsonElement {
                e_value: "hello".into(),
                variant: BsonElementVariant::Double(1.0),
            },
            BsonElement {
                e_value: "world".into(),
                variant: BsonElementVariant::String("hello".into()),
            },
            BsonElement {
                e_value: "sub document".into(),
                variant: BsonElementVariant::Document(BsonDocument {
                    e_list: vec![BsonElement {
                        e_value: "hello".into(),
                        variant: BsonElementVariant::Double(1.0),
                    }],
                }),
            },
            BsonElement {
                e_value: "array".into(),
                variant: BsonElementVariant::Array(BsonDocument {
                    e_list: vec![BsonElement {
                        e_value: "0".into(), // BSON...
                        variant: BsonElementVariant::Double(1.0),
                    }],
                }),
            },
        ],
    };
    let size = document.encoded_size()?;
    println!("Expected BSON size: {}", size);

    // We can also encode the JSON string into a buffer, like a Vec<u8> or &mut
    // [u8].
    let mut buf = Vec::with_capacity(size);
    document.encode(&mut buf)?;

    println!("{:?}", document);
    println!("{:?}", buf.as_slice());
    Ok(())
}

#[cfg(test)]
mod official_examples {
    //! Tests using official examples from BSON specification https://bsonspec.org/faq.html
    use super::*;

    #[test]
    fn assert_that_hello_world_example_is_encoded_right() {
        //! {"hello": "world"}
        let expected = b"\x16\x00\x00\x00\x02hello\x00\x06\x00\x00\x00world\x00\x00";
        let document = BsonDocument {
            e_list: vec![BsonElement {
                e_value: "hello".into(),
                variant: BsonElementVariant::String("world".into()),
            }],
        };

        let mut buf = Vec::new();
        document.encode(&mut buf).unwrap();

        assert_eq!(buf.as_slice(), expected);
    }
    #[test]
    fn assert_that_awesome_example_is_encoded_right() {
        //! {"BSON": ["awesome", 5.05, 1986]}
        let expected = b"\x31\x00\x00\x00\x04BSON\x00\x26\x00\x00\x00\x02\x30\x00\x08\x00\x00\x00awesome\x00\x01\x31\x00\x33\x33\x33\x33\x33\x33\x14\x40\x10\x32\x00\xc2\x07\x00\x00\x00\x00";
        let document = BsonDocument {
            e_list: vec![BsonElement {
                e_value: "BSON".into(),
                variant: BsonElementVariant::Array(BsonDocument {
                    e_list: vec![
                        BsonElement {
                            e_value: "0".into(),
                            variant: BsonElementVariant::String("awesome".into()),
                        },
                        BsonElement {
                            e_value: "1".into(),
                            variant: BsonElementVariant::Double(5.05),
                        },
                        BsonElement {
                            e_value: "2".into(),
                            variant: BsonElementVariant::Int32(1986),
                        },
                    ],
                }),
            }],
        };

        let mut buf = Vec::new();
        document.encode(&mut buf).unwrap();

        assert_eq!(buf.as_slice(), expected);
    }
}
