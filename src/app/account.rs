use actix_web::*;
use crate::models::fortnite::account::*;
use crate::models::error::*;
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
    app: crate::AppData,
    body: web::Json<TestCreate>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let body = body.into_inner();
    
    app.database.new_user(body.login, body.display_name, body.password).await?;
    
    Ok(HttpResponse::Ok())
}

// TEMP ENDPOINT to login
#[post("/test_login")]
pub async fn test_login(
    app: crate::AppData
) -> impl Responder {
    let token = app.new_token(None, true).await;
    
    HttpResponse::Ok().body(token.unwrap().to_simple().to_string())
}

#[post("/api/oauth/token")]
pub async fn oauth_token(
    app: crate::AppData,
    form: web::Form<OAuthForm>,
    req: HttpRequest
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let form = form.into_inner();
    
    match form.grant_type.as_str() {
        "password" => {
            let login = form.username.unwrap();
            let password = form.password.unwrap();
            
            let user = app.login(login, password).await?;
            let token = app.new_token(Some(user.id), false).await.unwrap();
            
            Ok(
                HttpResponse::Ok().json(
                    OAuthToken::new(token, &req, user)
                )
            )
        }
        _ => Ok(HttpResponse::BadRequest().json(
            EpicError::new()
                .error_code("errors.com.epicgames.common.oauth.unsupported_grant_type")
                .error_message(&("Unsupported grant type: ".to_owned() + &form.grant_type))
                .message_vars(vec![])
                .numeric_error_code(18059)
                .error_description(&("Unsupported grant type: ".to_owned() + &form.grant_type))
                .error("unsupported_grant_type")
        ))
    }
}