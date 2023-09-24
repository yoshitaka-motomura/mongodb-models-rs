//! Airport models
//! ## description
//! 
//! mongodbのairportsコレクションのモデル化
//! 
//! ## Examples
//! 
//! ```
//! use models::airport::AirportCollection;
//! use mongodb::bson::doc;
//! 
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!    let airport_collection = AirportCollection::new().await?;
//!    
//!   let geo_filter = doc! {
//!      "$geoNear": {
//!         "near": {
//!            "coordinates": [139.6917, 35.6895],
//!        },
//!       "distanceField": "dist.calculated",
//!      "maxDistance": 100000,
//!    "includeLocs": "dist.location",
//!  "spherical": true,
//! "query": {
//!   "airport_type": {
//!    "$ne": "heliport",
//! }   
//! }
//! }
//! };
//! let filters = vec![geo_filter];
//! let results = airport_collection.aggregate(filters).await?;
//! 
//! println!("{:?}", results);
//!  
//! Ok(())
//! }
//! ```

use mongodb::{Collection, bson::doc, options::FindOptions};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use mongodb::bson::Document;
use crate::db::DB;

/// Coordinates model
#[allow(dead_code)]
#[derive(Serialize, Debug, Deserialize)]
pub struct Coordinates {
    latitude: Option<f64>,
    longitude: Option<f64>,
}

/// Airport model
#[allow(dead_code)]
#[derive(Serialize, Debug, Deserialize)]
pub struct Airport {
    #[serde(rename = "_id", skip_serializing)]
    id: Option<bson::oid::ObjectId>,
    ident: String,
    airport_type: String,
    name: String,
    elevation_ft: String,
    continent: String,
    iso_country: String,
    iso_region: String,
    municipality: String,
    gps_code: String,
    iata_code: String,
    local_code: String,
    coordinates: Coordinates, 
}

/// Airport collection model
pub struct AirportCollection {
    collection: Collection<Airport>,
}

/// Airport collection implementation
impl AirportCollection {
    /// Create new airport collection
    #[allow(dead_code)]
    pub async fn new() -> Result<Self> {
        let db = DB::new().await?;
        let collection: Collection<Airport> = db.collection("airports");
        Ok(AirportCollection { collection })
    }

    /// Find all airports
    #[allow(dead_code)]
    pub async fn find_all(&self, limit:i64) -> Result<Vec<Airport>> {
        let find_options = FindOptions::builder().sort(doc! { "id": 1 }).limit(limit).build();
        let mut cursor = self.collection.find(None, find_options).await?;
        let mut airports: Vec<Airport> = Vec::new();
        while let Some(result) = cursor.next().await { // ここでnextメソッドを呼び出し
            match result {
                Ok(document) => {
                    airports.push(document);
                },
                Err(e) => return Err(e.into()),
            }
        }
        Ok(airports)
    }

    /// aggregate
    #[allow(dead_code)]
    pub async fn aggregate(&self, filter: Vec<Document>)-> Result<Vec<Document>> {
        let mut cursor = self.collection.aggregate(filter, None).await?;
        let mut results:Vec<Document> = Vec::new();
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                   results.push(document);
                },
                Err(e) => return Err(e.into()),
            }
        }
        Ok(results)
    }
}
