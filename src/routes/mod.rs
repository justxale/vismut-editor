use crate::VismutState;
use axum::extract::Path;
use axum::extract::Request;
use axum::http::{HeaderValue, StatusCode};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tower_http::cors::{CorsLayer, AllowOrigin};
use tracing::Span;

mod files;
mod schema;
mod console;

pub fn build_routes() -> Router<VismutState> {
    Router::new()
        .nest("/api", files::build_files_route())
        .nest("/api", schema::build_schema_route())
        .nest("/api", console::build_console_route())
        .route_service("/", ServeDir::new("static"))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http()
                    .make_span_with(|_: &Request| {
                        tracing::info_span!(
                            "api",
                            path = tracing::field::Empty,
                            err = tracing::field::Empty,
                        )
                    })
                    .on_request(|req: &Request, span: &Span| {
                        span.record("path", &tracing::field::display(&req.uri().path()));
                    })
                    .on_response(|response: &Response, duration: Duration, span: &Span| {
                        if !response.status().is_server_error() {
                            tracing::info!(parent: span, duration = ?duration.as_millis(), status_code = ?response.status());
                        }
                    }))
                .layer(TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(5)))
                .layer(CorsLayer::permissive().allow_origin( "http://localhost:11812".parse::<HeaderValue>().unwrap()))
        )
}
