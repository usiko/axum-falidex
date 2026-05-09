use crate::db::model::ErrorResult;
use crate::state::AppState;
use axum::{
    Json,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{DecodingKey, Validation, decode, errors::ErrorKind};
use serde_json::Value;

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

/// Middleware qui vérifie le JWT token dans le header Authorization
pub async fn verify_jwt_middleware(
    State(_state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    // Récupérer le header Authorization
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok());

    match auth_header {
        Some(auth_value) => {
            // Vérifier le format "Bearer <token>"
            if let Some(token) = auth_value.strip_prefix("Bearer ") {
                // Récupérer la clé secrète JWT
                let secret = crate::env::get_jwt_secret();
                let decoding_key = DecodingKey::from_secret(secret.as_bytes());

                // Décoder le token JWT
                match decode::<Value>(token, &decoding_key, &Validation::default()) {
                    Ok(_claims) => {
                        // Token valide, continuer
                        Ok(next.run(request).await)
                    }
                    Err(e) => {
                        // Analyser le type d'erreur JWT pour un message spécifique
                        let (error_type, message) = match e.kind() {
                            ErrorKind::ExpiredSignature => (
                                "JWT_EXPIRED",
                                "Le token JWT a expiré. Veuillez vous reconnecter.",
                            ),
                            ErrorKind::InvalidToken => {
                                ("JWT_INVALID", "Le token JWT est invalide ou malformé.")
                            }
                            ErrorKind::InvalidSignature => (
                                "JWT_INVALID_SIGNATURE",
                                "La signature du token JWT est invalide.",
                            ),
                            ErrorKind::InvalidIssuer => (
                                "JWT_INVALID_ISSUER",
                                "L'émetteur du token JWT n'est pas reconnu.",
                            ),
                            ErrorKind::InvalidAudience => (
                                "JWT_INVALID_AUDIENCE",
                                "L'audience du token JWT n'est pas valide.",
                            ),
                            _ => ("JWT_ERROR", "Erreur lors de la validation du token JWT."),
                        };

                        let error = ErrorResult {
                            error: error_type.to_string(),
                            message: message.to_string(),
                        };
                        Err((StatusCode::UNAUTHORIZED, Json(error)))
                    }
                }
            } else {
                let error = ErrorResult {
                    error: "JWT_MALFORMED_HEADER".to_string(),
                    message: "Le header Authorization doit commencer par 'Bearer '.".to_string(),
                };
                Err((StatusCode::UNAUTHORIZED, Json(error)))
            }
        }
        None => {
            let error = ErrorResult {
                error: "JWT_MISSING".to_string(),
                message: "Le header Authorization avec le token JWT est manquant.".to_string(),
            };
            Err((StatusCode::UNAUTHORIZED, Json(error)))
        }
    }
}
