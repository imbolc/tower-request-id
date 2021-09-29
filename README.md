[![version-badge][]][crate-url]
[![docs-badge][]][docs-url]
[![license-badge][]][crate-url]

# tower-request-id

A tiny [tower] ([hyper], [axum], [warp] etc) service to generate a random id for each
incoming request.

### Usage

After enabling [`RequestIdLayer`] the id is available in [`Request.extensions()`]. There's
also [`RequestSpan`] to use it as a [tracing::span]. E.g. the following code

```rust
use tower_request_id::{RequestId, RequestIdLayer, RequestSpan};

let middlewares = ServiceBuilder::new()
    .layer(RequestIdLayer)
    .layer(TraceLayer::new_for_http().make_span_with(RequestSpan))
    .into_inner();

let app = Router::new()
    .route(
        "/",
        get(|rq: Request<_>| async move {
            info!("it's in the request span");
            let id = rq.extensions().get::<RequestId>().unwrap();
            info!("or directly: {}", id);
            "ok"
        }),
    )
    .layer(middlewares);
```

will generate logging output like this:

```sh
INFO request{id=01FGR4DNBYJ0M7ZV6XS3JHXDD1 method=GET uri=/}: logging: it's in the request span
INFO request{id=01FGR4DNBYJ0M7ZV6XS3JHXDD1 method=GET uri=/}: logging: or directly: 01FGR4DNBYJ0M7ZV6XS3JHXDD1
```

The full example is available in `examples/logging.rs`

[tower]: https://crates.io/crates/tower
[hyper]: https://crates.io/crates/hyper
[axum]: https://crates.io/crates/axum
[warp]: https://crates.io/crates/warp
[`RequestIdLayer`]: crate::RequestIdLayer
[`RequestSpan`]: crate::RequestSpan
[tracing::span]: https://docs.rs/tracing/0.1.28/tracing/span/index.html
[`Request.extensions()`]: https://docs.rs/http/0.2.5/http/request/struct.Request.html#method.extensions

[version-badge]: https://img.shields.io/crates/v/tower-request-id.svg
[docs-badge]: https://docs.rs/tower-request-id/badge.svg
[license-badge]: https://img.shields.io/crates/l/tower-request-id.svg
[crate-url]: https://crates.io/crates/tower-request-id
[docs-url]: https://docs.rs/tower-request-id
