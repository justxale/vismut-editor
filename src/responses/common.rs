use axum::http::StatusCode;
use axum::Json;
use sea_orm::DbErr;
use serde::Serialize;
use crate::responses::ScriptResponse;

#[derive(Serialize)]
pub struct VismutErrorResponse {
    err: String,
}

impl VismutErrorResponse {
    pub fn new(err: impl ToString) -> Self {
       Self { err: err.to_string() }
    }
}

pub type FailableResponse<T> = Result<(StatusCode, Json<T>), (StatusCode, Json<VismutErrorResponse>)>;