use crate::{routes::users::AppClaims, state::AppState};
use axum::{extract::State, response::IntoResponse};
use axum_jwt::Claims;
pub async fn get_persistence(
    Claims(token): Claims<AppClaims>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let result = state
        .db
        .get_persistence("test".to_string())
        .await
        .expect("Failed to get");
    result
}
pub async fn set_persistence(State(state): State<AppState>, body: String) -> impl IntoResponse {
    let result = state
        .db
        .set_persistence("test".to_string(), body, true)
        .await
        .expect("Failed to get");
    result
}
