use crate::{db::falidex::symbole_sens, state::AppState};
use axum::{
    Json,
    response::{IntoResponse, Response},
};
use axum::{extract::State, http::status::StatusCode};

pub async fn get(State(state): State<AppState>) -> Response {
    match symbole_sens::get(&state.db.db).await {
        Ok(symboles_sens) => Json(symboles_sens).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get: {e}"),
        )
            .into_response(),
    }
}
