use crate::model::falidex_model::Placement;
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};

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
