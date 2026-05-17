use mongodb::{bson::Document, Collection, Database};

/// Helper pour les mises à jour partielles avec matched_count check
pub async fn partial_update<T: Send + Sync>(
    col: &Collection<T>,
    id: &str,
    update_doc: Document,
) -> Result<(u64, u64), String> {
    if update_doc.is_empty() {
        return Err("Aucun champ à mettre à jour".to_string());
    }

    let result = col
        .update_one(
            mongodb::bson::doc! { "_id": id },
            mongodb::bson::doc! { "$set": update_doc },
        )
        .await
        .map_err(|e| format!("Erreur lors de la mise à jour: {}", e))?;

    Ok((result.matched_count, result.modified_count))
}
