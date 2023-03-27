mod config;
mod mongodb_client;
mod models;

use tokio;
use futures::StreamExt;

use mongodb_client::{MongoDbClient, SalesDatabase};
use config::Config;


#[tokio::main]
async fn main() {
    // initialize config
    let config: Config = Config::new();
    // connect to client
    let client: MongoDbClient = MongoDbClient::new(config.get_mongo_uri()).await;
    // connect to database
    let db: SalesDatabase = SalesDatabase::new(&client.get_client());    

    let mut docs = db.find_docs_with_filter("NDC-FEBRUARY-2023").await.unwrap();

    while let Some(doc) = docs.next().await {
        match doc {
            Ok(d) => println!("{:?}", d),
            Err(e) => println!("Error: {}", e),
        }
    }
}
