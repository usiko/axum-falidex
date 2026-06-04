use crate::resources::cloudinary::{remove_picture, upload_url_picture};
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

pub async fn test_remote_url() -> (StatusCode, Json<serde_json::Value>) {
    let tags = Vec::from(["symbole".to_string(), "test".to_string()]);
    let result = upload_url_picture(
        "https://picsum.photos/800/800".to_string(),
        "falidex".to_string(),
        "test_falidex/test".to_string(),
        tags,
    )
    .await;

    match result {
        Ok(response) => {
            println!("upload from url {:?}", response)
        }
        Err(error) => {
            eprintln!("error upload from url {:?}", error)
        }
    };

    (
        StatusCode::OK,
        Json(json!({"success": true, "message": "test"})),
    )
}
