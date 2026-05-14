use crate::model::falidex_model::{CreateSymboleAccessoire, LinkDetail, OccurenceDetail, SymboleAccessoire};
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};

pub async fn get(db: &Database) -> Result<Vec<SymboleAccessoire>, String> {
    let col: Collection<SymboleAccessoire> = db.collection::<SymboleAccessoire>("symboles-accessories");
    let symbole_accessoires: Vec<SymboleAccessoire> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    Ok(symbole_accessoires)
}

pub async fn create(db: &Database, user_id: String, data: SymboleAccessoire) -> Result<String, String> {
    let _ = crate::db::log::add(db, user_id, "[falidex][symbole_accessoire][create]".to_string()).await;
    let col: Collection<SymboleAccessoire> = db.collection::<SymboleAccessoire>("symboles-accessories");
    col.insert_one(data)
        .await
        .map(|_| "Symbole accessoire créé avec succès".to_string())
        .map_err(|e| format!("Erreur lors de la création: {}", e))
}

pub async fn update(db: &Database, user_id: String, id: String, data: SymboleAccessoire) -> Result<String, String> {
    let _ = crate::db::log::add(db, user_id, format!("[falidex][symbole_accessoire][update] id: {}", id)).await;
    let col: Collection<SymboleAccessoire> = db.collection::<SymboleAccessoire>("symboles-accessories");
    let result = col
        .replace_one(doc! { "_id": id }, data)
        .await
        .map_err(|e| format!("Erreur lors de la mise à jour: {}", e))?;

    if result.modified_count > 0 {
        Ok("Symbole accessoire mis à jour avec succès".to_string())
    } else {
        Err("Aucun symbole accessoire trouvé avec cet identifiant".to_string())
    }
}

pub async fn delete(db: &Database, user_id: String, id: String) -> Result<String, String> {
    let _ = crate::db::log::add(db, user_id, format!("[falidex][symbole_accessoire][delete] id: {}", id)).await;
    // Vérifier les occurrences dans les relations
    let occurences = get_occurences(db, id.clone()).await?;
    
    if !occurences.is_empty() {
        let total_items: u64 = occurences.iter().map(|o| o.items).sum();
        return Err(format!(
            "Impossible de supprimer ce symbole accessoire car il est utilisé dans {} relation(s) ({} item(s) au total)",
            occurences.len(),
            total_items
        ));
    }

    let col: Collection<SymboleAccessoire> = db.collection::<SymboleAccessoire>("symboles-accessories");
    let result = col
        .delete_one(doc! { "_id": id })
        .await
        .map_err(|e| format!("Erreur lors de la suppression: {}", e))?;

    if result.deleted_count > 0 {
        Ok("Symbole accessoire supprimé avec succès".to_string())
    } else {
        Err("Aucun symbole accessoire trouvé avec cet identifiant".to_string())
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
                item.symbole_accessory_id
                    .as_ref()
                    .map_or(false, |symbole_accessory_id| symbole_accessory_id == &id)
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