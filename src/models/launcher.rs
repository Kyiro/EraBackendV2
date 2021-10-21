use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct LoginForm {
    pub login: Option<String>,
    pub password: Option<String>,
    pub launcher_token: Option<Uuid>
}

#[derive(Deserialize, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub account_id: String,
    pub display_name: String,
    pub launcher_token: String
}

#[derive(Deserialize, Serialize)]
pub struct ExchangeResponse {
    pub code: String
}

#[derive(Deserialize, Serialize)]
pub struct RegisterForm {
    pub captcha: Option<String>,
    pub display_name: String,
    pub login: String,
    pub password: String
}