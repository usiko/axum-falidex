use crate::model::falidex_model::{
    Circulaire, CirculaireColor, CreateCirculaire, LinkDetail, OccurenceDetail, UpdateCirculaire,
};
use futures::stream::TryStreamExt;
use mongodb::bson;
use mongodb::{Collection, Database, bson::doc};
use super::update_helper;

pub async fn get(db: &Database) -> Result<Vec<Circulaire>, String> {
    let col: Collection<Circulaire> = db.collection::<Circulaire>("circulaires");
    let circulaires: Vec<Circulaire> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    Ok(circulaires)
}
pub async fn create(
    db: &Database,
    user_id: String,
    data: CreateCirculaire,
) -> Result<String, String> {
    let _ = crate::db::log::add(db, user_id, "[falidex][circulaire][create]".to_string()).await;
    let col: Collection<CreateCirculaire> = db.collection::<CreateCirculaire>("circulaires");
    col.insert_one(data)
        .await
        .map(|_| "Circulaire créée avec succès".to_string())
        .map_err(|e| format!("Erreur lors de la création: {}", e))
}

pub async fn update(
    db: &Database,
    user_id: String,
    id: String,
    data: UpdateCirculaire,
) -> Result<String, String> {
    let _ = crate::db::log::add(
        db,
        user_id,
        format!("[falidex][circulaire][update] id: {}", id),
    )
    .await;
    let col: Collection<Circulaire> = db.collection::<Circulaire>("circulaires");

    let mut update_doc = doc! {};
    if let Some(matiere) = data.matiere {
        update_doc.insert("matiere", matiere);
    }
    if let Some(name) = data.name {
        update_doc.insert("name", name);
    }

    let (matched, modified) = update_helper::partial_update(&col, &id, update_doc).await?;

    if matched == 0 {
        Err("Aucune circulaire trouvée avec cet identifiant".to_string())
    } else if modified > 0 {
        Ok("Circulaire mise à jour avec succès".to_string())
    } else {
        Ok("Aucune modification détectée (valeurs identiques)".to_string())
    }
}

pub async fn delete(db: &Database, user_id: String, id: String) -> Result<String, String> {
    let _ = crate::db::log::add(
        db,
        user_id,
        format!("[falidex][circulaire][delete] id: {}", id),
    )
    .await;
    // Vérifier les occurrences dans les relations
    let occurences = get_occurences(db, id.clone()).await?;

    if !occurences.is_empty() {
        let total_items: u64 = occurences.iter().map(|o| o.items).sum();
        return Err(format!(
            "Impossible de supprimer cette circulaire car elle est utilisée dans {} relation(s) ({} item(s) au total)",
            occurences.len(),
            total_items
        ));
    }

    // D'abord supprimer les circulaire_color associés
    let col_colors: Collection<CirculaireColor> =
        db.collection::<CirculaireColor>("circulaires-colors");
    col_colors
        .delete_many(doc! { "circulaireId": &id })
        .await
        .map_err(|e| {
            format!(
                "Erreur lors de la suppression des circulaire_color associés: {}",
                e
            )
        })?;

    // Ensuite supprimer la circulaire
    let col: Collection<Circulaire> = db.collection::<Circulaire>("circulaires");
    let result = col
        .delete_one(doc! { "_id": id })
        .await
        .map_err(|e| format!("Erreur lors de la suppression: {}", e))?;

    if result.deleted_count > 0 {
        Ok("Circulaire supprimée avec succès".to_string())
    } else {
        Err("Aucune circulaire trouvée avec cet identifiant".to_string())
    }
}

pub async fn get_occurences(db: &Database, id: String) -> Result<Vec<OccurenceDetail>, String> {
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
                    .map_or(false, |circulaire_id| circulaire_id == &id)
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
