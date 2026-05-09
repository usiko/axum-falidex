use crate::model::falidex_model::Circulaire;
use futures::stream::TryStreamExt;
use mongodb::{Collection, Database, bson::doc};

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
