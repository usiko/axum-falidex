use crate::model::falidex_model::{CreatePosition, LinkDetail, OccurenceDetail, Position};
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};

pub async fn get(db: &Database) -> Result<Vec<Position>, String> {
    let col: Collection<Position> = db.collection::<Position>("positions");
    let positions: Vec<Position> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    Ok(positions)
}

pub async fn create(db: &Database, user_id: String, data: CreatePosition) -> Result<String, String> {
    let _ = crate::db::log::add(db, user_id, "[falidex][position][create]".to_string()).await;
    let col: Collection<CreatePosition> = db.collection::<CreatePosition>("positions");
    col.insert_one(data)
        .await
        .map(|_| "Position créée avec succès".to_string())
        .map_err(|e| format!("Erreur lors de la création: {}", e))
}

pub async fn update(db: &Database, user_id: String, id: String, data: Position) -> Result<String, String> {
    let _ = crate::db::log::add(db, user_id, format!("[falidex][position][update] id: {}", id)).await;
    let col: Collection<Position> = db.collection::<Position>("positions");
    let result = col
        .replace_one(doc! { "_id": id }, data)
        .await
        .map_err(|e| format!("Erreur lors de la mise à jour: {}", e))?;

    if result.modified_count > 0 {
        Ok("Position mise à jour avec succès".to_string())
    } else {
        Err("Aucune position trouvée avec cet identifiant".to_string())
    }
}

pub async fn delete(db: &Database, user_id: String, id: String) -> Result<String, String> {
    let _ = crate::db::log::add(db, user_id, format!("[falidex][position][delete] id: {}", id)).await;
    // Vérifier les occurrences dans les relations
    let occurences = get_occurences(db, id.clone()).await?;
    
    if !occurences.is_empty() {
        let total_items: u64 = occurences.iter().map(|o| o.items).sum();
        return Err(format!(
            "Impossible de supprimer cette position car elle est utilisée dans {} relation(s) ({} item(s) au total)",
            occurences.len(),
            total_items
        ));
    }

    let col: Collection<Position> = db.collection::<Position>("positions");
    let result = col
        .delete_one(doc! { "_id": id })
        .await
        .map_err(|e| format!("Erreur lors de la suppression: {}", e))?;

    if result.deleted_count > 0 {
        Ok("Position supprimée avec succès".to_string())
    } else {
        Err("Aucune position trouvée avec cet identifiant".to_string())
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
                item.position_id
                    .as_ref()
                    .map_or(false, |position_id| position_id == &id)
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
