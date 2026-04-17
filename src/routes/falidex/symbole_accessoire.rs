use axum::{
    Json,
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;

use crate::db::falidex::get_symbole_accessoires;

pub async fn get() -> Response {
    match get_symbole_accessoires().await {
        Ok(symbole_accessoires) => Json(symbole_accessoires).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get: {e}"),
        )
            .into_response(),
    }
}
