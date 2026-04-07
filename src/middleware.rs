use crate::db::model::ErrorResult;
use crate::state::AppState;
use axum::{
    Json,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

/// Middleware qui vérifie le token custom dans le header X-Token
pub async fn verify_token_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    // Récupérer le header X-Token
    let token = request
        .headers()
        .get("X-Token")
        .and_then(|value| value.to_str().ok());

    // Vérifier si le token existe et est valide
    match token {
        Some(token_str) => {
            if state.token_store.verify_stored_token(token_str) {
                Ok(next.run(request).await)
            } else {
                let error = ErrorResult {
                    error: "UNAUTHORIZED".to_string(),
                    message: "Invalid or expired token".to_string(),
                };
                Err((StatusCode::UNAUTHORIZED, Json(error)))
            }
        }
        None => {
            let error = ErrorResult {
                error: "UNAUTHORIZED".to_string(),
                message: "Missing X-Token header".to_string(),
            };
            Err((StatusCode::UNAUTHORIZED, Json(error)))
        }
    }
}
