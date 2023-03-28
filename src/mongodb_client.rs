use crate::config::Config;

use mongodb::{Client, Cursor, options::{FindOptions, ClientOptions, FindOneOptions}, bson::{doc, Document}};
use anyhow::Result;
pub struct MongoDbClient {
    client: Client,
}

impl MongoDbClient {
    pub async fn new() -> Self {
        let config: Config = Config::new();
        let uri: &str = config.get_mongo_uri();

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
}

pub struct MongoDatabase {
    client: Client,
    db: String,
    collection: String,
}

pub trait MongoDbModel {
    fn to_document(&self) -> Document;
    fn from_document(doc: Document) -> Self;
    fn get_collection_name() -> String;
}

impl MongoDatabase {
    pub fn new(client: &Client, db: Option<&str>, collection: Option<&str>) -> Self {
        let _db: &str = match db {
            Some(d) => d,
            None => panic!("No database specified."),
        };

        let _collection: &str = match collection {
            Some(c) => c,
            None => panic!("No collection specified."),
        };        

        Self {
            client: client.clone(),            
            db: _db.to_string(),
            collection: _collection.to_string(),
        }
    }        

    pub async fn find_docs_with_filter<T>(&self, filter: Option<Document>, options: Option<FindOptions>) -> Result<Cursor<T>, mongodb::error::Error> {        
        let _filter: Document = match filter {
            Some(f) => f,
            None => doc! { },
        };

        let _options: FindOptions = match options {
            Some(o) => o,
            None => FindOptions::builder().sort(doc! { }).build(),
        };
        
        let docs = self.client.database(&self.db).collection::<T>(&self.collection).find(_filter, _options).await?;

        Ok(docs)
    }

    pub async fn find_doc_with_filter<T>(&self, filter: Option<Document>) -> Result<(), mongodb::error::Error> {        
        let _filter: Document = match filter {
            Some(f) => f,
            None => doc! { },
        };

        let _options = FindOneOptions::builder().sort(doc! { }).build();

        let coll = self.client.database(&self.db).collection::<T>(&self.collection);

        let doc = coll.find_one(_filter, _options).await?;

        Ok(())        
    }
}

