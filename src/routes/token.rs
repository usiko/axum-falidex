use axum::{
    Json,
    extract::State,
    http::HeaderValue,
    response::{IntoResponse, Response},
};

use reqwest::{StatusCode, header::SET_COOKIE};

use crate::{
    db::model::{ErrorResult, TokenAuth, TokenAuthResponse},
    env,
    security_utils::{SecurityStoreAutorization, verify_temp_token},
    state::AppState,
};
pub async fn verify_hash(
    State(state): State<AppState>,
    Json(payload): Json<TokenAuth>,
) -> Result<Response, (StatusCode, Json<ErrorResult>)> {
    if verify_temp_token(&payload.role, payload.timestamp, &payload.hash) {
        let autorisation = state.security_store.generate_and_store();
        let cookie = gen_cookie(autorisation.clone());
        let mut response = Json(TokenAuthResponse {
            token: autorisation.token,
        })
        .into_response();
        let cookie_header = HeaderValue::from_str(&cookie).map_err(|_| {
            let error = ErrorResult {
                error: "INTERNAL_SERVER_ERROR".to_string(),
                message: "Failed to build Set-Cookie header".to_string(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error))
        })?;
        response.headers_mut().insert(SET_COOKIE, cookie_header);
        Ok(response)
    } else {
        let error = ErrorResult {
            error: "UNAUTHORIZED".to_string(),
            message: "you shoundn't be here".to_string(),
        };
        Err((StatusCode::UNAUTHORIZED, Json(error)))
    }
}

fn gen_cookie(autorisation: SecurityStoreAutorization) -> String {
    let domain = env::domain();
    if domain != "" {
        format!(
            "unique_id={}; Path=/; HttpOnly; SameSite=Lax; Secure; Domain={}",
            autorisation.cookie, domain
        )
    } else {
        format!(
            "unique_id={}; Path=/; HttpOnly; SameSite=Lax;",
            autorisation.cookie
        )
    }
}
