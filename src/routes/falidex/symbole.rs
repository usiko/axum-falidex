use super::model::{SymboleCreateReq, SymboleUpdateReq};
use crate::resources::cloudinary::upload_picture_for_symbole;
use crate::{db::falidex::symbole, routes::users::AppClaims, state::AppState};
use axum::extract::Multipart;
use axum::http::status::StatusCode;
use axum::{
    Json,
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use axum_jwt::Claims;
use serde_json::json;

pub async fn get(State(state): State<AppState>) -> Response {
    match symbole::get(&state.db.db).await {
        Ok(symboles) => Json(symboles).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get: {e}"),
        )
            .into_response(),
    }
}

pub async fn create(
    Claims(token): Claims<AppClaims>,
    State(state): State<AppState>,
    Json(payload): Json<SymboleCreateReq>,
) -> Response {
    let user_id = token.sub;
    let symbole = payload.into();
    match symbole::create(&state.db.db, user_id, symbole).await {
        Ok(message) => (
            StatusCode::CREATED,
            Json(json!({
                "success": true,
                "message": message
            })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "error": e
            })),
        )
            .into_response(),
    }
}

pub async fn update(
    Claims(token): Claims<AppClaims>,
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<SymboleUpdateReq>,
) -> Response {
    let user_id = token.sub;
    let symbole = payload.into();
    match symbole::update(&state.db.db, user_id, id, symbole).await {
        Ok(message) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": message
            })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "error": e
            })),
        )
            .into_response(),
    }
}

pub async fn delete(
    Claims(token): Claims<AppClaims>,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Response {
    let user_id = token.sub;
    match symbole::delete(&state.db.db, user_id, id).await {
        Ok(message) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": message
            })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "error": e
            })),
        )
            .into_response(),
    }
}

pub async fn get_occurences(State(state): State<AppState>, Path(id): Path<String>) -> Response {
    match symbole::get_occurences(&state.db.db, id).await {
        Ok(occurences) => Json(occurences).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get occurences: {e}"),
        )
            .into_response(),
    }
}

pub async fn add_picture(
    Path(symbole_id): Path<String>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut results = Vec::new();
    let mut has_error = false;
    let mut has_success = false;
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let filename = field
            .file_name()
            .map(|name| name.to_string())
            .unwrap_or("file.bin".to_string());
        let file_bytes = field.bytes().await.unwrap().to_vec();
        let result = upload_picture_for_symbole(file_bytes, filename, &symbole_id).await;

        match result {
            Ok(id) => {
                has_success = true;
                results.push(json!({"success": true, "id": id}))
            }
            Err(e) => {
                has_error = true;
                results.push(json!({"success": false, "error": e}))
            }
        }
    }
    let status = if has_error {
        if has_success {
            StatusCode::PARTIAL_CONTENT
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        }
    } else {
        StatusCode::OK
    };
    (status, Json(results)).into_response()
}
