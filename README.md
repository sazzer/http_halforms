# HAL Forms

[![Build status](https://github.com/sazzer/http_halforms/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/sazzer/http_halforms/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/http_halforms)](https://crates.io/crates/http_halforms)
[![Documentation](https://docs.rs/http_halforms/badge.svg)](https://docs.rs/http_halforms)

## Supported HTTP Servers

Currently this is only supported with the following HTTP Servers:

- [Axum](https://crates.io/crates/axum)

# Features

HTTP Server support is behind feature flags for the appropriate HTTP Server. As such, you will need to enable the correct feature for the HTTP Server that you are using.

Currently supported features are:

- `axum` - For the [Axum](https://crates.io/crates/axum) HTTP Server.

## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in 100% safe Rust.

## Minimum supported Rust version

The MSRV for `http_halforms` is 1.78.0. However, the HTTP Servers that are used with it might need a higher version.

## License

This project is licensed under the MIT license.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in `http_halforms` by you, shall be licensed as MIT, without any additional terms or conditions.
