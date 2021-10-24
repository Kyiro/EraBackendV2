use actix_web::*;
use crate::models::files::Item;
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
            EpicError::permission(
                format!("fortnite:profile:{}:commands", id.to_simple().to_string()),
                String::from("ALL")
            )
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

#[post("/api/game/v2/profile/{id}/client/EquipBattleRoyaleCustomization")]
pub async fn equip_battle_royale_customization(
    app: crate::AppData,
    body: web::Json<athena::EquipBattleRoyaleCustomization>,
    query: web::Query<Query>,
    id: web::Path<Uuid>,
    req: HttpRequest
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let body = body.into_inner();
    let query = query.into_inner();
    let id = id.into_inner();
    
    let idx = body.index.unwrap_or(0);
    let favorite_slot = if body.slot_name == "ItemWrap" {
        String::from("itemwraps")
    } else { body.slot_name.to_lowercase() };
    let item = app.files.get_cosmetic(
        body.item_to_slot.clone()
    ).unwrap_or(Item::from_body(&body));
    
    let mut changes: Vec<ProfileChanges> = Vec::new();
    
    match app.validate(&req, Some(id)).await {
        Some(_) => (),
        None => return Ok(HttpResponse::Unauthorized().json(
            EpicError::permission(
                format!("fortnite:profile:{}:commands", id.to_simple().to_string()),
                String::from("ALL")
            )
        ))
    };
    
    let mut user = app.database.athena.find_one(
        bson::doc! {
            "id": id
        },
        None
    ).await?.ok_or("No Athena")?;
    
    // if -1 (or below)
    if idx < 0 {
        let wraps = &mut user.locker.itemwrap;
        
        for i in 0..wraps.len() {
            wraps[i] = body.item_to_slot.clone();
        }
    } else {
        let idx = idx as usize;
        
        let slot = match body.slot_name.as_str() {
            "Character" => &mut user.locker.character,
            "Dance" => &mut user.locker.dance[idx],
            "ItemWrap" => &mut user.locker.itemwrap[idx],
            "Backpack" => &mut user.locker.backpack,
            "Pickaxe" => &mut user.locker.pickaxe,
            "Glider" => &mut user.locker.glider,
            "SkyDiveContrail" => &mut user.locker.skydivecontrail,
            "MusicPack" => &mut user.locker.musicpack,
            "LoadingScreen" => &mut user.locker.loadingscreen,
            _ => &mut user.locker.character,
        };
        
        *slot = body.item_to_slot.clone();
    }
    
    if let Some(variants) = body.variants {
        if variants.len() != 0 {
            let variants = athena::Variant::new(variants, item.variants);
            
            changes.push(ProfileChanges::Changed(AttrChanged {
                change_type: String::from("itemAttrChanged"),
                item_id: body.item_to_slot.clone(),
                attribute_name: String::from("variants"),
                attribute_value: Attributes::Variants(variants.clone())
            }));
            
            let slot = match body.slot_name.as_str() {
                "Character" => &mut user.locker.character_variants,
                "Backpack" => &mut user.locker.backpack_variants,
                "Pickaxe" => &mut user.locker.pickaxe_variants,
                "Glider" => &mut user.locker.glider_variants,
                _ => &mut user.locker.character_variants,
            };
            
            *slot = variants;
        }
    }
    
    app.database.athena.update_one(
        bson::doc! {
            "id": id
        },
        bson::doc! {
            "$set": {
                "locker": bson::to_bson(&user.locker)?
            }
        },
        None
    ).await?;
    
    changes.push(ProfileChanges::Stat(StatModified {
        change_type: String::from("statModified"),
        name: "favorite_".to_owned() + &favorite_slot,
        value: match body.slot_name.as_str() {
            "Dance" => StatValue::Vec(user.locker.dance.to_vec()),
            "ItemWrap" => StatValue::Vec(user.locker.itemwrap.to_vec()),
            _ => StatValue::String(body.item_to_slot.clone())
        },
    }));
    
    Ok(HttpResponse::Ok().json(Profile::new(
        String::from("athena"),
        changes,
        Some(query.rvn)
    )))
}

#[post("/api/game/v2/profile/{id}/client/SetItemFavoriteStatusBatch")]
pub async fn set_item_favorite_status_batch(
    app: crate::AppData,
    body: web::Json<athena::SetItemFavoriteStatusBatch>,
    query: web::Query<Query>,
    id: web::Path<Uuid>,
    req: HttpRequest
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let body = body.into_inner();
    let query = query.into_inner();
    let id = id.into_inner();
    
    let mut changes: Vec<ProfileChanges> = Vec::new();
    
    match app.validate(&req, Some(id)).await {
        Some(_) => (),
        None => return Ok(HttpResponse::Unauthorized().json(
            EpicError::permission(
                format!("fortnite:profile:{}:commands", id.to_simple().to_string()),
                String::from("ALL")
            )
        ))
    };
    
    let mut user = app.database.athena.find_one(
        bson::doc! {
            "id": id
        },
        None
    ).await?.ok_or("No Athena")?;
    
    for idx in 0..body.item_fav_status.len() {
        let (status, id) = (body.item_fav_status[idx], body.item_ids[idx].clone());
        
        if status == true {
            user.favourites.push(id.clone());
        } else {
            user.favourites = user.favourites
                .into_iter()
                .filter(|i| **i != id)
                .collect()
        }
        
        changes.push(ProfileChanges::Changed(AttrChanged {
            change_type: String::from("itemAttrChanged"),
            item_id: id,
            attribute_name: String::from("favorite"),
            attribute_value: Attributes::Bool(status),
        }));
    }
    
    app.database.athena.update_one(
        bson::doc! {
            "id": id
        },
        bson::doc! {
            "$set": {
                "favourites": bson::to_bson(&user.favourites)?
            }
        },
        None
    ).await?;
    
    Ok(HttpResponse::Ok().json(Profile::new(
        String::from("athena"),
        changes,
        Some(query.rvn)
    )))
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