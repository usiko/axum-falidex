use mongodb::{Collection, Database, bson::doc, options::UpdateOptions};
use serde::{Deserialize, Serialize};

use crate::db::model::Persistence;

pub async fn get(
    db: &Database,
    key: String,
    user_id: String,
) -> std::result::Result<String, String> {
    let doc = get_in_document(db, key, user_id)
        .await
        .map_err(|e| e.to_string())?;

    match doc {
        Some(p) => Ok(p.value),
        None => Err("not found".to_string()),
    }
}

pub async fn set(
    db: &Database,
    key: String,
    value: String,
    user_id: String,
    override_existing: bool,
) -> mongodb::error::Result<()> {
    let col: Collection<Persistence> = get_collection(db);

    let filter = doc! { "name": &key };

    let update = doc! {
        "$set": {
            "name": key,
            "value": value,
            "user_id":user_id
        }
    };

    let options = UpdateOptions::builder().upsert(override_existing).build();

    col.update_one(filter, update).with_options(options).await?;

    Ok(())
}

async fn get_in_document(
    db: &Database,
    key: String,
    user_id: String,
) -> mongodb::error::Result<Option<Persistence>> {
    let col = get_collection(db);
    let result = col.find_one(doc! { "name": key,"user_id":user_id }).await?;
    Ok(result)
}
fn get_collection(db: &Database) -> Collection<Persistence> {
    db.collection("text-persist")
}
