[![version-badge][]][crate-url]
[![docs-badge][]][docs-url]
[![license-badge][]][crate-url]

# tower-request-id

A tiny [tower] ([hyper], [axum], [warp] etc) service to generate a random id for each
incoming request.

### Usage

After adding the [`RequestIdLayer`] into the [axum] middlewares the request id is available in
the [`http::Request::extensions()`]. For the [tracing] integration, please refer to the
[logging example].

[tower]: https://crates.io/crates/tower
[hyper]: https://crates.io/crates/hyper
[axum]: https://crates.io/crates/axum
[warp]: https://crates.io/crates/warp
[tracing]: https://crates.io/crates/tracing
[`Request.extensions()`]: https://docs.rs/http/0.2.5/http/request/struct.Request.html#method.extensions
[logging example]: https://github.com/imbolc/tower-request-id/blob/main/examples/logging.rs

[version-badge]: https://img.shields.io/crates/v/tower-request-id.svg
[docs-badge]: https://docs.rs/tower-request-id/badge.svg
[license-badge]: https://img.shields.io/crates/l/tower-request-id.svg
[crate-url]: https://crates.io/crates/tower-request-id
[docs-url]: https://docs.rs/tower-request-id
