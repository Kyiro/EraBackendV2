pub mod athena;
pub mod common_core;
pub mod common_public;
pub mod profile0;

use chrono::prelude::*;
use crate::models::db::user::User;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
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
    Full(FullProfile),
    Stat(StatModified),
    Changed(AttrChanged)
}

#[derive(Serialize, Deserialize)]
pub struct StatModified {
    #[serde(rename = "changeType")]
    pub change_type: String,
    pub name: String,
    pub value: StatValue,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatValue {
    Vec(Vec<String>),
    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttrChanged {
    pub change_type: String,
    pub item_id: String,
    pub attribute_name: String,
    pub attribute_value: Attributes,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Attributes {
    Bool(bool),
    Variants(Vec<athena::Variant>)
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
    Cosmetic(athena::Cosmetic),
    Other(Value)
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
    Profile0(profile0::Attributes),
    Other(Value)
}

impl Profile {
    pub fn new(profile_id: String, changes: Vec<ProfileChanges>, rvn: Option<i32>) -> Self {
        Self {
            profile_revision: rvn.unwrap_or(1) + 1,
            profile_id: profile_id,
            profile_changes_base_revision: rvn.unwrap_or(2),
            profile_changes: changes,
            profile_command_revision: rvn.unwrap_or(1) + 1,
            server_time: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            response_version: 1
        }
    }
}

impl FullProfile {
    pub fn new(user: User, profile_id: &str) -> Self {
        Self {
            change_type: String::from("fullProfileUpdate"),
            profile: FullProfileInner {
                _id: user.id.to_simple().to_string(),
                created: user.creation_date.to_chrono()
                    .to_rfc3339_opts(SecondsFormat::Secs, true),
                updated: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
                rvn: 1,
                wipe_number: 1,
                account_id: user.id.to_simple().to_string(),
                profile_id: String::from(profile_id),
                version: String::from("era-backend"),
                items: collections::HashMap::new(),
                stats: Stats {
                    attributes: StatsAttributes::Other(json!({}))
                },
                command_revision: 1
            }
        }
    }
}