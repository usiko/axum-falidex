use crate::model::falidex_model::{LinkDetail, OccurenceDetail, Signification};
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
    let _ = crate::db::log::add(db, "system".to_string(), "[falidex][signification][create]".to_string()).await;
    let col: Collection<Signification> = db.collection::<Signification>("significations");
    col.insert_one(data)
        .await
        .map(|_| "Signification créée avec succès".to_string())
        .map_err(|e| format!("Erreur lors de la création: {}", e))
}

pub async fn update(db: &Database, id: String, data: Signification) -> Result<String, String> {
    let _ = crate::db::log::add(db, "system".to_string(), format!("[falidex][signification][update] id: {}", id)).await;
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
    let _ = crate::db::log::add(db, "system".to_string(), format!("[falidex][signification][delete] id: {}", id)).await;
    // Vérifier les occurrences dans les relations
    let occurences = get_occurences(db, id.clone()).await?;
    
    if !occurences.is_empty() {
        let total_items: u64 = occurences.iter().map(|o| o.items).sum();
        return Err(format!(
            "Impossible de supprimer cette signification car elle est utilisée dans {} relation(s) ({} item(s) au total)",
            occurences.len(),
            total_items
        ));
    }

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

pub async fn get_occurences(db: &Database, id: String) -> Result<Vec<OccurenceDetail>, String> {
    let col: Collection<LinkDetail> = db.collection::<LinkDetail>("links");

    let all_relations: Vec<LinkDetail> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;

    let mut occurences = Vec::new();

    for relation in all_relations {
        let items_count = relation
            .relations
            .iter()
            .filter(|item| {
                item.signification_id
                    .as_ref()
                    .map_or(false, |signification_id| signification_id == &id)
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
