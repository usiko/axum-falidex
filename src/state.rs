use axum::extract::FromRef;
use axum_jwt::{Decoder, jsonwebtoken::DecodingKey};
use uuid::Uuid;

use crate::db::mongo::MongoDB;
use crate::security_utils::SecurityStore;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<MongoDB>,
    pub jwt_decoder: Decoder,
    pub security_store: Arc<SecurityStore>,
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
        security_store: Arc::new(SecurityStore::new()),
    }
}

fn get_jwt_decoder() -> Decoder {
    let secret = crate::env::get_jwt_secret();
    Decoder::from_key(DecodingKey::from_secret(secret.as_bytes()))
}
