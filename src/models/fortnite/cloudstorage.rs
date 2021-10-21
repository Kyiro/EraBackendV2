use crate::models::db::cloudstorage;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")] 
pub struct SystemEntry {
    pub unique_filename: String,
    pub filename: String,
    pub hash: String,
    pub hash256: String,
    pub length: usize,
    pub content_type: String,
    pub uploaded: String,
    pub storage_type: String,
    pub do_not_cache: bool,
}

impl SystemEntry {
    pub fn new(id: String, file: cloudstorage::CloudStorageData) -> Self {
        SystemEntry {
            unique_filename: id,
            filename: file.filename,
            hash: file.hash,
            hash256: file.hash256,
            length: file.length,
            content_type: String::from("application/octet-stream"),
            uploaded: file.uploaded.to_chrono().to_rfc3339_opts(SecondsFormat::Secs, true),
            storage_type: String::from("S3"),
            do_not_cache: true,
        }
    }
}
