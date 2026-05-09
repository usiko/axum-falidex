use crate::model::falidex_model::Filiere;
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};

pub async fn get(db: &Database) -> Result<Vec<Filiere>, String> {
    let col: Collection<Filiere> = db.collection::<Filiere>("filieres");
    let filieres: Vec<Filiere> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    Ok(filieres)
}

pub async fn create(db: &Database, data: Filiere) -> Result<String, String> {
    let col: Collection<Filiere> = db.collection::<Filiere>("filieres");
    col.insert_one(data)
        .await
        .map(|_| "Filiere créée avec succès".to_string())
        .map_err(|e| format!("Erreur lors de la création: {}", e))
}

pub async fn update(db: &Database, id: String, data: Filiere) -> Result<String, String> {
    let col: Collection<Filiere> = db.collection::<Filiere>("filieres");
    let result = col
        .replace_one(doc! { "_id": id }, data)
        .await
        .map_err(|e| format!("Erreur lors de la mise à jour: {}", e))?;

    if result.modified_count > 0 {
        Ok("Filiere mise à jour avec succès".to_string())
    } else {
        Err("Aucune filiere trouvée avec cet identifiant".to_string())
    }
}

pub async fn delete(db: &Database, id: String) -> Result<String, String> {
    let col: Collection<Filiere> = db.collection::<Filiere>("filieres");
    let result = col
        .delete_one(doc! { "_id": id })
        .await
        .map_err(|e| format!("Erreur lors de la suppression: {}", e))?;

    if result.deleted_count > 0 {
        Ok("Filiere supprimée avec succès".to_string())
    } else {
        Err("Aucune filiere trouvée avec cet identifiant".to_string())
    }
}
