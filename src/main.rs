pub mod app;
pub mod db;
pub mod models;
pub mod utils;

use actix_web::*;
use std::*;

pub type AppData = web::Data<models::app::AppData>;

pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().json(models::errors::EpicError::not_found())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().unwrap();
    utils::init_logger();
    
    log::info!("era-backend v2 ({}) by Kyiro", env!("CARGO_PKG_VERSION"));
    
    let database = db::Database::new(
        &env::var("MONGODB_URL").expect("MONGODB_URL not found"),
        &env::var("MONGODB_NAME").expect("MONGODB_NAME not found")
    ).await.expect("Failed to load DB");
    
    let data = models::app::AppData::new_data(database,
        env::var("HCAPTCHA_TOKEN").is_ok()
    );
    
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(
                web::scope("/account")
                .service(app::account::accounts_metadata)
                .service(app::account::external_auths)
                .service(app::account::kill_sessions)
                .service(app::account::kill_sessions_id)
                .service(app::account::oauth_token)
                .service(app::account::public_account)
                .service(app::account::public_account_query)
                .service(app::account::ssodomains)
            )
            .service(
                web::scope("/content")
                .service(app::content::fortnite_game)
                .service(app::content::fortnite_game_)
            )
            .service(
                web::scope("/fortnite")
                .service(app::cloudstorage::user)
                .service(app::cloudstorage::user_file_get)
                .service(app::cloudstorage::user_file_put)
                .service(app::cloudstorage::user_file_delete)
                .service(app::cloudstorage::system)
                .service(app::cloudstorage::system_config)
                .service(app::fortnite::enabled_features)
                .service(app::fortnite::find_player)
                .service(app::fortnite::fortnite_version)
                .service(app::fortnite::keychain)
                .service(app::fortnite::play_on_platform)
                .service(app::fortnite::receipts)
                .service(app::fortnite::timeline)
                .service(app::fortnite::twitch)
                .service(app::fortnite::version_check)
                .service(app::fortnite::version_check_v2)
                .service(app::fortnite::world_info)
                .service(app::mcp::query_profile)
                .service(app::mcp::equip_battle_royale_customization)
                .service(app::mcp::set_item_favorite_status_batch)
                .service(app::mcp::other)
            )
            .service(
                web::scope("/launcher")
                .service(app::launcher::exchange)
                .service(app::launcher::form)
                .service(app::launcher::login)
                .service(app::launcher::register)
            )
            .service(app::other::blocklist)
            .service(app::other::bulk_status)
            .service(app::other::datarouter)
            .service(app::other::eulatracking)
            .service(app::other::fortnite_status)
            .service(app::other::friends)
            .service(app::other::party_user)
            .service(app::other::recent_players)
            .service(app::other::settings)
            .service(app::other::waitingroom)
            .default_service(web::to(not_found))
    })
    .bind(format!(
        "0.0.0.0:{}",
        env::var("PORT").unwrap_or("60101".to_string())
    ))?
    .run()
    .await
}