use actix_web::*;
use crate::models::*;
use std::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TestCreate {
    login: String,
    display_name: String,
    password: String
}

// TEMP ENDPOINT to register
#[post("/test_create")]
pub async fn test_create(
    app: web::Data<app::AppData>,
    body: web::Json<TestCreate>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let body = body.into_inner();
    
    app.database.new_user(body.login, body.display_name, body.password).await?;
    
    Ok(HttpResponse::Ok())
}

// TEMP ENDPOINT to login
#[post("/test_login")]
pub async fn test_login(
    app: web::Data<app::AppData>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let token = app.new_token(None, true);
    
    Ok(HttpResponse::Ok().body(token.await.unwrap()))
}