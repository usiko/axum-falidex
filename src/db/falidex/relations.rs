use crate::model::falidex_model::{CreateLinkDetail, Link, LinkDetail, LinkItem};
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};
use uuid::Uuid;

pub async fn get(db: &Database) -> Result<Vec<Link>, String> {
    let col: Collection<Link> = db.collection::<Link>("links");
    let data: Vec<Link> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    Ok(data)
}

pub async fn get_item(db: &Database, item: String) -> Result<LinkDetail, String> {
    let col: Collection<LinkDetail> = db.collection::<LinkDetail>("links");
    let result = col
        .find_one(doc! { "_id": item})
        .await
        .map_err(|e| e.to_string())?;
    result.ok_or("Link not found".to_string())
}

pub async fn create(db: &Database, user_id: String, data: CreateLinkDetail) -> Result<String, String> {
    let _ = crate::db::log::add(db, user_id, "[falidex][relations][create]".to_string()).await;
    let col: Collection<CreateLinkDetail> = db.collection::<CreateLinkDetail>("links");
    col.insert_one(data)
        .await
        .map(|_| "Link créé avec succès".to_string())
        .map_err(|e| format!("Erreur lors de la création du link: {}", e))
}

pub async fn update(db: &Database, user_id: String, data: LinkDetail) -> Result<String, String> {
    let _ = crate::db::log::add(db, user_id, format!("[falidex][relations][update] id: {}", data.id)).await;
    let col: Collection<LinkDetail> = db.collection::<LinkDetail>("links");
    let result = col
        .replace_one(doc! { "_id": data.id.clone()}, data)
        .await
        .map_err(|e| format!("Erreur lors de la mise à jour du link: {}", e))?;

    if result.modified_count > 0 {
        Ok("Link mis à jour avec succès".to_string())
    } else {
        Err("Aucun link trouvé avec cet identifiant".to_string())
    }
}

pub async fn delete(db: &Database, user_id: String, link_id: String) -> Result<String, String> {
    let _ = crate::db::log::add(db, user_id, format!("[falidex][relations][delete] id: {}", link_id)).await;
    let col: Collection<LinkDetail> = db.collection::<LinkDetail>("links");
    let result = col
        .delete_one(doc! { "_id": link_id })
        .await
        .map_err(|e| format!("Erreur lors de la suppression du link: {}", e))?;

    if result.deleted_count > 0 {
        Ok("Link supprimé avec succès".to_string())
    } else {
        Err("Aucun link trouvé avec cet identifiant".to_string())
    }
}

pub async fn create_item_relation(
    db: &Database,
    user_id: String,
    link_id: String,
    mut data: LinkItem,
) -> Result<String, String> {
    let _ = crate::db::log::add(db, user_id.clone(), format!("[falidex][relations][create_item_relation] link_id: {}", link_id)).await;
    // Récupérer le LinkDetail existant
    let mut item = get_item(db, link_id.clone()).await?;

    // Vérifier si le link est éditable
    if item.editable == Some(false) {
        return Err("Ce link n'est pas éditable".to_string());
    }

    // Générer un id si non fourni
    if data.id.is_none() {
        data.id = Some(Uuid::new_v4().to_string());
    }

    // Définir les dates de création et modification
    let now = chrono::Utc::now().to_rfc3339();
    data.created_at = Some(now.clone());
    data.updated_at = Some(now);

    // Ajouter le nouveau LinkItem aux relations
    item.relations.push(data);

    // Mettre à jour le LinkDetail complet
    update(db, user_id, item).await
}

pub async fn update_item_relation(
    db: &Database,
    user_id: String,
    link_id: String,
    mut data: LinkItem,
) -> Result<String, String> {
    let _ = crate::db::log::add(db, user_id.clone(), format!("[falidex][relations][update_item_relation] link_id: {}", link_id)).await;
    // Récupérer le LinkDetail existant
    let mut item = get_item(db, link_id.clone()).await?;

    // Vérifier si le link est éditable
    if item.editable == Some(false) {
        return Err("Ce link n'est pas éditable".to_string());
    }

    // Trouver et remplacer le LinkItem dans les relations par son id
    if let Some(relation_id) = &data.id {
        if let Some(pos) = item
            .relations
            .iter()
            .position(|r| r.id.as_ref() == Some(relation_id))
        {
            // Conserver la date de création originale
            let created_at = item.relations[pos].created_at.clone();
            data.created_at = created_at;

            // Mettre à jour la date de modification
            data.updated_at = Some(chrono::Utc::now().to_rfc3339());

            item.relations[pos] = data;
        } else {
            return Err("Aucune relation trouvée avec cet identifiant".to_string());
        }
    } else {
        return Err("L'identifiant de la relation est requis pour la mise à jour".to_string());
    }

    // Mettre à jour le LinkDetail complet
    update(db, user_id, item).await
}

pub async fn delete_item_relation(
    db: &Database,
    user_id: String,
    link_id: String,
    relation_id: String,
) -> Result<String, String> {
    let _ = crate::db::log::add(db, user_id.clone(), format!("[falidex][relations][delete_item_relation] link_id: {}, relation_id: {}", link_id, relation_id)).await;
    // Récupérer le LinkDetail existant
    let mut item = get_item(db, link_id.clone()).await?;

    // Vérifier si le link est éditable
    if item.editable == Some(false) {
        return Err("Ce link n'est pas éditable".to_string());
    }

    // Trouver et supprimer le LinkItem dans les relations par son id
    if let Some(pos) = item
        .relations
        .iter()
        .position(|r| r.id.as_ref() == Some(&relation_id))
    {
        item.relations.remove(pos);
    } else {
        return Err("Aucune relation trouvée avec cet identifiant".to_string());
    }

    // Mettre à jour le LinkDetail complet
    update(db, user_id, item).await?;
    Ok("Relation supprimée avec succès".to_string())
}

/*
* fixing current data
*/
pub async fn fix_relation_id(db: &Database) -> Result<String, String> {
    let col: Collection<LinkDetail> = db.collection::<LinkDetail>("links");

    // Récupérer tous les LinkDetail
    let mut links: Vec<LinkDetail> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;

    let mut updated_count = 0;

    // Parcourir chaque LinkDetail
    for link in &mut links {
        let mut has_changes = false;

        // Parcourir chaque relation et générer un ID si absent
        for relation in &mut link.relations {
            if relation.id.is_none() {
                relation.id = Some(Uuid::new_v4().to_string());
                has_changes = true;
            }
        }

        // Mettre à jour le document si des changements ont été effectués
        if has_changes {
            col.replace_one(doc! { "_id": &link.id }, link)
                .await
                .map_err(|e| format!("Erreur lors de la mise à jour: {}", e))?;
            updated_count += 1;
        }
    }

    Ok(format!("{} link(s) mis à jour avec succès", updated_count))
}
