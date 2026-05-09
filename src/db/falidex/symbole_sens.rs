use crate::model::falidex_model::SymboleSens;
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};

pub async fn get(db: &Database) -> Result<Vec<SymboleSens>, String> {
    let col: Collection<SymboleSens> = db.collection::<SymboleSens>("symboles-sens");
    let symboles_sens: Vec<SymboleSens> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    Ok(symboles_sens)
}

pub async fn create(db: &Database, data: SymboleSens) -> Result<String, String> {
    let col: Collection<SymboleSens> = db.collection::<SymboleSens>("symboles-sens");
    col.insert_one(data)
        .await
        .map(|_| "Symbole sens créé avec succès".to_string())
        .map_err(|e| format!("Erreur lors de la création: {}", e))
}

pub async fn update(db: &Database, id: String, data: SymboleSens) -> Result<String, String> {
    let col: Collection<SymboleSens> = db.collection::<SymboleSens>("symboles-sens");
    let result = col
        .replace_one(doc! { "_id": id }, data)
        .await
        .map_err(|e| format!("Erreur lors de la mise à jour: {}", e))?;

    if result.modified_count > 0 {
        Ok("Symbole sens mis à jour avec succès".to_string())
    } else {
        Err("Aucun symbole sens trouvé avec cet identifiant".to_string())
    }
}

pub async fn delete(db: &Database, id: String) -> Result<String, String> {
    let col: Collection<SymboleSens> = db.collection::<SymboleSens>("symboles-sens");
    let result = col
        .delete_one(doc! { "_id": id })
        .await
        .map_err(|e| format!("Erreur lors de la suppression: {}", e))?;

    if result.deleted_count > 0 {
        Ok("Symbole sens supprimé avec succès".to_string())
    } else {
        Err("Aucun symbole sens trouvé avec cet identifiant".to_string())
    }
}
