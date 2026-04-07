use axum::{Json, extract::State};
use reqwest::StatusCode;

use crate::{
    db::model::{ErrorResult, TokenAuth, TokenAuthResponse},
    state::AppState,
    token::verify_temp_token,
};
pub async fn verify_hash(
    State(state): State<AppState>,
    Json(payload): Json<TokenAuth>,
) -> Result<Json<TokenAuthResponse>, (StatusCode, Json<ErrorResult>)> {
    if !verify_temp_token(&payload.role, payload.timestamp, &payload.hash) {
        let token = state.token_store.generate_and_store();
        Ok(Json(TokenAuthResponse { token }))
    } else {
        let error = ErrorResult {
            error: "UNAUTHORIZED".to_string(),
            message: "you shoundn't be here".to_string(),
        };
        Err((StatusCode::UNAUTHORIZED, Json(error)))
    }
}
