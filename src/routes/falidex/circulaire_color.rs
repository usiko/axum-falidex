use crate::{db::falidex::circulaire_color, state::AppState};
use axum::{
    Json,
    response::{IntoResponse, Response},
};
use axum::{extract::State, http::status::StatusCode};

pub async fn get(State(state): State<AppState>) -> Response {
    match circulaire_color::get(&state.db.db).await {
        Ok(circulaires_colors) => Json(circulaires_colors).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get: {e}"),
        )
            .into_response(),
    }
}
