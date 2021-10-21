use crate::models::db::*;
use mongodb::{bson, Collection, Client, options::ClientOptions};
use uuid::Uuid;
use std::*;

// more like DataBASED
pub struct Database {
    pub athena: Collection<athena::Profile>,
    pub cloudstorage: Collection<cloudstorage::CloudStorageEntry>,
    pub cloudstorage_files: Collection<cloudstoragefile::CloudStorageFile>,
    pub users: Collection<user::User>
}

impl Database {
    pub async fn new(url: &str, app_name: &str) -> Result<Database, Box<dyn std::error::Error>> {
        let client_options = {
            let mut options = ClientOptions::parse(url).await?;
            
            options.app_name = Some(app_name.to_string());
            
            options
        };
        
        let client = Client::with_options(client_options)?;
        let db = client.database(app_name);
        
        Ok(Self {
            athena: db.collection("athena"),
            cloudstorage: db.collection("cloudstorage"),
            cloudstorage_files: db.collection("cloudstoragefiles"),
            users: db.collection("users")
        })
    }
    
    pub async fn new_user(&self, login: String, display_name: String, password: String) -> Result<Uuid, Box<dyn std::error::Error>> {
        let uuid = Uuid::new_v4();
        
        let login = login.to_lowercase();
        let display_name = display_name.trim().to_string();
        let password = password.trim().to_string();
        
        // sorry for this spaghetti
        // i hope danii enjoys it though
        
        for c in login.chars() {
            if !c.is_alphanumeric() {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "The login can only contain letters and numbers"
                )))
            }
        }
        
        if login.len() < 3 {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidData,
                "The login must be at least 3 characters long"
            )))
        }
        
        if display_name.len() < 1 {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidData,
                "The username must be at least 1 characters long"
            )))
        }
        
        if password.len() < 4 {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidData,
                "The password must be at least 4 characters long"
            )))
        }
        
        // check if the display name is taken
        if let Some(_) = self.users.find_one(
            bson::doc! {
                "display_name": display_name.clone()
            },
            None
        ).await? {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "That display name is taken"
            )))
        }
        
        // check if the login is available
        if let None = self.users.find_one(
            bson::doc! {
                "login": login.clone()
            },
            None
        ).await? {
            self.athena.insert_one(
                athena::Profile::new(uuid),
                None
            ).await?;
            
            self.cloudstorage.insert_one(
                cloudstorage::CloudStorageEntry::new(uuid),
                None
            ).await?;
            
            self.users.insert_one(
                user::User::new(uuid, login.clone(), display_name, password),
                None
            ).await?;
        } else {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "User already exists"
            )))
        }
        
        log::info!("New Account: {}", login);
        
        Ok(uuid)
    }
}