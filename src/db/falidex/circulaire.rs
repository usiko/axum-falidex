use crate::model::falidex_model::{Circulaire, CirculaireColor};
use futures::stream::TryStreamExt;
use mongodb::{Collection, Database, bson::doc};

pub async fn get(db: &Database) -> Result<Vec<Circulaire>, String> {
    let col: Collection<Circulaire> = db.collection::<Circulaire>("circulaires");
    let circulaires: Vec<Circulaire> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    Ok(circulaires)
}
pub async fn create(db: &Database, data: Circulaire) -> Result<String, String> {
    let col: Collection<Circulaire> = db.collection::<Circulaire>("circulaires");
    col.insert_one(data)
        .await
        .map(|_| "Circulaire créée avec succès".to_string())
        .map_err(|e| format!("Erreur lors de la création: {}", e))
}

pub async fn update(db: &Database, id: String, data: Circulaire) -> Result<String, String> {
    let col: Collection<Circulaire> = db.collection::<Circulaire>("circulaires");
    let result = col
        .replace_one(doc! { "_id": id }, data)
        .await
        .map_err(|e| format!("Erreur lors de la mise à jour: {}", e))?;

    if result.modified_count > 0 {
        Ok("Circulaire mise à jour avec succès".to_string())
    } else {
        Err("Aucune circulaire trouvée avec cet identifiant".to_string())
    }
}

pub async fn delete(db: &Database, id: String) -> Result<String, String> {
    // D'abord supprimer les circulaire_color associés
    let col_colors: Collection<CirculaireColor> =
        db.collection::<CirculaireColor>("circulaires-colors");
    col_colors
        .delete_many(doc! { "circulaireId": &id })
        .await
        .map_err(|e| {
            format!(
                "Erreur lors de la suppression des circulaire_color associés: {}",
                e
            )
        })?;

    // Ensuite supprimer la circulaire
    let col: Collection<Circulaire> = db.collection::<Circulaire>("circulaires");
    let result = col
        .delete_one(doc! { "_id": id })
        .await
        .map_err(|e| format!("Erreur lors de la suppression: {}", e))?;

    if result.deleted_count > 0 {
        Ok("Circulaire supprimée avec succès".to_string())
    } else {
        Err("Aucune circulaire trouvée avec cet identifiant".to_string())
    }
}
