use crate::db::mongo::MongoDB;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<MongoDB>,
}

pub async fn get_state() -> AppState {
    let db = MongoDB::new().await.unwrap();
    AppState { db: Arc::new(db) }
}
