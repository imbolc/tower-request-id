//! A tiny [tower] ([hyper], [axum], [warp] etc) service to generate a random id for each
//! incoming request.
//!
//! ## Usage
//!
//! After adding the [`RequestIdLayer`] into the [axum] middlewares the request id is available in
//! the [`http::Request::extensions()`]. For the [tracing] integration, please refer to the
//! [logging example].
//!
//! [tower]: https://crates.io/crates/tower
//! [hyper]: https://crates.io/crates/hyper
//! [axum]: https://crates.io/crates/axum
//! [warp]: https://crates.io/crates/warp
//! [tracing]: https://crates.io/crates/tracing
//! [`Request.extensions()`]: https://docs.rs/http/0.2.5/http/request/struct.Request.html#method.extensions
//! [logging example]: https://github.com/imbolc/tower-request-id/blob/main/examples/logging.rs

use http::Request;
use std::fmt;
use std::task::{Context, Poll};
use tower_layer::Layer;
use tower_service::Service;
use ulid::Ulid;

/// A newtype around [`ulid::Ulid`]
#[derive(Debug, Clone)]
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

/// Middleware to use [`RequestId`]
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

/// Layer to apply [`RequestIdService`] middleware.
#[derive(Clone, Debug)]
pub struct RequestIdLayer;

impl<S> Layer<S> for RequestIdLayer {
    type Service = RequestIdService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequestIdService { inner }
    }
}
