use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SchemaUser {
    first_name: String,
    last_name: String,
    telephon_numer: String,
    address: String,
    country: String,
    zip_code: String,
    age: i32,
}
