use crate::{
    db::falidex::import as import_db, model::import_model::ImportBatchRequest,
    routes::users::AppClaims, state::AppState,
};
use axum::Json;
use axum::extract::State;
use axum::http::status::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_jwt::Claims;
use serde_json::json;

pub async fn create(
    Claims(token): Claims<AppClaims>,
    State(state): State<AppState>,
    Json(payload): Json<ImportBatchRequest>,
) -> Response {
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
