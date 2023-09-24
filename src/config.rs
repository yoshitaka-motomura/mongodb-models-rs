//! # Config
//! ## Description
//! 環境変数から設定を読み込む

use dotenv::dotenv;
use std::env;
pub struct Config {
    pub mongo_uri: String,
    pub db_name: String,
}

impl Config {
    pub fn new() -> Config {
        dotenv().ok();
        let database = env::var("MONGO_DATABASE").unwrap_or("db_name".to_string());
        let username = env::var("MONGO_USERNAME").unwrap_or("root".to_string());
        let password = env::var("MONGO_PASSWORD").unwrap_or("root".to_string());
        let host = env::var("MONGO_HOST").unwrap_or("localhost".to_string());
        let port = env::var("MONGO_PORT").unwrap_or("27017".to_string());
        let mongo_uri = format!("mongodb://{}:{}@{}:{}", username, password, host, port);
        Config {
            mongo_uri: mongo_uri,
            db_name: database,
        }
    }
}
