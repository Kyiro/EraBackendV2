use crate::models::*;
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
    pub async fn new(url: &str, app_name: &str) -> Result<Database, Box<dyn error::Error>> {
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
    
    pub async fn new_user(&self, display_name: String, password: String) -> Result<Uuid, Box<dyn error::Error>> {
        let uuid = Uuid::new_v4();
        
        if let Ok(None) = self.users.find_one(
            bson::doc! { "display_name": display_name.clone() },
            None
        ).await {
            self.athena.insert_one(
                athena::Profile::new(uuid),
                None
            ).await?;
            
            self.cloudstorage.insert_one(
                cloudstorage::CloudStorageEntry::new(uuid),
                None
            ).await?;
            
            self.users.insert_one(
                user::User::new(uuid, display_name, password),
                None
            ).await?;
        } else {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "User already exists or an internal error occured"
            )))
        }
        
        Ok(uuid)
    }
}