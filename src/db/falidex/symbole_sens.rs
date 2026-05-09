use crate::model::falidex_model::SymboleSens;
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};

pub async fn get(db: &Database) -> Result<Vec<SymboleSens>, String> {
    let col: Collection<SymboleSens> = db.collection::<SymboleSens>("symboles-sens");
    let symboles_sens: Vec<SymboleSens> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    Ok(symboles_sens)
}
