use crate::model::falidex_model::{LinkDetail, OccurenceDetail, Placement};
use futures::stream::TryStreamExt;
use mongodb::{Collection, Database, bson::doc};

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

pub async fn create(db: &Database, data: Placement) -> Result<String, String> {
    let col: Collection<Placement> = db.collection::<Placement>("placements");
    col.insert_one(data)
        .await
        .map(|_| "Placement créé avec succès".to_string())
        .map_err(|e| format!("Erreur lors de la création: {}", e))
}

pub async fn update(db: &Database, id: String, data: Placement) -> Result<String, String> {
    let col: Collection<Placement> = db.collection::<Placement>("placements");
    let result = col
        .replace_one(doc! { "_id": id }, data)
        .await
        .map_err(|e| format!("Erreur lors de la mise à jour: {}", e))?;

    if result.modified_count > 0 {
        Ok("Placement mis à jour avec succès".to_string())
    } else {
        Err("Aucun placement trouvé avec cet identifiant".to_string())
    }
}

pub async fn delete(db: &Database, id: String) -> Result<String, String> {
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
