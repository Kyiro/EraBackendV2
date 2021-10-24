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
    pub captcha: Option<captcha::Client>,
    pub tokens: sync::RwLock<HashMap<Uuid, Token>>,
    pub exchange: sync::RwLock<HashMap<Uuid, ExchangeCode>>,
    pub files: files::Files
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Token {
    pub acc: Option<Uuid>,
    pub adm: bool
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExchangeCode {
    pub acc: Uuid
}

impl AppData {
    pub fn new(database: Database, include_captcha: bool) -> Self {
        let mut captcha = None;
        
        if include_captcha {
            captcha = Some(captcha::Client::new(
                std::env::var("HCAPTCHA_TOKEN").expect("HCAPTCHA_TOKEN Not Present")
            ));
        } else {
            log::warn!("Captcha is DISABLED");
        }
        
        Self {
            database,
            captcha,
            tokens: sync::RwLock::new(HashMap::new()),
            exchange: sync::RwLock::new(HashMap::new()),
            files: files::Files::new()
        }
    }
    
    pub fn new_data(database: Database, include_captcha: bool) -> crate::AppData {
        crate::AppData::new(Self::new(database, include_captcha))
    }
    
    pub async fn login(&self, login: String, password: String) -> Result<db::user::User, Box<dyn std::error::Error>> {
        if let Some(user) = self.database.users.find_one(
            bson::doc! {
                "login": login.to_lowercase()
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
    
    pub async fn validate(&self, req: &HttpRequest, id: Option<Uuid>) -> Option<(Uuid, Token)> {
        let token = Uuid::parse_str(&get_token(req)?).ok()?;
        let tokens = self.tokens.read().await;
        let user = tokens.get(&token)?;
        
        if let Some(id) = id {
            if user.acc == Some(id) {
                return Some((token, user.clone()))
            }
            return None
        }
        
        Some((token, user.clone()))
    }
    
    pub async fn validate_exchange(&self, exchange_code: String) -> Option<ExchangeCode> {
        let code = Uuid::parse_str(&exchange_code).ok()?;
        let user = {
            let codes = self.exchange.read().await;
            codes.get(&code)?.clone()
        };
        
        {
            let mut codes = self.exchange.write().await;
            codes.remove(&code);
        }
        
        Some(user)
    }
    
    pub async fn delete_token(&self, token: Uuid) {
        let mut tokens = self.tokens.write().await;
        
        tokens.remove(&token);
    }
    
    pub async fn delete_tokens(&self, user: Uuid, exception: Option<Uuid>) {
        // i guess this is more resourceful?
        let values = {
            let tokens = self.tokens.read().await;
            let mut values = Vec::<Uuid>::new();
            
            for (token, data) in tokens.iter() {
                let token = token.clone();
                
                if
                    data.acc == Some(user) &&
                    Some(token) != exception
                {
                    values.push(token);
                }
            }
            
            values
        };
        
        let mut tokens = self.tokens.write().await;
        
        for value in values {
            tokens.remove(&value);
        }
    }
    
    pub async fn delete_tokens_req(&self, req: &HttpRequest, exception: bool) -> Option<()> {
        let (token, data) = self.validate(req, None).await?;
        
        self.delete_tokens(data.acc?, match exception {
            true => Some(token),
            false => None
        }).await;
        
        Some(())
    }
    
    pub async fn new_exchange(&self, id: Uuid) -> Uuid {
        let data = ExchangeCode {
            acc: id
        };
        
        let code = uuid::Uuid::new_v4();
        
        {
            let mut codes = self.exchange.write().await;
            codes.insert(code, data);
        }
        
        code
    }
    
    pub async fn new_token(&self, id: Option<Uuid>, admin: bool) -> Uuid {
        let data = Token {
            acc: id,
            adm: admin
        };
        let token = uuid::Uuid::new_v4();
        
        {
            let mut tokens = self.tokens.write().await;
            tokens.insert(token, data);
        }
        
        token
    }
}