use chrono::Utc;
use mongodb::options::ReplaceOptions;
use mongodb::{Collection, Database, bson::doc};
use serde_json::Value;

use crate::model::import_model::ImportDraft;

/// Un seul brouillon actif par utilisateur : `_id` = `user_id`, une sauvegarde remplace toujours
/// la précédente (upsert).
const COLLECTION: &str = "import-drafts";

fn get_collection(db: &Database) -> Collection<ImportDraft> {
    db.collection(COLLECTION)
}

pub async fn save(db: &Database, user_id: String, payload: Value) -> Result<ImportDraft, String> {
    let draft = ImportDraft {
        user_id: user_id.clone(),
        saved_at: Utc::now().to_rfc3339(),
        payload,
    };

    let col = get_collection(db);
    col.replace_one(doc! { "_id": &user_id }, &draft)
        .with_options(ReplaceOptions::builder().upsert(true).build())
        .await
        .map_err(|e| format!("Erreur lors de la sauvegarde du brouillon d'import: {e}"))?;

    Ok(draft)
}

pub async fn get(db: &Database, user_id: String) -> Result<Option<ImportDraft>, String> {
    get_collection(db)
        .find_one(doc! { "_id": user_id })
        .await
        .map_err(|e| format!("Erreur lors de la récupération du brouillon d'import: {e}"))
}

pub async fn clear(db: &Database, user_id: String) -> Result<(), String> {
    get_collection(db)
        .delete_one(doc! { "_id": user_id })
        .await
        .map(|_| ())
        .map_err(|e| format!("Erreur lors de la suppression du brouillon d'import: {e}"))
}
