use crate::{routes::users::AppClaims, state::AppState};
use axum::{extract::State, response::IntoResponse};
use axum_jwt::Claims;
pub async fn get_persistence(
    Claims(token): Claims<AppClaims>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    println!("{:?}", token);
    let user_id = token.sub;
    let result = state
        .db
        .get_persistence("test".to_string(), user_id)
        .await
        .expect("Failed to get");
    result
}
pub async fn set_persistence(
    Claims(token): Claims<AppClaims>,
    State(state): State<AppState>,
    body: String,
) -> impl IntoResponse {
    let user_id = token.sub;
    let result = state
        .db
        .set_persistence("test".to_string(), body, user_id, true)
        .await
        .expect("Failed to get");
    result
}
