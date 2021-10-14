use bson::serde_helpers::uuid_as_binary;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudStorageFile {
    pub id: String,
    #[serde(with = "uuid_as_binary")]
    pub owner: Uuid,
    pub data: Vec<u8>
}

impl CloudStorageFile {
    pub fn new(id: String, owner: Uuid, data: Vec<u8>) -> Self {
        Self {
            id,
            owner,
            data
        }
    }
}