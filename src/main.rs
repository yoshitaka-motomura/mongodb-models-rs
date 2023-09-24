mod models;
mod db;
mod config;
mod init;
use anyhow::Result;
#[allow(unused_imports)]
use models::airport::AirportCollection;
#[allow(unused_imports)]
use mongodb::bson::doc;

use init::init;

#[tokio::main]
async fn main() -> Result<()> {

    init().await?;
    let airport_collection = AirportCollection::new().await?;
    
    let geo_filter = doc! {
        "$geoNear": {
            "near": {
                "coordinates": [139.6917, 35.6895],
            },
            "distanceField": "dist.calculated",
            "maxDistance": 100000,
            "includeLocs": "dist.location",
            "spherical": true,
            "query": {
                "airport_type": {
                    "$ne": "heliport",
                }
            }
        }
    };
    let filters = vec![geo_filter];
    let results = airport_collection.aggregate(filters).await?;

    println!("{:?}", results);

    Ok(())
}
