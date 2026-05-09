use crate::model::falidex_model::Filiere;
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};

pub async fn get(db: &Database) -> Result<Vec<Filiere>, String> {
    let col: Collection<Filiere> = db.collection::<Filiere>("filieres");
    let filieres: Vec<Filiere> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    Ok(filieres)
}
