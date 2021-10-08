pub mod app;
pub mod db;
pub mod middleware;
pub mod models;
pub mod utils;

use actix_web::*;
use std::*;

pub const SECRET: &'static str = "khf!J=tr8G(7KU@:";

#[get("/error")]
pub async fn error(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json(models::error::EpicError::unauthorized(&req))
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
                web::scope("/test")
                .service(error)
            )
            .service(
                web::scope("/account")
                .service(app::account::test_create)
                .service(app::account::test_login)
            )
    })
    .bind(format!(
        "0.0.0.0:{}",
        env::var("PORT").unwrap_or("60101".to_string())
    ))?
    .run()
    .await
}