use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Serialize, Deserialize, Debug)]
pub struct Persistence {
    pub name: String,
    pub value: String,
}

#[derive(Deserialize)]
pub struct AuthRequest {
    pub user_name: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(
        rename(serialize = "id", deserialize = "_id"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id_as_hex"
    )]
    pub id: Option<ObjectId>,
    username: String,
    #[serde(skip_serializing)]
    pwd_hash: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UserAuth {
    pub id: String,
}

fn serialize_object_id_as_hex<S>(value: &Option<ObjectId>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(id) => serializer.serialize_some(&id.to_hex()),
        None => serializer.serialize_none(),
    }
}
