pub mod athena;
pub mod common_core;
pub mod common_public;
pub mod profile0;

use serde::{Deserialize, Serialize};
use std::*;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub profile_revision: i32,
    pub profile_id: String,
    pub profile_changes_base_revision: i32,
    pub profile_changes: Vec<ProfileChanges>,
    pub profile_command_revision: i32,
    pub server_time: String,
    pub response_version: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProfileChanges {
    Full(FullProfile)
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullProfile {
    pub change_type: String,
    pub profile: FullProfileInner,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullProfileInner {
    #[serde(rename = "_id")]
    pub _id: String,
    pub created: String,
    pub updated: String,
    pub rvn: i32,
    pub wipe_number: i32,
    pub account_id: String,
    pub profile_id: String,
    pub version: String,
    pub items: collections::HashMap<String, Item>,
    pub stats: Stats,
    pub command_revision: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Item {
    CosmeticItem(athena::Cosmetic)
}

#[derive(Serialize, Deserialize)]
pub struct Stats {
    pub attributes: StatsAttributes,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatsAttributes {
    Athena(athena::Attributes),
    CommonCore(common_core::Attributes),
    CommonPublic(common_public::Attributes),
    Profile0(profile0::Attributes)
}