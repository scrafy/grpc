use crate::userstore::User;
use async_trait::async_trait;

#[async_trait]
pub trait IDBContext {
    fn set_database(&mut self, data: String) -> &mut dyn IDBContext;
    fn set_collection(&mut self, collection: String) -> &mut dyn IDBContext;
    async fn insert(&self, data: Vec<User>) -> ();
}
