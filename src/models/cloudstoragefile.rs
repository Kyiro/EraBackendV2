use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudStorageFile {
    pub id: Uuid,
    pub owner: Uuid,
    pub data: Vec<u8>
}

impl CloudStorageFile {
    pub fn new(id: Uuid, owner: Uuid, data: Vec<u8>) -> Self {
        Self {
            id,
            owner,
            data
        }
    }
}