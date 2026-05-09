use crate::{db::falidex::symbole_accessoire, state::AppState};
use axum::{
    Json,
    response::{IntoResponse, Response},
};
use axum::{extract::State, http::status::StatusCode};

pub async fn get(State(state): State<AppState>) -> Response {
    match symbole_accessoire::get(&state.db.db).await {
        Ok(symbole_accessoires) => Json(symbole_accessoires).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get: {e}"),
        )
            .into_response(),
    }
}
