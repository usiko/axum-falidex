use axum::{
    Json,
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;

use crate::db::falidex::get_symboles_sens;

pub async fn get() -> Response {
    match get_symboles_sens().await {
        Ok(symboles_sens) => Json(symboles_sens).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get: {e}"),
        )
            .into_response(),
    }
}
