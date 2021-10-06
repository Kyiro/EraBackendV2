pub mod app;
pub mod db;
pub mod models;
pub mod utils;

use actix_web::*;
use std::*;

#[get("/")]
pub async fn index(app: web::Data<models::app::AppData>) -> impl Responder {
    app.database.users.insert_one(
        models::user::User::new(uuid::Uuid::new_v4(), String::new(), String::new()),
        None
    ).await.unwrap();
    
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().unwrap();
    utils::init_logger();
    
    let database = db::Database::new(
        &env::var("MONGODB_URL").expect("MONGODB_URL not found"),
        &env::var("MONGODB_NAME").expect("MONGODB_NAME not found")
    ).await.unwrap();
    
    let data = models::app::AppData::new_data(database);
    
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}