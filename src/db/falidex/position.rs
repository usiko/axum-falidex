use crate::model::falidex_model::Position;
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};

pub async fn get(db: &Database) -> Result<Vec<Position>, String> {
    let col: Collection<Position> = db.collection::<Position>("positions");
    let positions: Vec<Position> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    Ok(positions)
}
