use crate::VismutState;
use axum::extract::Path;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::response::Response;
use axum::routing::get;
use axum::{Router};
use std::time::Duration;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::{TraceLayer};
use tracing::{Span};

mod files;
mod static_files;

async fn handler(Path(path): Path<String>) -> String {
    path
}

pub fn build_routes() -> Router<VismutState> {
    Router::new()
        .nest("/api", files::build_files_route())
        .route("/{*path}", get(handler))
        .layer(
            (
                TraceLayer::new_for_http()
                    .make_span_with(|_: &Request| {
                        tracing::info_span!(
                            "api",
                            path = tracing::field::Empty,
                        )
                    })
                    .on_request(|req: &Request, span: &Span| {
                        span.record("path", &tracing::field::display(&req.uri().path()));
                    })
                    .on_response(|response: &Response, latency: Duration, _span: &Span| {
                        tracing::info!(duration = ?latency.as_millis(), status_code = ?response.status());
                    }),
                TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(5))
            )
        )
}
