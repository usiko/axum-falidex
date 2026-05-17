use crate::model::falidex_model::{CreatePlacement, LinkDetail, OccurenceDetail, Placement, UpdatePlacement};
use futures::stream::TryStreamExt;
use mongodb::bson;
use mongodb::{Collection, Database, bson::doc};
use super::update_helper;

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

pub async fn create(db: &Database, user_id: String, data: CreatePlacement) -> Result<String, String> {
    let _ = crate::db::log::add(db, user_id, "[falidex][placement][create]".to_string()).await;
    let col: Collection<CreatePlacement> = db.collection::<CreatePlacement>("placements");
    col.insert_one(data)
        .await
        .map(|_| "Placement créé avec succès".to_string())
        .map_err(|e| format!("Erreur lors de la création: {}", e))
}

pub async fn update(db: &Database, user_id: String, id: String, data: UpdatePlacement) -> Result<String, String> {
    let _ = crate::db::log::add(db, user_id, format!("[falidex][placement][update] id: {}", id)).await;
    let col: Collection<Placement> = db.collection::<Placement>("placements");

    let mut update_doc = doc! {};
    if let Some(name) = data.name {
        update_doc.insert("name", name);
    }

    let (matched, modified) = update_helper::partial_update(&col, &id, update_doc).await?;

    if matched == 0 {
        Err("Aucun placement trouvé avec cet identifiant".to_string())
    } else if modified > 0 {
        Ok("Placement mis à jour avec succès".to_string())
    } else {
        Ok("Aucune modification détectée (valeurs identiques)".to_string())
    }
}

pub async fn delete(db: &Database, user_id: String, id: String) -> Result<String, String> {
    let _ = crate::db::log::add(db, user_id, format!("[falidex][placement][delete] id: {}", id)).await;
    // Vérifier les occurrences dans les relations
    let occurences = get_occurences(db, id.clone()).await?;
    
    if !occurences.is_empty() {
        let total_items: u64 = occurences.iter().map(|o| o.items).sum();
        return Err(format!(
            "Impossible de supprimer ce placement car il est utilisé dans {} relation(s) ({} item(s) au total)",
            occurences.len(),
            total_items
        ));
    }

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
            .filter(|item| &item.placement_id == &id)
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
