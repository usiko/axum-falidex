use crate::model::falidex_model::CirculaireColor;
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};

pub async fn get(db: &Database) -> Result<Vec<CirculaireColor>, String> {
    let col: Collection<CirculaireColor> = db.collection::<CirculaireColor>("circulaires-colors");
    let circulaires_colors: Vec<CirculaireColor> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    Ok(circulaires_colors)
}
