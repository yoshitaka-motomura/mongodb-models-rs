//! Database connection
//! ## Description
//! MongoDBへの接続を行うモデルで読み込んでいます。

use mongodb::{Client, Database, options::ClientOptions};
use anyhow::Result;
use crate::config::Config;
use mongodb::Collection;

pub struct DB {
    db: Database,
}

impl DB {
    pub async fn new() -> Result<Self> {
        let config = Config::new();
        let client_options = ClientOptions::parse(&config.mongo_uri).await?;
        let client = Client::with_options(client_options)?;
        let db = client.database(&config.db_name);
        Ok(DB { db })
    }

    pub fn collection<T>(&self, name: &str) -> Collection<T> {
        self.db.collection(name)
    }
}
