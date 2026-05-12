use crate::model::falidex_model::{CirculaireColor, CreateCirculaireColor, LinkDetail, OccurenceDetail};
use futures::stream::TryStreamExt;
use mongodb::{Collection, Database, bson::doc};

pub async fn get(db: &Database) -> Result<Vec<CirculaireColor>, String> {
    let col: Collection<CirculaireColor> = db.collection::<CirculaireColor>("circulaires-colors");
    let circulaires_colors: Vec<CirculaireColor> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    Ok(circulaires_colors)
}

pub async fn create(db: &Database, data: CreateCirculaireColor) -> Result<String, String> {
    let col: Collection<CreateCirculaireColor> = db.collection::<CreateCirculaireColor>("circulaires-colors");
    col.insert_one(data)
        .await
        .map(|_| "CirculaireColor créée avec succès".to_string())
        .map_err(|e| format!("Erreur lors de la création: {}", e))
}

pub async fn update(db: &Database, id: String, data: CirculaireColor) -> Result<String, String> {
    let col: Collection<CirculaireColor> = db.collection::<CirculaireColor>("circulaires-colors");
    let result = col
        .replace_one(doc! { "_id": id }, data)
        .await
        .map_err(|e| format!("Erreur lors de la mise à jour: {}", e))?;

    if result.modified_count > 0 {
        Ok("CirculaireColor mise à jour avec succès".to_string())
    } else {
        Err("Aucune circulaire color trouvée avec cet identifiant".to_string())
    }
}

pub async fn delete(db: &Database, id: String) -> Result<String, String> {
    // Vérifier les occurrences dans les relations
    let occurences = get_occurences(db, id.clone()).await?;

    if !occurences.is_empty() {
        let total_items: u64 = occurences.iter().map(|o| o.items).sum();
        return Err(format!(
            "Impossible de supprimer cette circulaire color car elle est utilisée dans {} relation(s) ({} item(s) au total)",
            occurences.len(),
            total_items
        ));
    }

    let col: Collection<CirculaireColor> = db.collection::<CirculaireColor>("circulaires-colors");
    let result = col
        .delete_one(doc! { "_id": id })
        .await
        .map_err(|e| format!("Erreur lors de la suppression: {}", e))?;

    if result.deleted_count > 0 {
        Ok("CirculaireColor supprimée avec succès".to_string())
    } else {
        Err("Aucune circulaire color trouvée avec cet identifiant".to_string())
    }
}

pub async fn get_occurences(
    db: &Database,
    circulaire_id: String,
) -> Result<Vec<OccurenceDetail>, String> {
    let col: Collection<LinkDetail> = db.collection::<LinkDetail>("links");

    // Récupérer toutes les relations
    let all_relations: Vec<LinkDetail> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;

    let mut occurences = Vec::new();

    for relation in all_relations {
        // Compter le nombre d'items qui référencent cette circulaire
        let items_count = relation
            .relations
            .iter()
            .filter(|item| {
                item.circulaire_id
                    .as_ref()
                    .map_or(false, |id| id == &circulaire_id)
            })
            .count() as u64;

        // Si au moins un item référence cette circulaire, ajouter à la liste
        if items_count > 0 {
            occurences.push(OccurenceDetail {
                relation: relation.name,
                items: items_count,
            });
        }
    }

    Ok(occurences)
}
