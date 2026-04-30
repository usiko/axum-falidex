use axum::{
    Json,
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;
use serde_json::json;

use crate::{
    db::falidex::{
        create_link_item, create_link_item_relation, delete_link_item, delete_link_item_relation,
        get_link_item, get_links, update_link_item, update_link_item_relation,
    },
    model::falidex_model::{CreateLinkDetail, LinkDetail, LinkItem},
    state::AppState,
};

pub async fn get(State(state): State<AppState>) -> Response {
    match get_links(&state.db.db).await {
        Ok(links) => Json(links).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get: {e}"),
        )
            .into_response(),
    }
}
pub async fn get_item(State(state): State<AppState>, Path(link_id): Path<String>) -> Response {
    match get_link_item(&state.db.db, link_id).await {
        Ok(link) => Json(link).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get: {e}"),
        )
            .into_response(),
    }
}
pub async fn update_item(
    State(state): State<AppState>,
    Json(payload): Json<LinkDetail>,
) -> Response {
    match update_link_item(&state.db.db, payload).await {
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
pub async fn update_item_relation(
    State(state): State<AppState>,
    Path(link_id): Path<String>,
    Json(payload): Json<LinkItem>,
) -> Response {
    match update_link_item_relation(&state.db.db, link_id, payload).await {
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
pub async fn create_item_relation(
    State(state): State<AppState>,
    Path(link_id): Path<String>,
    Json(payload): Json<LinkItem>,
) -> Response {
    match create_link_item_relation(&state.db.db, link_id, payload).await {
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
pub async fn create_item(
    State(state): State<AppState>,
    Json(payload): Json<CreateLinkDetail>,
) -> Response {
    match create_link_item(&state.db.db, payload).await {
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

pub async fn delete_item(State(state): State<AppState>, Path(link_id): Path<String>) -> Response {
    match delete_link_item(&state.db.db, link_id).await {
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

pub async fn delete_item_relation(
    State(state): State<AppState>,
    Path((link_id, relation_id)): Path<(String, String)>,
) -> Response {
    match delete_link_item_relation(&state.db.db, link_id, relation_id).await {
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
