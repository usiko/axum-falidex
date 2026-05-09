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
    // Vérifier les occurrences (circulaire_color + relations)
    let occurences = get_occurences(db, id.clone()).await?;
    
    if !occurences.is_empty() {
        let total_items: u64 = occurences.iter().map(|o| o.items).sum();
        return Err(format!(
            "Impossible de supprimer cette color car elle est utilisée dans {} relation(s) ({} item(s) au total)",
            occurences.len(),
            total_items
        ));
    }

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

pub async fn get_occurences(db: &Database, id: String) -> Result<Vec<OccurenceDetail>, String> {
    // Étape 1: Trouver tous les CirculaireColor qui contiennent cette color
    let col_circulaire_colors: Collection<CirculaireColor> =
        db.collection::<CirculaireColor>("circulaires-colors");

    let circulaire_colors: Vec<CirculaireColor> = col_circulaire_colors
        .find(doc! { "colorIds": &id })
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;

    // Étape 2: Extraire tous les circulaire_ids
    let circulaire_ids: Vec<String> = circulaire_colors
        .iter()
        .map(|cc| cc.circulaire_id.clone())
        .collect();

    // Si aucun circulaire n'utilise cette color, retourner une liste vide
    if circulaire_ids.is_empty() {
        return Ok(Vec::new());
    }

    // Étape 3: Chercher dans LinkDetail toutes les relations
    use crate::model::falidex_model::LinkDetail;
    let col: Collection<LinkDetail> = db.collection::<LinkDetail>("links");
    let all_relations: Vec<LinkDetail> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;

    let mut occurences = Vec::new();

    // Étape 4: Pour chaque relation, compter les items qui utilisent un circulaire avec cette color
    for relation in all_relations {
        let items_count = relation
            .relations
            .iter()
            .filter(|item| {
                item.circulaire_id
                    .as_ref()
                    .map_or(false, |cid| circulaire_ids.contains(cid))
            })
            .count() as u64;

        if items_count > 0 {
            occurences.push(OccurenceDetail {
                relation: relation.name,
                items: items_count,
            });
        }
    }

    Ok(occurences)
}
