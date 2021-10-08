use bcrypt::{DEFAULT_COST, hash};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
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
            login,
            display_name,
            password: hash(password.as_str(), DEFAULT_COST).unwrap(),
            creation_date: bson::DateTime::now(),
            admin: false
        }
    }
}