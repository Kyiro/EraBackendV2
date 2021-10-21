use bcrypt::{DEFAULT_COST, hash};
use bson::serde_helpers::uuid_as_binary;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(with = "uuid_as_binary")]
    pub id: Uuid,
    #[serde(with = "uuid_as_binary")]
    pub launcher_token: Uuid,
    pub login: String,
    pub display_name: String,
    pub password: String,
    pub creation_date: bson::DateTime,
    pub admin: bool
}

impl User {
    pub fn new(id: Uuid, login: String, display_name: String, password: String) -> Self {
        Self {
            id,
            launcher_token: Uuid::new_v4(),
            login,
            display_name,
            password: hash(password.as_str(), DEFAULT_COST).unwrap(),
            creation_date: bson::DateTime::now(),
            admin: false
        }
    }
}