# Encode, a Rust library for building encoders and serializers

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/Altair-Bueno/encode/actions/workflows/ci.yaml/badge.svg)](https://github.com/Altair-Bueno/encode/actions/workflows/ci.yaml)
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

## Cargo features

- `default`: Enables the `std` feature.
- `std`: Enables the use of the standard library.
- `alloc`: Enables the use of the `alloc` crate.
- `arrayvec`: Implements [`Encodable`] for [`arrayvec::ArrayVec`].
