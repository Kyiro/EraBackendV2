use actix_web::*;
use crate::db::Database;
use crate::models::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::*;
use uuid::Uuid;

pub fn get_token(req: &HttpRequest) -> Option<String> {
    Some(
        req.headers().get("Authorization")?.to_str().ok()?
        .split_whitespace().collect::<Vec<_>>()[1].to_string()
    )
}

pub struct BasicToken {
    pub client_id: String,
    pub secret: String
}

pub fn get_basic(req: &HttpRequest) -> Option<BasicToken> {
    let token = get_token(req)?;
    let decoded = String::from_utf8(base64::decode(token).ok()?).ok()?;
    let split = decoded.split(":").collect::<Vec<_>>();
    
    Some(BasicToken {
        client_id: split.get(0)?.to_string(),
        secret: split.get(1)?.to_string()
    })
}

pub struct AppData {
    pub database: Database,
    pub tokens: sync::RwLock<HashMap<Uuid, Token>>
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Token {
    pub acc: Option<Uuid>,
    pub adm: bool
}

impl AppData {
    pub fn new(database: Database) -> Self {
        Self {
            database,
            tokens: sync::RwLock::new(HashMap::new())
        }
    }
    
    pub fn new_data(database: Database) -> crate::AppData {
        crate::AppData::new(Self::new(database))
    }
    
    pub async fn login(&self, login: String, password: String) -> Result<user::User, Box<dyn std::error::Error>> {
        if let Some(user) = self.database.users.find_one(
            bson::doc! {
                "login": login
            },
            None
        ).await? {
            if let Ok(true) = bcrypt::verify(password, &user.password) {
                return Ok(user)
            }
        }
        
        return Err(Box::new(io::Error::new(
            io::ErrorKind::NotFound,
            "Invalid Login"
        )))
    }
    
    pub async fn validate(&self, req: &HttpRequest, id: Option<Uuid>) -> Option<()> {
        let token = Uuid::parse_str(&get_token(req)?).ok()?;
        
        let tokens = self.tokens.read().await;
            
        let user = tokens.get(&token)?;
        
        if let Some(id) = id {
            if user.acc == Some(id) {
                return Some(())
            }
            return None
        }
        
        Some(())
    }
    
    pub async fn new_token(&self, id: Option<Uuid>, admin: bool) -> Option<Uuid> {
        let data = Token {
            acc: id,
            adm: admin
        };
        let token = uuid::Uuid::new_v4();
        
        {
            let mut tokens = self.tokens.write().await;
            tokens.insert(token, data);
        }
        
        Some(token)
    }
}