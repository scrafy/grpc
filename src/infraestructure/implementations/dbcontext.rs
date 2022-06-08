use crate::infraestructure::schemas::SchemaUser;
use crate::infraestructure::traits::IDBContext;
use crate::userstore::Users;
use async_trait::async_trait;
use bson::Document;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use tokio;

pub struct DBContext {
    uri: String,
    database: Option<String>,
    collection: Option<String>,
}

impl DBContext {
    pub fn new() -> Self {
        DBContext{
                uri:"mongodb+srv://velocitech:compaq7500@velocitech.5peyq.mongodb.net/?retryWrites=true&w=majority".to_string(),
                database:None,
                collection:None
            }
    }

    #[tokio::main]
    async fn get_client(&self) -> Client {
        let options =
            ClientOptions::parse_with_resolver_config(&self.uri, ResolverConfig::cloudflare())
                .await
                .unwrap();
        return Client::with_options(options).unwrap();
    }
}

#[async_trait]
impl IDBContext for DBContext {
    fn set_database(&mut self, database: String) -> &Self {
        self.database = Some(database);
        return self;
    }

    fn set_collection(&mut self, collection: String) -> &Self {
        self.collection = Some(collection);
        return self;
    }

    async fn insert(&self, data: Vec<Users>) -> () {
        let mut user_documents: Vec<&Document> = Vec::new();

        for user in data.iter() {
            
            user_documents.push(
                bson::to_bson(&SchemaUser {
                    first_name: user.first_name,
                    last_name: user.last_name,
                    telephon_numer: user.telephon_numer,
                    address: user.address,
                    country: user.country,
                    zip_code: user.zip_code,
                    age: user.age
                })
                .unwrap()
                .as_document()
                .unwrap()
            )
        }        

        self.get_client()
            .database(&self.database.unwrap())
            .collection::<Document>(&self.collection.unwrap())
            .insert_many(user_documents, None);
    }
}
