use mongodb::{
    Client, Collection, Database,
    bson::{Document, doc, oid::ObjectId},
    error::Result,
    options::UpdateOptions,
};

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct Persistence {
    name: String,
    value: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Users {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    username: String,
    pwd_hash: String,
}

pub struct MongoDB {
    pub client: Client,
    pub db: Database,
}

impl MongoDB {
    pub async fn new() -> mongodb::error::Result<Self> {
        // Replace the placeholder with your Atlas connection string
        let uri = "mongodb+srv://quentinusiko_db_user:58sErWBbkymGdUHb@cluster0.1vbyumm.mongodb.net/?appName=Cluster0";
        // Create a new client and connect to the server
        let client = Client::with_uri_str(uri).await?;
        let db = client.database("axum");

        Ok(Self { client, db })
    }

    fn get_col_persistence(&self) -> Collection<Persistence> {
        self.db.collection("text-persist")
    }
    fn get_col_users(&self) -> Collection<Users> {
        self.db.collection("users")
    }

    async fn get_doc_persistence(&self, key: String) -> Result<Option<Persistence>> {
        let col = self.get_col_persistence();
        let result = col.find_one(doc! { "name": key }).await?;
        Ok(result)
    }

    pub async fn get_persistence(&self, key: String) -> std::result::Result<String, String> {
        let doc = self
            .get_doc_persistence(key)
            .await
            .map_err(|e| e.to_string())?;

        match doc {
            Some(p) => Ok(p.value),
            None => Err("not found".to_string()),
        }
    }

    pub async fn set_persistence(
        &self,
        key: String,
        value: String,
        override_existing: bool,
    ) -> Result<()> {
        let col: Collection<Persistence> = self.get_col_persistence();

        let filter = doc! { "name": &key };

        let update = doc! {
            "$set": {
                "name": key,
                "value": value
            }
        };

        let options = UpdateOptions::builder().upsert(override_existing).build();

        col.update_one(filter, update).with_options(options).await?;

        Ok(())
    }

    fn auth_user(&self, user: String, hash_pwd: String) {
        let col = self.get_col_users();
    }
    fn get_user(&self, userId: String) {
        let col = self.get_col_users();
    }
}
