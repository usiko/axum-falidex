use crate::db::model::{AuthRequest, ErrorResult, User, UserAuth};
use crate::{db::user, state::AppState};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use axum_jwt::Claims;
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppClaims {
    pub sub: String, // user_id
    exp: usize,      // expiration
}

pub async fn auth(
    State(state): State<AppState>,
    Json(payload): Json<AuthRequest>,
) -> Result<Json<UserAuth>, (StatusCode, Json<ErrorResult>)> {
    let db = &state.db.db;
    user::auth(db, payload.user_name, payload.password)
        .await
        .map(|result| {
            let id = result
                .id
                .map(|id: ObjectId| id.to_string())
                .unwrap_or_default();
            let token = generate_token(id.clone());
            Json(UserAuth {
                token,
                id,
                username: result.username,
            })
        })
        .map_err(|e| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResult {
                    error: "Unauthorized".to_string(),
                    message: e.to_string(),
                }),
            )
        })
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<Json<User>, (StatusCode, String)> {
    print!("req user with id {}", user_id);
    let result = user::get_by_id(&state.db.db, user_id).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get user: {e}"),
        )
    })?;
    Ok(Json(result))
}
pub async fn get_current_user(
    Claims(token): Claims<AppClaims>,
    State(state): State<AppState>,
) -> Result<Json<User>, (StatusCode, String)> {
    let user_id = token.sub;
    let result = user::get_by_id(&state.db.db, user_id).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get user: {e}"),
        )
    })?;
    Ok(Json(result))
}

fn generate_token(user_id: String) -> String {
    println!("gen token from{}", user_id);
    let secret = crate::env::get_jwt_secret();
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24)) // durée du token
        .unwrap()
        .timestamp() as usize;

    let claims = AppClaims {
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
