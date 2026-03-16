use crate::state::AppState;
use axum::{extract::State, response::IntoResponse};
pub async fn get_persistence(State(state): State<AppState>) -> impl IntoResponse {
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
