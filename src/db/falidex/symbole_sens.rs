use crate::model::falidex_model::{LinkDetail, OccurenceDetail, SymboleSens};
use futures::stream::TryStreamExt;
use mongodb::{Collection, Database, bson::doc};

pub async fn get(db: &Database) -> Result<Vec<SymboleSens>, String> {
    let col: Collection<SymboleSens> = db.collection::<SymboleSens>("symboles-sens");
    let symboles_sens: Vec<SymboleSens> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    Ok(symboles_sens)
}

pub async fn create(db: &Database, data: SymboleSens) -> Result<String, String> {
    let _ = crate::db::log::add(
        db,
        "system".to_string(),
        "[falidex][symbole_sens][create]".to_string(),
    )
    .await;
    let col: Collection<SymboleSens> = db.collection::<SymboleSens>("symboles-sens");
    col.insert_one(data)
        .await
        .map(|_| "Symbole sens créé avec succès".to_string())
        .map_err(|e| format!("Erreur lors de la création: {}", e))
}

pub async fn update(db: &Database, id: String, data: SymboleSens) -> Result<String, String> {
    let _ = crate::db::log::add(
        db,
        "system".to_string(),
        format!("[falidex][symbole_sens][update] id: {}", id),
    )
    .await;
    let col: Collection<SymboleSens> = db.collection::<SymboleSens>("symboles-sens");
    let result = col
        .replace_one(doc! { "_id": id }, data)
        .await
        .map_err(|e| format!("Erreur lors de la mise à jour: {}", e))?;

    if result.modified_count > 0 {
        Ok("Symbole sens mis à jour avec succès".to_string())
    } else {
        Err("Aucun symbole sens trouvé avec cet identifiant".to_string())
    }
}

pub async fn delete(db: &Database, id: String) -> Result<String, String> {
    let _ = crate::db::log::add(
        db,
        "system".to_string(),
        format!("[falidex][symbole_sens][delete] id: {}", id),
    )
    .await;
    // Vérifier les occurrences dans les relations
    let occurences = get_occurences(db, id.clone()).await?;

    if !occurences.is_empty() {
        let total_items: u64 = occurences.iter().map(|o| o.items).sum();
        return Err(format!(
            "Impossible de supprimer ce symbole sens car il est utilisé dans {} relation(s) ({} item(s) au total)",
            occurences.len(),
            total_items
        ));
    }

    let col: Collection<SymboleSens> = db.collection::<SymboleSens>("symboles-sens");
    let result = col
        .delete_one(doc! { "_id": id })
        .await
        .map_err(|e| format!("Erreur lors de la suppression: {}", e))?;

    if result.deleted_count > 0 {
        Ok("Symbole sens supprimé avec succès".to_string())
    } else {
        Err("Aucun symbole sens trouvé avec cet identifiant".to_string())
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
                item.symbole_sens_id
                    .as_ref()
                    .map_or(false, |symbole_sens_id| symbole_sens_id == &id)
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
