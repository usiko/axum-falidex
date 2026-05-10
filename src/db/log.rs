use crate::model::falidex_model::Log;
use chrono::Utc;
use futures::stream::TryStreamExt;
use mongodb::{Collection, Database, bson::doc};

pub async fn get(db: &Database) -> Result<Vec<Log>, String> {
    let col: Collection<Log> = db.collection::<Log>("log");
    let logs: Vec<Log> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    Ok(logs)
}

pub async fn add(db: &Database, user_id: String, info: String) -> Result<String, String> {
    let col: Collection<Log> = db.collection::<Log>("log");

    let log = Log {
        id: uuid::Uuid::new_v4().to_string(),
        date: Utc::now().to_rfc3339(),
        user_id,
        info,
    };

    col.insert_one(log)
        .await
        .map(|_| "Log créé avec succès".to_string())
        .map_err(|e| format!("Erreur lors de la création du log: {}", e))
}
