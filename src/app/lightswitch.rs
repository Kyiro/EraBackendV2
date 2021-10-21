use actix_web::*;
use serde_json::json;

#[get("/api/service/bulk/status")]
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

#[get("/api/service/Fortnite/status")]
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