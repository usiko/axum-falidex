use crate::db::model::{AuthRequest, User, UserAuth};
use crate::state::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthToken {
    access_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // user_id
    exp: usize,  // expiration
}

pub async fn auth(
    State(state): State<AppState>,
    Json(payload): Json<AuthRequest>,
) -> Result<Json<AuthToken>, (StatusCode, String)> {
    let result = state
        .db
        .auth_user(payload.user_name, payload.hash)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to auth: {e}"),
            )
        })?;
    Ok(Json(AuthToken {
        access_token: generate_token(result.id),
    }))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<Json<User>, (StatusCode, String)> {
    print!("req user with id {}", user_id);
    let result = state.db.get_user(user_id).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get user: {e}"),
        )
    })?;
    Ok(Json(result))
}

fn generate_token(user_id: String) -> String {
    //let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET non défini");
    let secret =
        std::env::var("JWT_SECRET").unwrap_or("default_dev_secret_please_change".to_string());
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24)) // durée du token
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap()
}
