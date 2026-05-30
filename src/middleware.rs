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

    // Vérifier si le token existe, est valide et que le cookie associé est aussi valide
    match token {
        Some(token_str) => {
            if let Some(cookie) = state.security_store.verify_token(token_str) {
                // Vérifie aussi la validité du cookie associé
                if state.security_store.verify_cookie(&cookie).is_some() {
                    Ok(next.run(request).await)
                } else {
                    // Révoque le token si le cookie n'est plus valide
                    state.security_store.revoke_by_token(token_str);
                    let error = ErrorResult {
                        error: "UNAUTHORIZED".to_string(),
                        message: "Associated cookie invalid or expired".to_string(),
                    };
                    Err((StatusCode::UNAUTHORIZED, Json(error)))
                }
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

pub async fn verify_cookie_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, impl IntoResponse> {
    // Récupérer l'en-tête Cookie
    let cookie_header = request
        .headers()
        .get("cookie")
        .and_then(|v| v.to_str().ok());
    let mut unique_id: Option<&str> = None;
    if let Some(header) = cookie_header {
        // Chercher unique_id=... dans la chaîne de cookies
        for cookie in header.split(';') {
            let cookie = cookie.trim();
            if let Some(val) = cookie.strip_prefix("unique_id=") {
                unique_id = Some(val);
                break;
            }
        }
    }
    match unique_id {
        Some(cookie_val) => {
            if let Some(token) = state.security_store.verify_cookie(cookie_val) {
                // Vérifie aussi la validité du token associé
                if state.security_store.verify_token(&token).is_some() {
                    Ok(next.run(request).await)
                } else {
                    // Révoque le cookie si le token n'est plus valide
                    state.security_store.revoke_by_cookie(cookie_val);
                    let error = ErrorResult {
                        error: "UNAUTHORIZED".to_string(),
                        message: "Associated token invalid or expired".to_string(),
                    };
                    Err((StatusCode::UNAUTHORIZED, Json(error)))
                }
            } else {
                let error = ErrorResult {
                    error: "UNAUTHORIZED".to_string(),
                    message: "Invalid or expired cookie".to_string(),
                };
                Err((StatusCode::UNAUTHORIZED, Json(error)))
            }
        }
        None => {
            let error = ErrorResult {
                error: "UNAUTHORIZED".to_string(),
                message: "Missing unique_id cookie".to_string(),
            };
            Err((StatusCode::UNAUTHORIZED, Json(error)))
        }
    }
}
