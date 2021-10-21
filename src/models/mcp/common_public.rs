use crate::models::{db, mcp};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Attributes {
    pub banner_color: String,
    pub homebase_name: String,
    pub banner_icon: String,
}

impl mcp::FullProfile {
    pub fn new_common_public(user: db::user::User) -> Self {
        let mut full_profile = Self::new(user, "common_public");

        full_profile.profile.stats.attributes = mcp::StatsAttributes::CommonPublic(Attributes {
            banner_color: String::from(""),
            banner_icon: String::from(""),
            homebase_name: String::from("Project Era"),
        });
        
        full_profile
    }
}