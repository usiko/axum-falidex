use crate::{db::falidex::import_draft, routes::users::AppClaims, state::AppState};
use axum::Json;
use axum::extract::State;
use axum::http::status::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_jwt::Claims;
use serde_json::{Value, json};

/// Sauvegarde (ou remplace) le brouillon d'import de l'utilisateur courant.
pub async fn save(
    Claims(token): Claims<AppClaims>,
    State(state): State<AppState>,
    Json(payload): Json<Value>,
) -> Response {
    match import_draft::save(&state.db.db, token.sub, payload).await {
        Ok(draft) => Json(draft).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "success": false, "error": e })),
        )
            .into_response(),
    }
}

/// Récupère le brouillon d'import de l'utilisateur courant, s'il y en a un.
pub async fn get(Claims(token): Claims<AppClaims>, State(state): State<AppState>) -> Response {
    match import_draft::get(&state.db.db, token.sub).await {
        Ok(Some(draft)) => Json(draft).into_response(),
        Ok(None) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "success": false, "error": e })),
        )
            .into_response(),
    }
}

/// Supprime le brouillon d'import de l'utilisateur courant (import appliqué ou annulé).
pub async fn delete(Claims(token): Claims<AppClaims>, State(state): State<AppState>) -> Response {
    match import_draft::clear(&state.db.db, token.sub).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "success": false, "error": e })),
        )
            .into_response(),
    }
}
