use axum::{
    Json,
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;

use crate::db::falidex::get_colors;

pub async fn get() -> Response {
    match get_colors().await {
        Ok(colors) => Json(colors).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get: {e}"),
        )
            .into_response(),
    }
}
