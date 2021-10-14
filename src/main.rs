pub mod app;
pub mod db;
pub mod models;
pub mod utils;

use actix_web::*;
use std::*;

pub type AppData = web::Data<models::app::AppData>;
pub const SECRET: &'static str = "khf!J=tr8G(7KU@:";

pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().json(models::errors::EpicError::not_found())
}

#[get("/test")]
pub async fn validate(req: HttpRequest, app: AppData) -> impl Responder {
    match app.validate(&req, None).await {
        Some(()) => HttpResponse::Ok(),
        None => HttpResponse::Unauthorized()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().unwrap();
    utils::init_logger();
    
    log::info!("era-backend v2 ({}) by Kyiro", env!("CARGO_PKG_VERSION"));
    
    let database = db::Database::new(
        &env::var("MONGODB_URL").expect("MONGODB_URL not found"),
        &env::var("MONGODB_NAME").expect("MONGODB_NAME not found")
    ).await.unwrap();
    
    let data = models::app::AppData::new_data(database);
    
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(
                web::scope("/fortnite")
                .service(app::cloudstorage::user)
                .service(app::cloudstorage::user_file_get)
                .service(app::cloudstorage::user_file_put)
            )
            .service(
                web::scope("/account")
                .service(app::account::test_create)
                .service(app::account::test_login)
                .service(app::account::oauth_token)
            )
            .service(validate)
            .default_service(web::to(not_found))
    })
    .bind(format!(
        "0.0.0.0:{}",
        env::var("PORT").unwrap_or("60101".to_string())
    ))?
    .run()
    .await
}