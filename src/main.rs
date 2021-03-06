use bson::Document;
use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use serde::{Serialize, Deserialize};
use serde::ser::{SerializeStruct, Serializer};
use tonic::{transport::Server, Request, Response, Status};
use userstore::user_service_server::{UserService, UserServiceServer};
use userstore::{LoadUsersRequest, LoadUsersResponse, User};

mod userstore {
    include!("userstore.rs");
}


#[derive(Serialize, Deserialize, Debug)]
pub struct SchemaUser {
    pub first_name: String,
    pub last_name: String,
    pub telephon_number: String,
    pub address: String,
    pub country: String,
    pub zip_code: String,
    pub age: i32,
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
        println!("processing {} users", body.len());
        let handle = tokio::spawn(async move {
            let options =
                ClientOptions::parse_with_resolver_config("mongodb://localhost:27017/?readPreference=primary&appname=MongoDB%20Compass&ssl=false".to_string(), ResolverConfig::cloudflare())
                    .await
                    .unwrap();
            let client = Client::with_options(options).unwrap();
            client
                .database("Users")
                .collection::<Document>("users")
                .insert_many(documents(body), None)
                .await;
        });
       // handle.await;
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

fn documents(data: Vec<User>) -> Vec<Document> {
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
    return user_documents;
}
