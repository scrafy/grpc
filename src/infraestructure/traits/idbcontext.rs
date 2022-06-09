use crate::userstore::User;
use async_trait::async_trait;

#[async_trait]
pub trait IDBContext {
    fn set_database(&mut self, data: String) -> &Self;
    fn set_collection(&mut self, collection: String) -> &Self;
   async fn insert(&self, data: Vec<User>) -> ();
}
