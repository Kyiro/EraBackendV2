use actix_web::*;
use crate::models::fortnite::account::*;
use crate::models::errors::*;
use std::*;
use serde_json::json;
use uuid::Uuid;

#[post("/api/oauth/token")]
pub async fn oauth_token(
    app: crate::AppData,
    form: web::Form<OAuthForm>,
    req: HttpRequest
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let form = form.into_inner();
    
    match form.grant_type.as_str() {
        "password" => {
            let login = form.username.unwrap().split("@").collect::<Vec<&str>>()[0].to_string();
            let password = form.password.unwrap();
            
            let user = app.login(login, password).await?;
            let token = app.new_token(Some(user.id), false).await;
            
            Ok(HttpResponse::Ok().json(
                OAuthToken::new(token, &req, user)
            ))
        },
        "client_credentials" => {
            Ok(HttpResponse::Ok().json(
                ClientCredentials::new(uuid::Uuid::nil(), &req)
            ))
        },
        "exchange_code" => {
            let exchange_code = form.exchange_code.unwrap();
            
            let code = match app.validate_exchange(exchange_code).await {
                Some(code) => code,
                None => return Ok(HttpResponse::Unauthorized().json(
                    EpicError::new()
                    .error_code("errors.com.epicgames.account.oauth.exchange_code_not_found")
                    .error_message("Sorry the exchange code you supplied was not found. It is possible that it was no longer valid")
                    .message_vars(Vec::new())
                    .numeric_error_code(18057)
                    .originating_service("com.epicgames.account.public")
                    .error_description("Sorry the exchange code you supplied was not found. It is possible that it was no longer valid")
                    .error("invalid_grant")
                ))
            };
            
            let user = app.database.users.find_one(
                bson::doc! {
                    "id": code.acc
                },
                None
            ).await?.ok_or("Account not found")?;
            let token = app.new_token(Some(code.acc), false).await;
            
            Ok(HttpResponse::Ok().json(
                OAuthToken::new(token, &req, user)
            ))
        },
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

#[get("/api/public/account/{id}")]
pub async fn public_account(
    app: crate::AppData,
    id: web::Path<Uuid>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let id = id.into_inner();
    
    let user = app.database.users.find_one(
        bson::doc! {
            "id": id
        },
        None
    ).await?.ok_or("Can't find user")?;
    
    Ok(HttpResponse::Ok().json(json!({
        "id": id.to_simple().to_string(),
        "displayName": user.display_name,
        "name": "Project",
        "email": user.login + "@erafn.xyz",
        "failedLoginAttempts": 0,
        "lastFailedLogin": "2021-01-22T23:00:00.000Z",
        "lastLogin": "2021-01-22T23:00:00.000Z",
        "numberOfDisplayNameChanges": 1,
        "ageGroup": "UNKNOWN",
        "headless": false,
        "country": "PL",
        "lastName": "Era",
        "preferredLanguage": "en",
        "lastDisplayNameChange": "2021-01-22T23:00:00.000Z",
        "canUpdateDisplayName": true,
        "tfaEnabled": false,
        "emailVerified": true,
        "minorVerified": false,
        "minorExpected": false,
        "minorStatus": "UNKNOWN"
    })))
}

#[get("/api/public/account")]
pub async fn public_account_query(
    app: crate::AppData,
    query: web::Query<PublicAccount>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let query = query.into_inner();
    
    let user = app.database.users.find_one(
        bson::doc! {
            "id": query.account_id
        },
        None
    ).await?.ok_or("Can't find user")?;
    
    Ok(HttpResponse::Ok().json(json!([{
        "id": user.id.to_simple().to_string(),
        "displayName": user.display_name,
        "externalAuths": {}
    }])))
}

#[get("/api/public/account/{i}/externalAuths")]
pub async fn external_auths() -> impl Responder {
    HttpResponse::Ok().json(Vec::<i8>::new())
}

#[get("/api/accounts/{i}/metadata")]
pub async fn accounts_metadata() -> impl Responder {
    HttpResponse::Ok().json(json!({}))
}

#[delete("/api/oauth/sessions/kill")]
pub async fn kill_sessions(
    app: crate::AppData,
    query: web::Query<SessionsKill>,
    req: HttpRequest
) -> impl Responder {
    let query = query.into_inner();
    
    match query.kill_type.as_str() {
        "OTHERS_ACCOUNT_CLIENT_SERVICE" => {
            app.delete_tokens_req(&req, true).await;
        },
        _ => ()
    };
    
    HttpResponse::NoContent()
}

#[delete("/api/oauth/sessions/kill/{token}")]
pub async fn kill_sessions_id(
    app: crate::AppData,
    token: web::Path<Uuid>
) -> impl Responder {
    let token = token.into_inner();
    app.delete_token(token).await;
    HttpResponse::NoContent()
}

#[get("/api/epicdomains/ssodomains")]
pub async fn ssodomains() -> impl Responder {
    HttpResponse::Ok().json(Vec::<i8>::new())
}