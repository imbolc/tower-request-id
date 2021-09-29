//! A tiny [tower] ([hyper], [axum], [warp] etc) service to generate a random id for each
//! incoming request.
//!
//! ## Usage
//!
//! After enabling [`RequestIdLayer`] the id is available in [`Request.extensions()`]. There's
//! also [`RequestSpan`] to use it as a [tracing::span]. E.g. the following code
//!
//! ```rust,no_run
//! use tower_request_id::{RequestId, RequestIdLayer, RequestSpan};
//!
//! let middlewares = ServiceBuilder::new()
//!     .layer(RequestIdLayer)
//!     .layer(TraceLayer::new_for_http().make_span_with(RequestSpan))
//!     .into_inner();
//!
//! let app = Router::new()
//!     .route(
//!         "/",
//!         get(|rq: Request<_>| async move {
//!             info!("it's in the request span");
//!             let id = rq.extensions().get::<RequestId>().unwrap();
//!             info!("or directly: {}", id);
//!             "ok"
//!         }),
//!     )
//!     .layer(middlewares);
//! ```
//!
//! will generate logging output like this:
//!
//! ```sh
//! INFO request{id=01FGR4DNBYJ0M7ZV6XS3JHXDD1 method=GET uri=/}: logging: it's in the request span
//! INFO request{id=01FGR4DNBYJ0M7ZV6XS3JHXDD1 method=GET uri=/}: logging: or directly: 01FGR4DNBYJ0M7ZV6XS3JHXDD1
//! ```
//!
//! The full example is available in [examples/logging.rs][example]
//!
//! [tower]: https://crates.io/crates/tower
//! [hyper]: https://crates.io/crates/hyper
//! [axum]: https://crates.io/crates/axum
//! [warp]: https://crates.io/crates/warp
//! [`RequestIdLayer`]: crate::RequestIdLayer
//! [`RequestSpan`]: crate::RequestSpan
//! [tracing::span]: https://docs.rs/tracing/0.1.28/tracing/span/index.html
//! [`Request.extensions()`]: https://docs.rs/http/0.2.5/http/request/struct.Request.html#method.extensions
//! [example]: https://github.com/imbolc/tower-request-id/blob/main/examples/logging.rs

use http::Request;
#[cfg(feature = "tower-layer")]
pub use layer::RequestIdLayer;
use std::fmt;
use std::task::{Context, Poll};
use tower_service::Service;
#[cfg(feature = "tower-http-trace")]
pub use trace::RequestSpan;
use ulid::Ulid;

#[cfg(feature = "tower-layer")]
pub mod layer;

#[cfg(feature = "tower-http-trace")]
pub mod trace;

#[derive(Debug)]
pub struct RequestId(pub Ulid);

impl RequestId {
    fn new() -> Self {
        Self(Ulid::new())
    }
}

impl fmt::Display for RequestId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let mut buffer = [0; ulid::ULID_LEN];
        write!(f, "{}", self.0.to_str(&mut buffer).unwrap_or_default())
    }
}

#[derive(Clone, Debug)]
pub struct RequestIdService<S> {
    inner: S,
}

impl<S> RequestIdService<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<B, S> Service<Request<B>> for RequestIdService<S>
where
    S: Service<Request<B>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<B>) -> Self::Future {
        let id = RequestId::new();
        req.extensions_mut().insert(id);
        self.inner.call(req)
    }
}
