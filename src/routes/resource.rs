use crate::resources::cloudinary::remove_picture;
use axum::{Json, extract::Path, http::StatusCode};
use serde_json::json;

pub async fn remove(Path(id): Path<String>) -> (StatusCode, Json<serde_json::Value>) {
    match remove_picture(id).await {
        Ok(msg) => (
            StatusCode::OK,
            Json(json!({"success": true, "message": msg})),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"success": false, "error": e})),
        ),
    }
}
