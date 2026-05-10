use crate::{db::falidex::circulaire_color, state::AppState};
use super::model::CirculaireColorReq;
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
    match circulaire_color::get(&state.db.db).await {
        Ok(circulaires_colors) => Json(circulaires_colors).into_response(),
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
    Json(payload): Json<CirculaireColorReq>,
) -> Response {
    let user_id = token.sub;
    let circulaire_color = payload.into();
    match circulaire_color::create(&state.db.db, user_id, circulaire_color).await {
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
    Json(payload): Json<CirculaireColorReq>,
) -> Response {
    let user_id = token.sub;
    let circulaire_color = payload.into();
    match circulaire_color::update(&state.db.db, user_id, id, circulaire_color).await {
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
    match circulaire_color::delete(&state.db.db, user_id, id).await {
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
    match circulaire_color::get_occurences(&state.db.db, id).await {
        Ok(occurences) => Json(occurences).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get occurences: {e}"),
        )
            .into_response(),
    }
}
