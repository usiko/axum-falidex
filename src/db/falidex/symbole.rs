use crate::model::falidex_model::{CreateSymbole, LinkDetail, OccurenceDetail, Symbole};
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};

pub async fn get(db: &Database) -> Result<Vec<Symbole>, String> {
    let col: Collection<Symbole> = db.collection::<Symbole>("symboles");
    let symboles: Vec<Symbole> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    Ok(symboles)
}

pub async fn create(db: &Database, user_id: String, data: CreateSymbole) -> Result<String, String> {
    let _ = crate::db::log::add(db, user_id, "[falidex][symbole][create]".to_string()).await;
    let col: Collection<CreateSymbole> = db.collection::<CreateSymbole>("symboles");
    col.insert_one(data)
        .await
        .map(|_| "Symbole créé avec succès".to_string())
        .map_err(|e| format!("Erreur lors de la création: {}", e))
}

pub async fn update(db: &Database, user_id: String, id: String, data: Symbole) -> Result<String, String> {
    let _ = crate::db::log::add(db, user_id, format!("[falidex][symbole][update] id: {}", id)).await;
    let col: Collection<Symbole> = db.collection::<Symbole>("symboles");
    let result = col
        .replace_one(doc! { "_id": id }, data)
        .await
        .map_err(|e| format!("Erreur lors de la mise à jour: {}", e))?;

    if result.modified_count > 0 {
        Ok("Symbole mis à jour avec succès".to_string())
    } else {
        Err("Aucun symbole trouvé avec cet identifiant".to_string())
    }
}

pub async fn delete(db: &Database, user_id: String, id: String) -> Result<String, String> {
    let _ = crate::db::log::add(db, user_id, format!("[falidex][symbole][delete] id: {}", id)).await;
    // Vérifier les occurrences dans les relations
    let occurences = get_occurences(db, id.clone()).await?;
    
    if !occurences.is_empty() {
        let total_items: u64 = occurences.iter().map(|o| o.items).sum();
        return Err(format!(
            "Impossible de supprimer ce symbole car il est utilisé dans {} relation(s) ({} item(s) au total)",
            occurences.len(),
            total_items
        ));
    }

    let col: Collection<Symbole> = db.collection::<Symbole>("symboles");
    let result = col
        .delete_one(doc! { "_id": id })
        .await
        .map_err(|e| format!("Erreur lors de la suppression: {}", e))?;

    if result.deleted_count > 0 {
        Ok("Symbole supprimé avec succès".to_string())
    } else {
        Err("Aucun symbole trouvé avec cet identifiant".to_string())
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
                item.symbole_id
                    .as_ref()
                    .map_or(false, |symbole_id| symbole_id == &id)
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
