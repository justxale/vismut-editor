use crate::VismutState;
use axum::extract::State;
use axum::routing::{get};
use axum::Router;
use std::time::Duration;

async fn post_file(State(state): State<VismutState>) {}

async fn delete_file(State(state): State<VismutState>) {}

async fn get_file(State(state): State<VismutState>) {
    tokio::time::sleep(Duration::from_secs(2)).await;
}

async fn put_file(State(state): State<VismutState>) {}

pub fn build_files_route() -> Router<VismutState> {
    Router::new()
        .route("/files", get(get_file).post(post_file))
        .route(
            "/files/{id}",
            get(get_file).put(put_file).delete(delete_file),
        )
}
