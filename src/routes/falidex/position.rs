use crate::{db::falidex::get_positions, state::AppState};
use axum::{
    Json,
    response::{IntoResponse, Response},
};
use axum::{extract::State, http::status::StatusCode};

pub async fn get(State(state): State<AppState>) -> Response {
    match get_positions(&state.db.db).await {
        Ok(positions) => Json(positions).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get: {e}"),
        )
            .into_response(),
    }
}
