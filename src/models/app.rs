use actix_web::*;
use crate::db::Database;
use jsonwebtoken::*;
use serde::{Deserialize, Serialize};
use tokio::*;
use uuid::Uuid;

pub struct AppData {
    pub database: Database,
    pub tokens: sync::RwLock<Vec<String>>
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Token {
    pub acc: Option<Uuid>,
    pub adm: bool,
    pub date: bson::DateTime
}

impl AppData {
    pub fn new(database: Database) -> Self {
        Self {
            database,
            tokens: sync::RwLock::new(Vec::new())
        }
    }
    
    pub fn new_data(database: Database) -> web::Data<AppData> {
        web::Data::new(Self::new(database))
    }
    
    pub async fn new_token(&self, id: Option<Uuid>, admin: bool) -> Option<String> {
        let token = Token {
            acc: id,
            adm: admin,
            date: bson::DateTime::now()
        };
        
        let encoded = encode(
            &Header::default(),
            &token,
            &EncodingKey::from_secret(crate::SECRET.as_ref())
        ).ok()?;
        
        {
            let mut tokens = self.tokens.write().await;
            tokens.push(encoded.clone());
        }
        
        Some(encoded)
    }
}