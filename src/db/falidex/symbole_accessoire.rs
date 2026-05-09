use crate::model::falidex_model::SymboleAccessoire;
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};

pub async fn get(db: &Database) -> Result<Vec<SymboleAccessoire>, String> {
    let col: Collection<SymboleAccessoire> = db.collection::<SymboleAccessoire>("symboles-accessories");
    let symbole_accessoires: Vec<SymboleAccessoire> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    Ok(symbole_accessoires)
}
