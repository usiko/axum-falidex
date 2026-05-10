use axum::{
    Json,
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use axum_jwt::Claims;
use mongodb::bson::oid::ObjectId;
use reqwest::StatusCode;
use serde_json::json;

use crate::{
    db::falidex::relations,
    model::falidex_model::{CreateLinkDetail, LinkDetail, LinkDetailReq, LinkItem},
    routes::users::AppClaims,
    state::AppState,
};

pub async fn get(State(state): State<AppState>) -> Response {
    match relations::get(&state.db.db).await {
        Ok(links) => Json(links).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get: {e}"),
        )
            .into_response(),
    }
}
pub async fn get_item(State(state): State<AppState>, Path(link_id): Path<String>) -> Response {
    match relations::get_item(&state.db.db, link_id).await {
        Ok(link) => Json(link).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get: {e}"),
        )
            .into_response(),
    }
}
pub async fn update_item(
    Claims(token): Claims<AppClaims>,
    State(state): State<AppState>,
    Json(payload): Json<LinkDetailReq>,
) -> Response {
    let user_id = token.sub;
    let now = chrono::Utc::now().to_rfc3339();
    let link_detail = LinkDetail {
        id: payload.id,
        name: payload.name,
        relations: payload.relations,
        specificites: payload.specificites,
        annee: payload.annee,
        created_at: payload.created_at.or(Some(now.clone())),
        last_update: now,
        default: payload.default,
        visible: payload.visible,
        editable: payload.editable,
        national: payload.national,
        ville: payload.ville,
    };

    match relations::update(&state.db.db, user_id, link_detail).await {
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
    Claims(token): Claims<AppClaims>,
    State(state): State<AppState>,
    Path(link_id): Path<String>,
    Json(payload): Json<LinkItem>,
) -> Response {
    let user_id = token.sub;
    match relations::update_item_relation(&state.db.db, user_id, link_id, payload).await {
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
    Claims(token): Claims<AppClaims>,
    State(state): State<AppState>,
    Path(link_id): Path<String>,
    Json(payload): Json<LinkItem>,
) -> Response {
    let user_id = token.sub;
    match relations::create_item_relation(&state.db.db, user_id, link_id, payload).await {
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
    Claims(token): Claims<AppClaims>,
    State(state): State<AppState>,
    Json(mut payload): Json<CreateLinkDetail>,
) -> Response {
    let user_id = token.sub;
    // Générer un _id string si absent
    if payload.id.is_none() {
        payload.id = Some(ObjectId::new().to_hex());
    }

    // Définir les dates de création et modification
    let now = chrono::Utc::now().to_rfc3339();
    payload.created_at = Some(now.clone());
    payload.last_update = Some(now);
    match relations::create(&state.db.db, user_id, payload).await {
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

pub async fn delete_item(
    Claims(token): Claims<AppClaims>,
    State(state): State<AppState>,
    Path(link_id): Path<String>,
) -> Response {
    let user_id = token.sub;
    match relations::delete(&state.db.db, user_id, link_id).await {
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
    Claims(token): Claims<AppClaims>,
    State(state): State<AppState>,
    Path((link_id, relation_id)): Path<(String, String)>,
) -> Response {
    let user_id = token.sub;
    match relations::delete_item_relation(&state.db.db, user_id, link_id, relation_id).await {
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
