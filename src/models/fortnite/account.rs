use actix_web::HttpRequest;
use chrono::prelude::*;
use chrono::Duration;
use uuid::Uuid;
use crate::models::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct OAuthForm {
    pub grant_type: String,
    pub code: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub exchange_code: Option<String>
}

#[derive(Deserialize, Serialize)]
pub struct OAuthToken {
    pub access_token: String,
    pub expires_in: i32,
    pub expires_at: String,
    pub token_type: String,
    pub refresh_token: String,
    pub refresh_expires: i32,
    pub refresh_expires_at: String,
    pub account_id: String,
    pub client_id: String,
    pub internal_client: bool,
    pub client_service: String,
    pub scope: Vec<String>,
    
    #[serde(rename = "displayName")]
    pub display_name: String,
    
    pub app: String,
    pub in_app_id: String,
}

#[derive(Deserialize)]
pub struct PublicAccount {
    #[serde(rename = "accountId")]
    pub account_id: Uuid,
}

#[derive(Deserialize)]
pub struct SessionsKill {
    #[serde(rename = "killType")]
    pub kill_type: String,
}

#[derive(Deserialize, Serialize)]
pub struct ClientCredentials {
    pub access_token: String,
    pub expires_in: i32,
    pub expires_at: String,
    pub token_type: String,
    pub client_id: String,
    pub internal_client: bool,
    pub client_service: String
}

impl OAuthToken {
    pub fn new(token: uuid::Uuid, req: &HttpRequest, user: db::user::User) -> Option<Self> {
        let basic = app::get_basic(req)?;
        
        Some(Self {
            access_token: token.to_simple().to_string(),
            expires_in: 28800,
            expires_at: (Utc::now() + Duration::minutes(28800))
                .to_rfc3339_opts(SecondsFormat::Secs, true),
            token_type: String::from("bearer"),
            refresh_token: token.to_simple().to_string(),
            refresh_expires: 115200,
            refresh_expires_at: (Utc::now() + Duration::minutes(115200))
                .to_rfc3339_opts(SecondsFormat::Secs, true),
            account_id: user.id.to_simple().to_string(),
            client_id: basic.client_id,
            internal_client: true,
            client_service: String::from("fortnite"),
            scope: Vec::new(),
            display_name: user.display_name,
            app: String::from("fortnite"),
            in_app_id: user.id.to_simple().to_string(),
        })
    }
}

impl ClientCredentials {
    pub fn new(token: uuid::Uuid, req: &HttpRequest) -> Option<Self> {
        let basic = app::get_basic(req)?;
        
        Some(Self {
            access_token: token.to_simple().to_string(),
            expires_in: 14400,
            expires_at: (Utc::now() + Duration::minutes(14400))
                .to_rfc3339_opts(SecondsFormat::Secs, true),
            token_type: String::from("bearer"),
            client_id: basic.client_id,
            internal_client: true,
            client_service: String::from("fortnite")
        })
    }
}