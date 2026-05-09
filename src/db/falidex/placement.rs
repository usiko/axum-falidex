use crate::model::falidex_model::Placement;
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};

pub async fn get(db: &Database) -> Result<Vec<Placement>, String> {
    let col: Collection<Placement> = db.collection::<Placement>("placements");
    let placements: Vec<Placement> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    Ok(placements)
}

pub async fn create(db: &Database, data: Placement) -> Result<String, String> {
    let col: Collection<Placement> = db.collection::<Placement>("placements");
    col.insert_one(data)
        .await
        .map(|_| "Placement créé avec succès".to_string())
        .map_err(|e| format!("Erreur lors de la création: {}", e))
}

pub async fn update(db: &Database, id: String, data: Placement) -> Result<String, String> {
    let col: Collection<Placement> = db.collection::<Placement>("placements");
    let result = col
        .replace_one(doc! { "_id": id }, data)
        .await
        .map_err(|e| format!("Erreur lors de la mise à jour: {}", e))?;

    if result.modified_count > 0 {
        Ok("Placement mis à jour avec succès".to_string())
    } else {
        Err("Aucun placement trouvé avec cet identifiant".to_string())
    }
}

pub async fn delete(db: &Database, id: String) -> Result<String, String> {
    let col: Collection<Placement> = db.collection::<Placement>("placements");
    let result = col
        .delete_one(doc! { "_id": id })
        .await
        .map_err(|e| format!("Erreur lors de la suppression: {}", e))?;

    if result.deleted_count > 0 {
        Ok("Placement supprimé avec succès".to_string())
    } else {
        Err("Aucun placement trouvé avec cet identifiant".to_string())
    }
}
