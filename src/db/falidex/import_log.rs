use crate::model::import_model::ImportCode;
use futures::stream::TryStreamExt;
use mongodb::{Collection, Database, bson::doc};

/// Nouvelle collection dédiée à la traçabilité des imports (QUE-68), distincte de la collection
/// générique `log` (qui ne conserve qu'une ligne de texte par action).
const COLLECTION: &str = "import-codes";

pub async fn record(db: &Database, code: &ImportCode) -> Result<(), String> {
    let col: Collection<ImportCode> = db.collection::<ImportCode>(COLLECTION);
    col.insert_one(code)
        .await
        .map(|_| ())
        .map_err(|e| format!("Erreur lors de l'enregistrement de l'import: {}", e))
}

pub async fn get(db: &Database) -> Result<Vec<ImportCode>, String> {
    let col: Collection<ImportCode> = db.collection::<ImportCode>(COLLECTION);
    let codes: Vec<ImportCode> = col
        .find(doc! {})
        .sort(doc! { "date": -1 })
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    Ok(codes)
}

pub async fn get_item(db: &Database, id: String) -> Result<ImportCode, String> {
    let col: Collection<ImportCode> = db.collection::<ImportCode>(COLLECTION);
    let result = col
        .find_one(doc! { "_id": id })
        .await
        .map_err(|e| e.to_string())?;
    result.ok_or("Import introuvable".to_string())
}
