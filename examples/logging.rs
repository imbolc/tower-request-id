use axum::{routing::get, Router};
use http::Request;
use hyper::Body;
use tower_http::trace::TraceLayer;
use tower_request_id::{RequestId, RequestIdLayer, RequestSpan};
use tracing::{error_span, info, Level};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let app = Router::new()
        .route("/", get(handler))
        .layer(
            // Let's create a tracing span for each request
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                // We get the request id from the extensions
                let request_id = request
                    .extensions()
                    .get::<RequestId>()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| "unknown".into());
                // And then we put it along with other information into the `request` span
                tracing::error_span!(
                    "request",
                    id = %request_id,
                    method = %request.method(),
                    uri = %request.uri(),
                )
            }),
        )
        // This layer creates a new id for each request and puts it into the request extensions.
        // Note that it should be added after the Trace layer.
        .layer(RequestIdLayer);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    info!("hi");
    "ok"
}
