use bson::serde_helpers::uuid_as_binary;
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use sha2::{Digest, Sha256};
use std::*;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CloudStorageEntry {
    // system id is Uuid::nil()
    #[serde(with = "uuid_as_binary")]
    pub id: Uuid,
    pub files: collections::HashMap<String, CloudStorageData>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CloudStorageData {
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
            files: collections::HashMap::new()
        }
    }
}

impl CloudStorageData {
    pub fn new(filename: String, data: Vec<u8>) -> Self {
        let mut sha1 = Sha1::new();
        let mut sha256 = Sha256::new();
        sha1.update(&data);
        sha256.update(&data);
        let sha1 = sha1.finalize();
        let sha256 = sha256.finalize();

        CloudStorageData {
            filename,
            hash: format!("{:x}", sha1),
            hash256: format!("{:x}", sha256),
            length: data.len(),
            uploaded: bson::DateTime::now()
        }
    }
    
    pub fn from_str(filename: String, data: String) -> Self {
        Self::new(filename, String::into_bytes(data))
    }
}
