use actix_web::*;
use serde_json::json;

#[get("/waitingroom/api/waitingroom")]
pub async fn waitingroom() -> impl Responder {
    HttpResponse::NoContent()
}

#[get("/party/api/v1/Fortnite/user/{u}")]
pub async fn party_user() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "current": [],
        "pending": [],
        "invites": [],
        "pings": []
    }))
}

#[get("/friends/api/public/friends/{i}")]
pub async fn friends() -> impl Responder {
    HttpResponse::Ok().json(Vec::<i8>::new())
}

#[get("/friends/api/public/list/fortnite/{i}/recentPlayers")]
pub async fn recent_players() -> impl Responder {
    HttpResponse::Ok().json(Vec::<i8>::new())
}

#[get("/friends/api/public/blocklist/{i}")]
pub async fn blocklist() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "blockedUsers": []
    }))
}

#[get("/friends/api/v1/{i}/settings")]
pub async fn settings() -> impl Responder {
    HttpResponse::Ok().json(json!({}))
}

#[post("/datarouter/api/v1/public/data")]
pub async fn datarouter() -> impl Responder {
    HttpResponse::NoContent().json(json!({}))
}

#[get("/eulatracking/api/shared/agreements/fn")]
pub async fn eulatracking() -> impl Responder {
    HttpResponse::NoContent().json(json!({}))
}

#[get("/lightswitch/api/service/bulk/status")]
pub async fn bulk_status() -> impl Responder {
    HttpResponse::Ok().json(json!([
        {
            "serviceInstanceId": "fortnite",
            "status": "UP",
            "message": "Project Era is UP",
            "maintenanceUri": null,
            "overrideCatalogIds": [
                "a7f138b2e51945ffbfdacc1af0541053"
            ],
            "allowedActions": [
                "PLAY",
                "DOWNLOAD"
            ],
            "banned": false,
            "launcherInfoDTO": {
                "appName": "Fortnite",
                "catalogItemId": "4fe75bbc5a674f4f9b356b5c90567da5",
                "namespace": "fn"
            }
        }
    ]))
}

#[get("/lightswitch/api/service/Fortnite/status")]
pub async fn fortnite_status() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "serviceInstanceId": "fortnite",
        "status": "UP",
        "message": "Project Era is UP",
        "maintenanceUri": null,
        "overrideCatalogIds": [
            "a7f138b2e51945ffbfdacc1af0541053"
        ],
        "allowedActions": [
            "PLAY",
            "DOWNLOAD"
        ],
        "banned": false,
        "launcherInfoDTO": {
            "appName": "Fortnite",
            "catalogItemId": "4fe75bbc5a674f4f9b356b5c90567da5",
            "namespace": "fn"
        }
    }))
}