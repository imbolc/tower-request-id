use crate::RequestId;
use http::Request;
use tower_http::trace::MakeSpan;
use tracing::Span;

#[derive(Clone)]
pub struct RequestSpan;

impl<B> MakeSpan<B> for RequestSpan {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        let id = request
            .extensions()
            .get::<RequestId>()
            .map(ToString::to_string)
            .unwrap_or_else(|| "unknown".into());
        tracing::error_span!(
            "request",
            id = %id,
            method = %request.method(),
            uri = %request.uri(),
        )
    }
}
