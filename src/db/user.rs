use mongodb::{
    Collection, Database,
    bson::{doc, oid::ObjectId},
};
use serde::{Deserialize, Serialize};

use crate::db::model::{User, UserAuth};

pub async fn get_by_id(db: &Database, id: String) -> std::result::Result<User, String> {
    let col = get_collection(db);
    // essayer de parser l'id
    let obj_id = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return Err("Invalid ObjectId".to_string()), // retourne une erreur si l'id est malformé
    };
    let result = col
        .find_one(doc! { "_id": obj_id })
        .await
        .map_err(|e| e.to_string())?;
    match result {
        Some(user) => Ok(user),
        None => Err("not found".to_string()),
    }
}
pub async fn auth(
    db: &Database,
    name: String,
    hash: String,
) -> std::result::Result<UserAuth, String> {
    let col = get_collection(db);
    let result = col
        .find_one(doc! { "username": name, "pwd_hash": hash })
        .await
        .map_err(|e| e.to_string())?;

    match result {
        Some(user) => Ok(UserAuth {
            id: user
                .id
                .map(|id: ObjectId| id.to_string())
                .unwrap_or_default(),
        }),
        None => Err("not found".to_string()),
    }
}
fn get_collection(db: &Database) -> Collection<User> {
    db.collection("users")
}
