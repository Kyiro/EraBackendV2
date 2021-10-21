use chrono::prelude::*;
use crate::models::{db, mcp};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize)]
pub struct Attributes {
    pub survey_data: Value,
    pub personal_offers: Value,
    pub intro_game_played: bool,
    pub import_friends_claimed: Value,
    pub mtx_purchase_history: Value,
    pub undo_cooldowns: Vec<Value>,
    pub mtx_affiliate_set_time: String,
    pub inventory_limit_bonus: i32,
    pub current_mtx_platform: String,
    pub mtx_affiliate: String,
    pub weekly_purchases: Value,
    pub daily_purchases: Value,
    pub ban_history: Value,
    pub in_app_purchases: Value,
    pub permissions: Vec<Value>,
    pub undo_timeout: String,
    pub monthly_purchases: Value,
    pub allowed_to_send_gifts: bool,
    pub mfa_enabled: bool,
    pub allowed_to_receive_gifts: bool,
    pub gift_history: Value,
}

impl mcp::FullProfile {
    pub fn new_common_core(user: db::user::User) -> Self {
        let mut full_profile = Self::new(user, "common_core");
        
        full_profile.profile.items.insert(
            String::from("Currency:MtxComplimentary"),
            mcp::Item::Other(json!({
                "templateId": "Currency:MtxComplimentary",
                "attributes": {
                    "platform": "Shared"
                },
                "quantity": 13500
            }))
        );

        full_profile.profile.stats.attributes = mcp::StatsAttributes::CommonCore(Attributes {
            survey_data: json!({}),
            personal_offers: json!({}),
            intro_game_played: false,
            import_friends_claimed: json!({}),
            mtx_purchase_history: json!({}),
            undo_cooldowns: Vec::new(),
            mtx_affiliate_set_time: Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            inventory_limit_bonus: 0,
            current_mtx_platform: String::from("EpicPC"),
            mtx_affiliate: String::from(""),
            weekly_purchases: json!({}),
            daily_purchases: json!({}),
            ban_history: json!({}),
            in_app_purchases: json!({}),
            permissions: Vec::new(),
            undo_timeout: String::from("min"),
            monthly_purchases: json!({}),
            allowed_to_send_gifts: true,
            mfa_enabled: false,
            allowed_to_receive_gifts: true,
            gift_history: json!({}),
        });
        
        full_profile
    }
}