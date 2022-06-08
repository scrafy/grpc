mod infraestructure;

use crate::infraestructure::implementations::DBContext;
use crate::infraestructure::traits::IDBContext;
use serde::ser::{SerializeStruct, Serializer};
use tonic::{transport::Server, Request, Response, Status};
use userstore::user_service_server::{UserService, UserServiceServer};
use userstore::{LoadUsersRequest, LoadUsersResponse, Users};

mod userstore {
    include!("userstore.rs");
}

impl serde::Serialize for Users {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Users", 8)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("first_name", &self.first_name)?;
        state.serialize_field("last_name", &self.last_name)?;
        state.serialize_field("telephon_numer", &self.telephon_numer)?;
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
        let body: Vec<Users> = request.into_inner().users;
        println!("Reciving message {:?}", body);
        let db = DBContext::new();
        //store(&db, &body);
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
/*
fn store(service: Box<dyn IDBContext>, data: &[Users]) -> () {
    service
        .set_database("Users")
        .set_collection("users")
        .insert(data);
}
*/