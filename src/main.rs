mod infraestructure;

use crate::infraestructure::schemas::SchemaUser;
use crate::infraestructure::traits::IDBContext;

use async_trait::async_trait;
use bson::Document;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};


use crate::infraestructure::implementations::DBContext;
//use crate::infraestructure::traits::IDBContext;
use serde::ser::{SerializeStruct, Serializer};
use tonic::{transport::Server, Request, Response, Status};
use userstore::user_service_server::{UserService, UserServiceServer};
use userstore::{LoadUsersRequest, LoadUsersResponse, User};

mod userstore {
    include!("userstore.rs");
}

impl serde::Serialize for User {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Users", 8)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("first_name", &self.first_name)?;
        state.serialize_field("last_name", &self.last_name)?;
        state.serialize_field("telephon_number", &self.telephon_number)?;
        state.serialize_field("address", &self.address)?;
        state.serialize_field("country", &self.country)?;
        state.serialize_field("zip_code", &self.zip_code)?;
        state.serialize_field("age", &self.age)?;
        state.end()
    }
}

#[derive(Default)]
pub struct UserServiceImpl {}

#[tonic::async_trait]
impl UserService for UserServiceImpl {
    async fn load_users(
        &self,
        request: Request<LoadUsersRequest>,
    ) -> std::result::Result<Response<LoadUsersResponse>, Status> {
        let body: Vec<User> = request.into_inner().users;
        


        let db = Box::new(DBContext::new());


        store(db, body);
        let response = LoadUsersResponse {
            message: "Users received".to_string(),
        };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();
    let userstore = UserServiceImpl::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(UserServiceServer::new(userstore))
        .serve(addr)
        .await?;

    Ok(())
}


fn store(mut service: Box<dyn IDBContext>, data: Vec<User>) -> () {
    
    service.as_mut().set_database("Users".to_string()).set_collection("users".to_string());
    insert(data);
   // service.as_mut().set_collection("users".to_string());
    //service.insert(data);
}


 fn insert(data: Vec<User>) -> () {
    
    let mut user_documents: Vec<Document> = Vec::new();
    for user in data.into_iter() {
        let schema_user: SchemaUser = SchemaUser {
            first_name: user.first_name,
            last_name: user.last_name,
            telephon_number: user.telephon_number,
            address: user.address,
            country: user.country,
            zip_code: user.zip_code,
            age: user.age,
        };

        user_documents.push(
            bson::to_bson(&schema_user)
                .unwrap()
                .as_document()
                .unwrap()
                .clone(),
        );
    }
   
   /* self.get_client()
        .database("Users")
        .collection::<Document>("users")
        .insert_many(user_documents, None);

        println!("{:?}", user_documents);

    let options =
        ClientOptions::parse_with_resolver_config("mongodb+srv://velocitech:compaq7500@velocitech.5peyq.mongodb.net/?retryWrites=true&w=majority".to_string(), ResolverConfig::cloudflare())
            .await
            .unwrap();
    Client::with_options(options)
        .unwrap()
        .database("Users")
        .collection::<Document>("users")
        .insert_many(user_documents, None);*/
}