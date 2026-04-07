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
    let secret = get_token_hash_key();
    if verify_temp_token(&payload.role, &secret, payload.timestamp, &payload.hash) {
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

fn get_token_hash_key() -> String {
    std::env::var("TOKEN_HASH_KEY").unwrap_or("default_dev_token_hash_please_change".to_string())
}
