use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Serialize, Deserialize)]
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