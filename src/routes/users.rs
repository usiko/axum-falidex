use crate::db::model::{AuthRequest, ErrorResult, User, UserAuth};
use crate::state::AppState;
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
    let result = state
        .db
        .auth_user(payload.user_name, payload.password)
        .await
        .map_err(|e| {
            let status = if e == "not found" {
                StatusCode::UNAUTHORIZED
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            let error = ErrorResult {
                error: e,
                message: "unable to authenticate".to_string(),
            };
            (status, Json(error))
        })?;
    let id = result
        .id
        .map(|id: ObjectId| id.to_string())
        .unwrap_or_default();
    let token = generate_token(id.clone());
    println!("{}", token);
    Ok(Json(UserAuth {
        token,
        id,
        username: result.username,
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
pub async fn get_current_user(
    Claims(token): Claims<AppClaims>,
    State(state): State<AppState>,
) -> Result<Json<User>, (StatusCode, String)> {
    let user_id = token.sub;
    let result = state.db.get_user(user_id).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get user: {e}"),
        )
    })?;
    Ok(Json(result))
}

fn generate_token(user_id: String) -> String {
    println!("gen token from{}", user_id);
    //let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET non défini");
    let secret =
        std::env::var("JWT_SECRET").unwrap_or("default_dev_secret_please_change".to_string());
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
