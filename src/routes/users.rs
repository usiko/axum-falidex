use crate::db::model::{AuthRequest, User, UserAuth};
use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;

pub async fn auth(
    State(state): State<AppState>,
    Json(payload): Json<AuthRequest>,
) -> Result<Json<UserAuth>, (StatusCode, String)> {
    let result = state
        .db
        .auth_user(payload.user_name, payload.hash)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to auth: {e}"),
            )
        })?;
    Ok(Json(result))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<Json<User>, (StatusCode, String)> {
    print!("req user with id {}", user_id);
    let result = state.db.get_user(user_id).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get user: {e}"),
        )
    })?;
    Ok(Json(result))
}
