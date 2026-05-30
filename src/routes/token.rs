use axum::{
    Json,
    extract::State,
    http::HeaderValue,
    response::{IntoResponse, Response},
};

use reqwest::{StatusCode, header::SET_COOKIE};

use crate::{
    db::model::{ErrorResult, TokenAuth, TokenAuthResponse},
    security_utils::verify_temp_token,
    state::AppState,
};
pub async fn verify_hash(
    State(state): State<AppState>,
    Json(payload): Json<TokenAuth>,
) -> Result<Response, (StatusCode, Json<ErrorResult>)> {
    if verify_temp_token(&payload.role, payload.timestamp, &payload.hash) {
        let autorisation = state.security_store.generate_and_store();
        let cookie = format!(
            "Set-Cookie: unique_id={}; Path=/; HttpOnly; SameSite=None; Secure",
            autorisation.cookie
        );
        let mut response = Json(TokenAuthResponse {
            token: autorisation.token,
        })
        .into_response();
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
