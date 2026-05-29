use super::update_helper;
use crate::model::falidex_model::{
    CreateSymbole, Img, LinkDetail, OccurenceDetail, Symbole, UpdateSymbole,
};
use futures::stream::TryStreamExt;
use mongodb::bson;
use mongodb::{Collection, Database, bson::doc};

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

pub async fn update(
    db: &Database,
    user_id: String,
    id: String,
    data: UpdateSymbole,
) -> Result<String, String> {
    let _ = crate::db::log::add(
        db,
        user_id,
        format!("[falidex][symbole][update] id: {}", id),
    )
    .await;
    let col: Collection<Symbole> = db.collection::<Symbole>("symboles");

    let mut update_doc = doc! {};
    if let Some(name) = data.name {
        update_doc.insert("name", name);
    }
    if let Some(imgs) = data.imgs {
        let imgs_bson = bson::to_bson(&imgs)
            .map_err(|e| format!("Erreur lors de la sérialisation des images: {}", e))?;
        update_doc.insert("imgs", imgs_bson);
    }

    let (matched, modified) = update_helper::partial_update(&col, &id, update_doc).await?;

    if matched == 0 {
        Err("Aucun symbole trouvé avec cet identifiant".to_string())
    } else if modified > 0 {
        Ok("Symbole mis à jour avec succès".to_string())
    } else {
        Ok("Aucune modification détectée (valeurs identiques)".to_string())
    }
}

pub async fn delete(db: &Database, user_id: String, id: String) -> Result<String, String> {
    let _ = crate::db::log::add(
        db,
        user_id,
        format!("[falidex][symbole][delete] id: {}", id),
    )
    .await;
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

pub async fn set_picture_migrated(
    db: &Database,
    img: Img,
    symbole_id: &str,
) -> Result<String, String> {
    let col: Collection<Symbole> = db.collection::<Symbole>("symboles");

    // Récupère le symbole pour avoir la liste des images
    let symbole = col
        .find_one(doc! { "_id": &symbole_id })
        .await
        .map_err(|e| format!("Erreur lors de la récupération du symbole: {}", e))?
        .ok_or_else(|| "Symbole introuvable".to_string())?;

    let mut update_doc = doc! {};
    if let Some(mut imgs) = symbole.imgs {
        for i in &mut imgs {
            if i.id == img.id {
                i.migrated = Some(true);
            }
        }
        let imgs_bson = bson::to_bson(&imgs)
            .map_err(|e| format!("Erreur lors de la sérialisation des images: {}", e))?;
        update_doc.insert("imgs", imgs_bson);
    }
    let (matched, modified) = update_helper::partial_update(&col, &symbole_id, update_doc).await?;

    if matched == 0 {
        Err("Aucun symbole trouvé avec cet identifiant".to_string())
    } else if modified > 0 {
        Ok("Symbole mis à jour avec succès".to_string())
    } else {
        Ok("Aucune modification détectée (valeurs identiques)".to_string())
    }
}
