//! テストデータを投入する為関数です
//! このファイルを読み込んでるファイルから削除して問題ないです。
//! 
use std::fs::File;

use anyhow::Result;
use crate::config::Config;
use mongodb::{bson::doc, IndexModel};
use mongodb::{Client, options::ClientOptions};

use crate::models::airport::Airport;
pub async fn init() -> Result<()> {
    let config = Config::new();
    
    let client_options = ClientOptions::parse(&config.mongo_uri).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database(&config.db_name);

    let collection_names = vec!["airports",];

    // データベースにコレクションが存在しない場合は作成する
    for collection_name in collection_names {
        match db.list_collection_names(None).await?.iter().find(|&name| name == collection_name) {
            Some(_) => {},
            None => {
                db.create_collection(collection_name, None).await?;
                let collection = db.collection::<Airport>(collection_name);
                match collection.list_index_names().await?.iter().find(|&index| index == "coordinates_2dsphere") {
                    Some(_) => {},
                    None => {
                        let index = doc! { "coordinates": "2dsphere" };
                        let index_model = IndexModel::builder()
                            .keys(index)
                            .build();
                        collection.create_index(index_model, None).await?;
                    },
                }
            },
        }
    }
    //データがあるか確認してなければデータを投入する

    let collection = db.collection::<Airport>("airports");
    if collection.count_documents(None, None).await? == 0 {
        println!("データがありません。テストデータを投入します。");
        //resources/airports.jsonを読み込んでデータを投入する
        let json = File::open("resources/airports.json")?;
        let airports: Vec<Airport> = serde_json::from_reader(json)?;
        collection.insert_many(airports, None).await?;
    } else {
        println!("🚧この関数は既に不要です main.rsから削除しても問題ありません。");
    }

    
    Ok(())
}