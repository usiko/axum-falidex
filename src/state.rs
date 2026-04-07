use axum::extract::FromRef;
use axum_jwt::{Decoder, jsonwebtoken::DecodingKey};

use crate::db::mongo::MongoDB;
use crate::token::TokenStore;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<MongoDB>,
    pub jwt_decoder: Decoder,
    pub token_store: Arc<TokenStore>,
}

impl FromRef<AppState> for Decoder {
    fn from_ref(state: &AppState) -> Self {
        state.jwt_decoder.clone()
    }
}

pub async fn get_state() -> AppState {
    let db = MongoDB::new().await.unwrap();
    AppState {
        db: Arc::new(db),
        jwt_decoder: get_jwt_decoder(),
        token_store: Arc::new(TokenStore::new()),
    }
}

fn get_jwt_decoder() -> Decoder {
    let secret =
        std::env::var("JWT_SECRET").unwrap_or("default_dev_secret_please_change".to_string());
    Decoder::from_key(DecodingKey::from_secret(secret.as_bytes()))
}
