use axum::{
    Json,
    extract::Path,
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;

use crate::db::falidex::{get_link_item, get_links};

pub async fn get() -> Response {
    match get_links().await {
        Ok(links) => Json(links).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get: {e}"),
        )
            .into_response(),
    }
}
pub async fn get_item(Path(link_id): Path<String>) -> Response {
    match get_link_item(link_id).await {
        Ok(link) => Json(link).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get: {e}"),
        )
            .into_response(),
    }
}
