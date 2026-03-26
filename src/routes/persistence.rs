use crate::{routes::users::AppClaims, state::AppState};
use axum::{extract::State, http::status::StatusCode};
use axum_jwt::Claims;

// change this for get_myapp_data with key = my app
pub async fn get_persistence(
    Claims(token): Claims<AppClaims>,
    State(state): State<AppState>,
) -> Result<String, (StatusCode, String)> {
    println!("{:?}", token);
    let user_id = token.sub;
    let result = state
        .db
        .get_persistence("test".to_string(), user_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to get: {e}"),
            )
        })?;
    Ok(result)
}
pub async fn set_persistence(
    Claims(token): Claims<AppClaims>,
    State(state): State<AppState>,
    body: String,
) -> Result<(), (StatusCode, String)> {
    let user_id = token.sub;
    state
        .db
        .set_persistence("test".to_string(), body, user_id, true)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to set: {e}"),
            )
        })?;
    Ok(())
}
