use crate::model::falidex_model::SymboleAccessoire;
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};

pub async fn get(db: &Database) -> Result<Vec<SymboleAccessoire>, String> {
    let col: Collection<SymboleAccessoire> = db.collection::<SymboleAccessoire>("symboles-accessories");
    let symbole_accessoires: Vec<SymboleAccessoire> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    Ok(symbole_accessoires)
}

pub async fn create(db: &Database, data: SymboleAccessoire) -> Result<String, String> {
    let col: Collection<SymboleAccessoire> = db.collection::<SymboleAccessoire>("symboles-accessories");
    col.insert_one(data)
        .await
        .map(|_| "Symbole accessoire créé avec succès".to_string())
        .map_err(|e| format!("Erreur lors de la création: {}", e))
}

pub async fn update(db: &Database, id: String, data: SymboleAccessoire) -> Result<String, String> {
    let col: Collection<SymboleAccessoire> = db.collection::<SymboleAccessoire>("symboles-accessories");
    let result = col
        .replace_one(doc! { "_id": id }, data)
        .await
        .map_err(|e| format!("Erreur lors de la mise à jour: {}", e))?;

    if result.modified_count > 0 {
        Ok("Symbole accessoire mis à jour avec succès".to_string())
    } else {
        Err("Aucun symbole accessoire trouvé avec cet identifiant".to_string())
    }
}

pub async fn delete(db: &Database, id: String) -> Result<String, String> {
    let col: Collection<SymboleAccessoire> = db.collection::<SymboleAccessoire>("symboles-accessories");
    let result = col
        .delete_one(doc! { "_id": id })
        .await
        .map_err(|e| format!("Erreur lors de la suppression: {}", e))?;

    if result.deleted_count > 0 {
        Ok("Symbole accessoire supprimé avec succès".to_string())
    } else {
        Err("Aucun symbole accessoire trouvé avec cet identifiant".to_string())
    }
}
