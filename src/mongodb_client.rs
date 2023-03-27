use crate::models;

use models::Sale;
use mongodb::{Client, Collection, Cursor, options::{FindOptions, ClientOptions}, bson::{doc, Document}};


const DB: &str = "busse_sales_data_warehouse";
const COLLECTION: &str = "sales";

pub struct MongoDbClient {
    client: Client,    
}

impl MongoDbClient {
    pub async fn new(uri: &str) -> Self {
        let mut client_options: ClientOptions = ClientOptions::parse(uri).await.unwrap();
        client_options.app_name = Some("busse".to_string());

        let client: Client = Client::with_options(client_options).unwrap();

        Self {
            client,
        }
    }    

    pub fn get_client(&self) -> &Client {
        &self.client
    }

    pub async fn get_databases(&self) -> Vec<String> {
        self.client.list_database_names(None, None).await.unwrap()
    }
}

pub struct SalesDatabase {
    client: Client,
    database: Collection<Sale>,
    db: String,
    collection: String,
}

impl SalesDatabase {
    pub fn new(client: &Client) -> Self {        
        Self {
            client: client.clone(),
            database: client.database(DB).collection(COLLECTION),
            db: DB.to_string(),
            collection: COLLECTION.to_string(),
        }
    }

    pub fn get_database(&self) -> &Collection<Sale> {
        &self.database
    }

    pub async fn get_collections(&self) -> Vec<String> {        
        self.client.database(&self.db).list_collection_names(None).await.unwrap()      
    }

    pub async fn find_docs_with_filter(&self, key: &str) -> Result<Cursor<Sale>, mongodb::error::Error> {        
        let filter: Document = doc! { "key": doc! { "$regex": &key, "$options": "i"} };
        let options: FindOptions = FindOptions::builder().sort(doc! { "rep": 1 }).build();
        
        let docs = self.database.find(filter, options).await?;

        Ok(docs)
    }
}

