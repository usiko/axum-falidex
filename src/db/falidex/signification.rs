use crate::model::falidex_model::Signification;
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};

pub async fn get(db: &Database) -> Result<Vec<Signification>, String> {
    let col: Collection<Signification> = db.collection::<Signification>("significations");
    let significations: Vec<Signification> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    Ok(significations)
}

pub async fn create(db: &Database, data: Signification) -> Result<String, String> {
    let col: Collection<Signification> = db.collection::<Signification>("significations");
    col.insert_one(data)
        .await
        .map(|_| "Signification créée avec succès".to_string())
        .map_err(|e| format!("Erreur lors de la création: {}", e))
}

pub async fn update(db: &Database, id: String, data: Signification) -> Result<String, String> {
    let col: Collection<Signification> = db.collection::<Signification>("significations");
    let result = col
        .replace_one(doc! { "_id": id }, data)
        .await
        .map_err(|e| format!("Erreur lors de la mise à jour: {}", e))?;

    if result.modified_count > 0 {
        Ok("Signification mise à jour avec succès".to_string())
    } else {
        Err("Aucune signification trouvée avec cet identifiant".to_string())
    }
}

pub async fn delete(db: &Database, id: String) -> Result<String, String> {
    let col: Collection<Signification> = db.collection::<Signification>("significations");
    let result = col
        .delete_one(doc! { "_id": id })
        .await
        .map_err(|e| format!("Erreur lors de la suppression: {}", e))?;

    if result.deleted_count > 0 {
        Ok("Signification supprimée avec succès".to_string())
    } else {
        Err("Aucune signification trouvée avec cet identifiant".to_string())
    }
}
