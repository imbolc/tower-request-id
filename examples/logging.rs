use axum::{handler::get, Router};
use http::Request;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tower_request_id::{RequestId, RequestIdLayer, RequestSpan};
use tracing::{info, Level};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

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

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
