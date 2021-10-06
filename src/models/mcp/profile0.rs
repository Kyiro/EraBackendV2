use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Serialize, Deserialize)]
pub struct Attributes {
    pub node_costs: Value,
    pub mission_alert_redemption_record: Value,
    pub twitch: Value,
    pub client_settings: Value,
    pub level: i32,
    pub named_counters: Value,
    pub default_hero_squad_id: String,
    pub collection_book: Value,
    pub quest_manager: Value,
    pub bans: Value,
    pub gameplay_stats: Vec<Value>,
    pub inventory_limit_bonus: i32,
    pub current_mtx_platform: String,
    pub weekly_purchases: Value,
    pub daily_purchases: Value,
    pub mode_loadouts: Vec<Value>,
    pub in_app_purchases: Value,
    pub daily_rewards: Value,
    pub monthly_purchases: Value,
    pub xp: i32,
    pub homebase: Value,
    pub packs_granted: i32
}