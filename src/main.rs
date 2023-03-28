mod config;
mod mongodb_client;
mod pg_client;
mod models;

use mongodb::{bson::{doc, Document}, options::FindOptions};
use tokio;
use futures::StreamExt;
use anyhow::Result;

use mongodb_client::{MongoDbClient, MongoDatabase};
use pg_client::PgClient;
use models::{Sale, PgSale};


#[tokio::main]
async fn main() {
    // connect to mongodb client
    let client: MongoDbClient = MongoDbClient::new().await;
    // connect to database
    let sales_database: MongoDatabase = MongoDatabase::new(&client.get_client(), Some("busse_sales_data_warehouse"), Some("sales"));    

    let filter: Document = doc! {"key": {"$regex": "february-2023$", "$options": "i"}};
    let options: FindOptions = FindOptions::builder().sort(doc! {"key": 1}).build();
    let mut docs = sales_database.find_docs_with_filter::<Sale>(Some(filter), Some(options)).await.unwrap();

    while let Some(doc) = docs.next().await {
        match doc {
            Ok(_) => {},
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    // testing pg client

    // connect to postgres client
    let pg_connection: PgClient = PgClient::new().await;
    pg_connection.create_table_if_not_exists(Some("sales")).await;    

}
