use axum::http::{
    StatusCode,
    header::{HeaderValue, SET_COOKIE},
};
use axum::{
    Json,
    extract::State,
    response::{IntoResponse, Response},
};

use crate::security_utils::verify_temp_token;
use crate::{
    db::model::{ErrorResult, TokenAuth, TokenAuthResponse},
    state::AppState,
};
pub async fn verify_hash(
    State(state): State<AppState>,
    Json(payload): Json<TokenAuth>,
) -> Result<Response, (StatusCode, Json<ErrorResult>)> {
    if verify_temp_token(&payload.role, payload.timestamp, &payload.hash) {
        let (token, cookie_value) = state.security_store.generate_and_store();
        let cookie = format!("unique_id={}; HttpOnly; SameSite=Lax; Path=/", cookie_value);
        let mut response = Json(TokenAuthResponse { token }).into_response();
        response
            .headers_mut()
            .insert(SET_COOKIE, HeaderValue::from_str(&cookie).unwrap());
        Ok(response)
    } else {
        let error = ErrorResult {
            error: "UNAUTHORIZED".to_string(),
            message: "you shoundn't be here".to_string(),
        };
        Err((StatusCode::UNAUTHORIZED, Json(error)))
    }
}
