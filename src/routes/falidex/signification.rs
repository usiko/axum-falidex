use crate::{db::falidex::signification, state::AppState};
use super::model::SignificationReq;
use axum::{
    Json,
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use axum::http::status::StatusCode;
use serde_json::json;
use crate::routes::users::AppClaims;
use axum_jwt::Claims;

pub async fn get(State(state): State<AppState>) -> Response {
    match signification::get(&state.db.db).await {
        Ok(significations) => Json(significations).into_response(),
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
    Json(payload): Json<SignificationReq>,
) -> Response {
    let user_id = token.sub;
    let signification = payload.into();
    match signification::create(&state.db.db, user_id, signification).await {
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
    Json(payload): Json<SignificationReq>,
) -> Response {
    let user_id = token.sub;
    let signification = payload.into();
    match signification::update(&state.db.db, user_id, id, signification).await {
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
    match signification::delete(&state.db.db, user_id, id).await {
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
    match signification::get_occurences(&state.db.db, id).await {
        Ok(occurences) => Json(occurences).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get occurences: {e}"),
        )
            .into_response(),
    }
}
