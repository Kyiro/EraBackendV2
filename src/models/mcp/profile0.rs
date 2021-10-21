use crate::models::{db, mcp};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize)]
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

impl mcp::FullProfile {
    pub fn new_profile0(user: db::user::User) -> Self {
        let mut full_profile = Self::new(user, "profile0");
        
        full_profile.profile.stats.attributes = mcp::StatsAttributes::Profile0(Attributes {
            node_costs: json!({}),
            mission_alert_redemption_record: json!({}),
            twitch: json!({}),
            client_settings: json!({}),
            level: 0,
            named_counters: json!({
                "SubGameSelectCount_Campaign": {
                    "current_count": 0
                },
                "SubGameSelectCount_Athena": {
                    "current_count": 0
                }
            }),
            default_hero_squad_id: String::new(),
            collection_book: json!({}),
            quest_manager: json!({
                "dailyLoginInterval": "2017-01-01T01:00:00.602Z",
                "dailyQuestRerolls": 1
            }),
            bans: json!({}),
            gameplay_stats: Vec::new(),
            inventory_limit_bonus: 0,
            current_mtx_platform: String::from("Epic"),
            weekly_purchases: json!({}),
            daily_purchases: json!({}),
            mode_loadouts: Vec::new(),
            in_app_purchases: json!({}),
            daily_rewards: json!({}),
            monthly_purchases: json!({}),
            xp: 0,
            homebase: json!({
                "townName": "ProjectEra",
                "bannerIconId": "",
                "bannerColorId": "",
                "flagPattern": -1,
                "flagColor": -1
            }),
            packs_granted: 0
        });
            
        full_profile
    }
}