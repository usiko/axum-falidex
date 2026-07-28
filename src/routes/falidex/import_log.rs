use crate::{db::falidex::import_log, state::AppState};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::status::StatusCode;
use axum::response::{IntoResponse, Response};

pub async fn get(State(state): State<AppState>) -> Response {
    match import_log::get(&state.db.db).await {
        Ok(codes) => Json(codes).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get: {e}"),
        )
            .into_response(),
    }
}

pub async fn get_item(State(state): State<AppState>, Path(id): Path<String>) -> Response {
    match import_log::get_item(&state.db.db, id).await {
        Ok(code) => Json(code).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get: {e}"),
        )
            .into_response(),
    }
}
