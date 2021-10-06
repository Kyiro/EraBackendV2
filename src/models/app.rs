use actix_web::*;
use crate::db::Database;

pub struct AppData {
    pub database: Database
}

impl AppData {
    pub fn new(database: Database) -> Self {
        Self {
            database
        }
    }
    
    pub fn new_data(database: Database) -> web::Data<AppData> {
        web::Data::new(Self::new(database))
    }
}