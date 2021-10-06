use mongodb::bson;
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use sha2::{Digest, Sha256};
use std::*;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudStorageEntry {
    // system id is Uuid::nil()
    pub id: Uuid,
    pub files: Vec<CloudStorageData>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudStorageData {
    pub id: Uuid,
    pub filename: String,
    pub hash: String,
    pub hash256: String,
    pub length: usize,
    pub uploaded: bson::DateTime
}

impl CloudStorageEntry {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            files: Vec::new()
        }
    }
}

impl CloudStorageData {
    pub fn new(id: Uuid, filename: String, data: Vec<u8>) -> Self {
        let mut sha1 = Sha1::new();
        let mut sha256 = Sha256::new();
        sha1.update(&data);
        sha256.update(&data);
        let sha1 = sha1.finalize();
        let sha256 = sha256.finalize();

        CloudStorageData {
            id,
            filename,
            hash: format!("{:x}", sha1),
            hash256: format!("{:x}", sha256),
            length: data.len(),
            uploaded: bson::DateTime::now()
        }
    }
    
    pub fn from_str(id: Uuid, filename: String, data: String) -> Self {
        Self::new(id, filename, String::into_bytes(data))
    }
}
