use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Router;
use axum::routing::get;
use vismut_core::RegistrySchema;
use crate::responses::{FailableResponse, VismutErrorResponse};
use crate::state::VismutState;

async fn get_schema(State(state): State<VismutState>) -> FailableResponse<RegistrySchema> {
    match state.get_executor().get_schema().as_ref() {
        Some(schema) => Ok((StatusCode::OK, Json(schema.clone()))),
        None => Err((StatusCode::SERVICE_UNAVAILABLE, Json(VismutErrorResponse::new("Schema is not generated"))))
    }
}

pub fn build_schema_route() -> Router<VismutState> {
    Router::new().route("/schema", get(get_schema))
}