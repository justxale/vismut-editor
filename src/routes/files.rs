use std::error::Error;
use crate::database::{Script, ScriptActiveModel};
use crate::requests::{OffsetLimitQuery, PostScript};
use crate::responses::{AllScriptsResponse, VismutErrorResponse};
use crate::responses::{FailableResponse, ScriptResponse};
use crate::VismutState;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use sea_orm::{DbErr, EntityTrait, QueryOrder, QuerySelect, RuntimeErr};
use tracing::Span;
use uuid::Uuid;

async fn get_files(
    State(state): State<VismutState>,
    Query(query): Query<OffsetLimitQuery>,
) -> FailableResponse<AllScriptsResponse> {
    match Script::find()
        .order_by_desc(Script::COLUMN.edited_at)
        .limit(query.limit)
        .offset(query.offset)
        .all(state.get_connection())
        .await
    {
        Ok(files) => Ok((StatusCode::OK, Json(AllScriptsResponse::from(files)))),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(VismutErrorResponse::new(err)),
        ))
    }
}

async fn post_file(
    State(state): State<VismutState>,
    Json(request): Json<PostScript>,
) -> FailableResponse<ScriptResponse> {
    let script = ScriptActiveModel::new(&request.title);
    match Script::insert(script)
        .exec_with_returning(state.get_connection())
        .await
    {
        Ok(model) => Ok((StatusCode::OK, Json(ScriptResponse::from(model)))),
        Err(err) => {
            Span::current().record("err", &tracing::field::display(&err));
            Err((
                StatusCode::CONFLICT,
                Json(VismutErrorResponse::new(err)),
            ))
        }
    }
}

async fn get_file_by_id(
    Path(id): Path<Uuid>,
    State(state): State<VismutState>,
) -> FailableResponse<ScriptResponse> {
    match Script::find_by_id(id).one(state.get_connection()).await {
        Ok(Some(model)) => Ok((StatusCode::OK, Json(ScriptResponse::from(model)))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(VismutErrorResponse::new(format!("No such script: {}", id))),
        )),
        Err(err) => {
            match err {
                DbErr::RecordNotFound(_) => Err((
                    StatusCode::NOT_FOUND,
                    Json(VismutErrorResponse::new(err)),
                )),
                _ => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(VismutErrorResponse::new(err)))),
            }
        }
    }
}

async fn put_file_by_id(
    Path(id): Path<Uuid>,
    State(state): State<VismutState>,
) -> FailableResponse<ScriptResponse> {
    Err((StatusCode::NOT_IMPLEMENTED, Json(VismutErrorResponse::new("Not implemented"))))
}

async fn delete_file_by_id(
    Path(id): Path<Uuid>,
    State(state): State<VismutState>,
) -> FailableResponse<ScriptResponse> {
    Err((StatusCode::NOT_IMPLEMENTED, Json(VismutErrorResponse::new("Not implemented"))))
}

pub fn build_files_route() -> Router<VismutState> {
    Router::new()
        .route("/files", get(get_files).post(post_file))
        .route(
            "/files/{id}",
            get(get_file_by_id)
                .put(put_file_by_id)
                .delete(delete_file_by_id),
        )
}
