use super::model::{SymboleCreateReq, SymboleUpdateReq};
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
    Path(_user_id): Path<String>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut idPictures: Vec<String> = Vec::new();

    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let content_type = field
            .content_type()
            .unwrap_or("application/octet-stream")
            .to_string();
        let data = field.bytes().await.unwrap();
        match upload_picture(&data, &content_type).await {
            Ok(id) => idPictures.push(id),
            Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
        }
    }
    Json(urls).into_response()
}
