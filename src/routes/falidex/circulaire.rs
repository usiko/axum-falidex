use axum::{
    Json,
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;

use crate::db::falidex::get_circulaires;

pub async fn get() -> Response {
    match get_circulaires().await {
        Ok(circulaires) => Json(circulaires).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get: {e}"),
        )
            .into_response(),
    }
}
