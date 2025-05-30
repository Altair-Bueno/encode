# Encode, a Rust library for building encoders and serializers

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/Altair-Bueno/encode/actions/workflows/ci.yml/badge.svg)](https://github.com/Altair-Bueno/encode/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/Altair-Bueno/encode/graph/badge.svg?token=Q89UGZC3RI)](https://codecov.io/gh/Altair-Bueno/encode)
[![Crates.io Version](https://img.shields.io/crates/v/encode.svg)](https://crates.io/crates/encode)

> Encoders/serializers made easy.

`encode` is a toolbox for building encoders and serializers in Rust. It is
heavily inspired by the [`winnow`](https://docs.rs/winnow/latest/winnow/) and
[`nom`](https://docs.rs/nom/latest/nom/) crates, which are used for building
parsers. It is meant to be a companion to these crates, providing a similar
level of flexibility and ease of use for reversing the parsing process.

The main idea behind `encode` is to provide a set of combinators for building
serializers. These combinators can be used to build complex encoders from simple
building blocks. This makes it easy to build encoders for different types of
data, without having to write a lot of boilerplate code.

Another key feature of `encode` is its support for `no_std` environments. This
makes it suitable for use in embedded systems, where the standard library (and
particularly the [`std::io`] module) is not available.

See the `examples` folder for some examples of how to use `encode`. Also, check
the [`combinators`] module for a list of all the combinators provided by the
crate.

## Feature highlights

- `#![no_std]` compatible
- `#![forbid(unsafe_code)]`
- Simple and flexible API
- Minimal dependencies
- Ready to use combinators for minimizing boilerplate.
- Write encoders that serialize data to UTF-8 and/or raw bytes

## Cargo features

- `default`: Enables the `std` feature.
- `std`: Enables the use of the standard library.
- `alloc`: Enables the use of the `alloc` crate.
- `arrayvec`: Implements [`Encodable`] and [`ByteEncoder`] for
  [`arrayvec::ArrayVec`] and [`arrayvec::ArrayString`]. Implements
  [`StrEncoder`] for [`arrayvec::ArrayString`].
- `bytes`: Implements [`Encodable`] and [`ByteEncoder`] for [`bytes::BytesMut`].
  Implements [`Encodable`] for [`bytes::Bytes`].

## FAQs

### Why the `ByteEncoder` trait instead of `bytes::BufMut`?

From
[bytes documentation](https://docs.rs/bytes/latest/bytes/buf/trait.BufMut.html)

> A buffer stores bytes in memory such that write operations are **infallible**.
> The underlying storage may or may not be in contiguous memory. A BufMut value
> is a cursor into the buffer. Writing to BufMut advances the cursor position.

The bytes crate was never designed with falible writes nor `no_std` targets in
mind. This means that targets with little memory are forced to crash when memory
is low, instead of gracefully handling errors.

### Why the `ByteEncoder` trait instead of `std::io::Write`?

Because
[it's not available on `no_std`](https://github.com/rust-lang/rust/issues/48331)

### Why the `StrEncoder` trait instead of `std::fmt::Write`?

Because `std::fmt::Write` is not implemented for a lot of types, most notably
`Vec<u8>` or `&mut [u8]`. This means that you would have to use some sort of
adapter in between to use these types as buffers. Furthermore, `std::fmt::Write`
the error type is limited on what it can do:

> The purpose of that error is to abort the formatting operation when the
> underlying destination encounters some error preventing it from accepting more
> text; in particular, it does not communicate any information about what error
> occurred. It should generally be propagated rather than handled, at least when
> implementing formatting traits.

### Why did you build this?

- Because there is no alternative, at least that i know of, that supports
  `no_std` properly
- Because it easily lets you create
  [TLV types](https://en.wikipedia.org/wiki/Type–length–value)
- Because it's easier to work with than `std::io::Write` and `std::fmt::Write`
- Because using `format_args!` with binary data often leads to a lot of
  boilerplate

### [`BaseEncoder`] vs [`ByteEncoder`] vs [`StrEncoder`]

- [`BaseEncoder`]: Provides abstraction and trait bounds. It is useful for
  building combinators that can work with any type of encoder.
- [`StrEncoder`]: Provides a simple interface for encoding UTF-8 text. It is
  suitable for UTF-8 text output.
- [`ByteEncoder`]: Provides full control over the encoding process. It is
  suitable for low-level encoding tasks, such as writing raw bytes to a buffer.
