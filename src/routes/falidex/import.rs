use crate::{
    db::falidex::import as import_db,
    model::import_model::{self, ImportBatchRequest},
    routes::users::AppClaims,
    state::AppState,
};
use axum::Json;
use axum::extract::State;
use axum::http::status::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_jwt::Claims;
use serde_json::{Value, json};

pub async fn create(
    Claims(token): Claims<AppClaims>,
    State(state): State<AppState>,
    Json(raw): Json<Value>,
) -> Response {
    if let Err(errors) = import_model::validate_batch_json(&raw) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": "JSON invalide au regard du schéma d'import",
                "details": errors
            })),
        )
            .into_response();
    }

    let payload: ImportBatchRequest = match serde_json::from_value(raw) {
        Ok(payload) => payload,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "success": false,
                    "error": format!("Erreur de désérialisation: {e}")
                })),
            )
                .into_response();
        }
    };

    let user_id = token.sub;
    match import_db::apply_batch(&state.db.db, &state.db.client, user_id, payload).await {
        Ok(code) => (
            StatusCode::CREATED,
            Json(json!({
                "success": true,
                "code": code
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
