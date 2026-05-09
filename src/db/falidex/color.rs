use crate::model::falidex_model::{CirculaireColor, Color, OccurenceDetail};
use futures::stream::TryStreamExt;
use mongodb::{Collection, Database, bson::doc};

pub async fn get(db: &Database) -> Result<Vec<Color>, String> {
    let col: Collection<Color> = db.collection::<Color>("colors");
    let colors: Vec<Color> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    Ok(colors)
}
pub async fn create(db: &Database, data: Color) -> Result<String, String> {
    let col: Collection<Color> = db.collection::<Color>("colors");
    col.insert_one(data)
        .await
        .map(|_| "Color créée avec succès".to_string())
        .map_err(|e| format!("Erreur lors de la création: {}", e))
}

pub async fn update(db: &Database, id: String, data: Color) -> Result<String, String> {
    let col: Collection<Color> = db.collection::<Color>("colors");
    let result = col
        .replace_one(doc! { "_id": id }, data)
        .await
        .map_err(|e| format!("Erreur lors de la mise à jour: {}", e))?;

    if result.modified_count > 0 {
        Ok("Color mise à jour avec succès".to_string())
    } else {
        Err("Aucune color trouvée avec cet identifiant".to_string())
    }
}

pub async fn delete(db: &Database, id: String) -> Result<String, String> {
    // Vérifier qu'aucun circulaire_color n'utilise cette color
    let col_colors: Collection<CirculaireColor> =
        db.collection::<CirculaireColor>("circulaires-colors");
    let count = col_colors
        .count_documents(doc! { "colorIds": &id })
        .await
        .map_err(|e| format!("Erreur lors de la vérification: {}", e))?;

    if count > 0 {
        return Err(format!(
            "Impossible de supprimer cette color car elle est utilisée par {} circulaire(s)",
            count
        ));
    }

    // Supprimer la color
    let col: Collection<Color> = db.collection::<Color>("colors");
    let result = col
        .delete_one(doc! { "_id": id })
        .await
        .map_err(|e| format!("Erreur lors de la suppression: {}", e))?;

    if result.deleted_count > 0 {
        Ok("Color supprimée avec succès".to_string())
    } else {
        Err("Aucune color trouvée avec cet identifiant".to_string())
    }
}

pub async fn get_occurences(_db: &Database, _id: String) -> Result<Vec<OccurenceDetail>, String> {
    // Note: Color n'est pas directement référencé dans LinkItem
    // Il faudrait chercher via circulaire -> circulaireColor -> color
    // Pour l'instant, retourne une liste vide
    Ok(Vec::new())
}
