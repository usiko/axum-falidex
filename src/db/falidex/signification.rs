use crate::model::falidex_model::Signification;
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};

pub async fn get(db: &Database) -> Result<Vec<Signification>, String> {
    let col: Collection<Signification> = db.collection::<Signification>("significations");
    let significations: Vec<Signification> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    Ok(significations)
}
