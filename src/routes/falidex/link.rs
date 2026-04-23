use axum::{
    Json,
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;

use crate::{
    db::falidex::{get_link_item, get_links},
    state::AppState,
};

pub async fn get(State(state): State<AppState>) -> Response {
    match get_links(&state.db.db).await {
        Ok(links) => Json(links).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get: {e}"),
        )
            .into_response(),
    }
}
pub async fn get_item(State(state): State<AppState>, Path(link_id): Path<String>) -> Response {
    match get_link_item(&state.db.db, link_id).await {
        Ok(link) => Json(link).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get: {e}"),
        )
            .into_response(),
    }
}
