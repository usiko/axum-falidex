use crate::model::falidex_model::Color;
use futures::stream::TryStreamExt;
use mongodb::{Collection, Database, bson::doc};

pub async fn get(db: &Database) -> Result<Vec<Color>, String> {
    let col: Collection<Color> = db.collection::<Color>("colors");
    let colors: Vec<Color> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    Ok(colors)
}
