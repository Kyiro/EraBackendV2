use actix_web::*;
use crate::models::mcp::*;
use crate::models::errors::EpicError;
use crate::utils;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Query {
    #[serde(rename = "profileId")]
    pub profile_id: String,
    pub rvn: i32,
}

#[derive(Deserialize)]
pub struct QueryProfile {}

#[post("/api/game/v2/profile/{id}/client/QueryProfile")]
pub async fn query_profile(
    app: crate::AppData,
    _: web::Json<QueryProfile>,
    query: web::Query<Query>,
    id: web::Path<Uuid>,
    req: HttpRequest,
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let query = query.into_inner();
    let id = id.into_inner();
    let build = utils::get_build(&req).unwrap_or(utils::Build::default());
    
    match app.validate(&req, Some(id)).await {
        Some(_) => (),
        None => return Ok(HttpResponse::Unauthorized().json(
            EpicError::permission(String::new(), String::new())
        ))
    };
    
    let user = app.database.users.find_one(
        bson::doc! {
            "id": id
        },
        None
    ).await?.ok_or("Can't find user profile")?;
    
    match query.profile_id.as_str() {
        "athena" => {
            let athena = app.database.athena.find_one(
                bson::doc! {
                    "id": id
                },
                None
            ).await?.ok_or("Can't find athena profile")?;
            
            Ok(HttpResponse::Ok().json(Profile::new(
                query.profile_id.clone(),
                vec![ProfileChanges::Full(FullProfile::new_athena(
                    athena,
                    app.files.cosmetics.clone(),
                    build.season,
                    user
                ))],
                None
            )))
        },
        "profile0" => Ok(HttpResponse::Ok().json(Profile::new(
            query.profile_id.clone(),
            vec![ProfileChanges::Full(FullProfile::new_profile0(user))],
            None
        ))),
        "common_core" => Ok(HttpResponse::Ok().json(Profile::new(
            query.profile_id.clone(),
            vec![ProfileChanges::Full(FullProfile::new_common_core(user))],
            None
        ))),
        "common_public" => Ok(HttpResponse::Ok().json(Profile::new(
            query.profile_id.clone(),
            vec![ProfileChanges::Full(FullProfile::new_common_public(user))],
            None
        ))),
        _ => Ok(HttpResponse::Ok().json(Profile::new(
            query.profile_id.clone(),
            vec![ProfileChanges::Full(FullProfile::new(user, &query.profile_id))],
            Some(query.rvn)
        ))),
    }
}

#[post("/api/game/v2/profile/{id}/client/{action}")]
pub async fn other(query: web::Query<Query>) -> impl Responder {
    let query = query.into_inner();
    
    HttpResponse::Ok().json(Profile::new(
        query.profile_id,
        Vec::new(),
        Some(query.rvn)
    ))
}