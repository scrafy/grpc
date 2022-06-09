use serde::{Deserialize, Serialize};

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
