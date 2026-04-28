use mongodb::{Client, Collection, Database, bson::oid::ObjectId};

use crate::db::model::User;
use crate::db::model::UserAuth;

use super::persistence;
use super::user;

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
        let db = client.database("falidex");

        Ok(Self { client, db })
    }

    pub async fn get_persistence(
        &self,
        key: String,
        user_id: String,
    ) -> std::result::Result<String, String> {
        persistence::get(&self.db, key, user_id).await
    }

    pub async fn set_persistence(
        &self,
        key: String,
        value: String,
        user_id: String,
        override_existing: bool,
    ) -> mongodb::error::Result<()> {
        persistence::set(&self.db, key, value, user_id, override_existing).await
    }

    /*pub async fn auth_user(
        &self,
        user_name: String,
        hash_pwd: String,
    ) -> std::result::Result<User, String> {
        let db = &self.db;
        user::auth(db, user_name, hash_pwd).await
    }
    pub async fn get_user(&self, user_id: String) -> std::result::Result<User, String> {
        let db = &self.db;
        user::get_by_id(db, user_id).await
    }*/
}
