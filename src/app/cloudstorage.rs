use actix_web::*;
use crate::models::*;
use uuid::Uuid;

// clean up this mess...

#[get("/api/cloudstorage/user/{id}")]
pub async fn user(
    app: crate::AppData,
    req: HttpRequest,
    id: web::Path<Uuid>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let id = id.into_inner();
    
    match app.validate(&req, Some(id)).await {
        Some(_) => (),
        None => return Ok(
            HttpResponse::Unauthorized().json(
                errors::EpicError::permission(
                    format!("fortnite:cloudstorage:user:{}", id.to_simple().to_string()),
                    String::from("READ")
                )
            )
        )
    };
    
    let mut entries = Vec::<fortnite::cloudstorage::SystemEntry>::new();
    
    let cloudstorage = app.database.cloudstorage.find_one(
        bson::doc! {
            "id": id
        },
        None
    ).await?.ok_or("None")?;
    
    for (id, file) in cloudstorage.files {
        let id = String::from_utf8(base64::decode(id)?)?;
        
        entries.push(fortnite::cloudstorage::SystemEntry::new(id, file));
    }
    
    Ok(HttpResponse::Ok().json(entries))
}

#[get("/api/cloudstorage/user/{id}/{file}")]
pub async fn user_file_get(
    app: crate::AppData,
    req: HttpRequest,
    path: web::Path<(Uuid, String)>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let (id, file) = path.into_inner();
    
    match app.validate(&req, Some(id)).await {
        Some(_) => (),
        None => return Ok(
            HttpResponse::Unauthorized().json(
                errors::EpicError::permission(
                    format!(
                        "fortnite:cloudstorage:user:{}:{}",
                        id.to_simple().to_string(),
                        file
                    ),
                    String::from("READ")
                )
            )
        )
    };
    
    if let Some(file) = app.database.cloudstorage_files.find_one(
        bson::doc! {
            "id": file,
            "owner": id
        },
        None
    ).await? {
        return Ok(HttpResponse::Ok().body(file.data.bytes))
    }
    
    Ok(HttpResponse::NoContent().into())
}

#[put("/api/cloudstorage/user/{id}/{file}")]
pub async fn user_file_put(
    app: crate::AppData,
    body: web::Bytes,
    req: HttpRequest,
    path: web::Path<(Uuid, String)>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let body = body.to_vec();
    let (id, file) = path.into_inner();
    let file_encoded = base64::encode(file.clone());
    
    match app.validate(&req, Some(id)).await {
        Some(_) => (),
        None => return Ok(
            HttpResponse::Unauthorized().json(
                errors::EpicError::permission(
                    format!(
                        "fortnite:cloudstorage:user:{}:{}",
                        id.to_simple().to_string(),
                        file.clone()
                    ),
                    String::from("PUT")
                )
            )
        )
    };
    
    if let None = app.database.cloudstorage_files.find_one(
        bson::doc! {
            "owner": id,
            "id": file.clone()
        },
        None
    ).await? {
        app.database.cloudstorage_files.insert_one(
            db::cloudstoragefile::CloudStorageFile::new(file.clone(), id, body.clone()),
            None
        ).await?;
    } else {
        app.database.cloudstorage_files.update_one(
            bson::doc! {
                "owner": id,
                "id": file.clone()
            },
            bson::doc! {
                "$set": {
                    "data": bson::Binary {
                        subtype: bson::spec::BinarySubtype::Generic,
                        bytes: body.clone()
                    }
                }
            },
            None
        ).await?;
    }
    
    app.database.cloudstorage.update_one(
        bson::doc! {
            "id": id
        },
        bson::doc! {
            "$set": {
                "files.".to_owned() + &file_encoded: bson::to_bson(&db::cloudstorage::CloudStorageData::new(file, body))?
            }
        },
        None
    ).await?;
    
    Ok(HttpResponse::Ok().into())
}

#[delete("/api/cloudstorage/user/{id}/{file}")]
pub async fn user_file_delete(
    app: crate::AppData,
    req: HttpRequest,
    path: web::Path<(Uuid, String)>
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let (id, file) = path.into_inner();
    let file_encoded = base64::encode(file.clone());
    
    match app.validate(&req, Some(id)).await {
        Some(_) => (),
        None => return Ok(
            HttpResponse::Unauthorized().json(
                errors::EpicError::permission(
                    format!(
                        "fortnite:cloudstorage:user:{}:{}",
                        id.to_simple().to_string(),
                        file.clone()
                    ),
                    String::from("DELETE")
                )
            )
        )
    };
    
    app.database.cloudstorage_files.delete_one(
        bson::doc! {
            "id": file,
            "owner": id
        },
        None
    ).await?;
    
    app.database.cloudstorage.update_one(
        bson::doc! {
            "id": id
        },
        bson::doc! {
            "$unset": {
                "files.".to_owned() + &file_encoded: {}
            }
        },
        None
    ).await?;
    
    Ok(HttpResponse::Ok().into())
}

#[get("/api/cloudstorage/system/config")]
pub async fn system_config() -> impl Responder {
    HttpResponse::NoContent().json(Vec::<i8>::new())
}

#[get("/api/cloudstorage/system")]
pub async fn system() -> impl Responder {
    HttpResponse::Ok().json(Vec::<i8>::new())
}