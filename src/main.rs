use tonic::{transport::Server, Request, Response, Status};
use userstore::user_service_server::{UserService, UserServiceServer};
use userstore::{LoadUsersRequest, LoadUsersResponse};


mod userstore {
    include!("userstore.rs");
   
}


#[derive(Default)]
pub struct UserServiceImpl {}

#[tonic::async_trait]
impl UserService for UserServiceImpl {
    async fn load_users(
        &self,
        request: Request<LoadUsersRequest>,
    ) -> Result<Response<LoadUsersResponse>, Status> {
        println!("Request {:?}", request);

        let response = LoadUsersResponse {
            message: "Users received".to_string()
        };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let userstore = UserServiceImpl::default();


    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(UserServiceServer::new(userstore))
        .serve(addr)
        .await?;

    Ok(())
}