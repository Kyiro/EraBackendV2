use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Serialize, Deserialize)]
pub struct Attributes {
    pub past_seasons: Vec<Value>,
    pub season_match_boost: i32,
    pub mfa_reward_claimed: bool,
    pub rested_xp_overflow: i32,
    pub quest_manager: Value,
    pub book_level: i32,
    pub season_num: usize,
    pub book_xp: i32,
    pub permissions: Vec<Value>,
    pub season: Value,
    pub battlestars: i32,
    pub vote_data: Value,
    pub book_purchased: bool,
    pub lifetime_wins: i32,
    pub party_assist_quest: String,
    pub purchased_battle_pass_tier_offers: Value,
    pub rested_xp_exchange: i32,
    pub level: i32,
    pub xp_overflow: i32,
    pub rested_xp: i32,
    pub rested_xp_mult: f32,
    #[serde(rename = "accountLevel")]
    pub account_level: i32,
    pub competitive_identity: Value,
    pub inventory_limit_bonus: i32,
    pub daily_rewards: Value,
    pub xp: i32,
    pub season_friend_match_boost: i32,
    // cosmetics
    pub favorite_character: String,
    pub favorite_backpack: String,
    pub favorite_pickaxe: String,
    pub favorite_glider: String,
    pub favorite_skydivecontrail: String,
    pub favorite_musicpack: String,
    pub favorite_loadingscreen: String,
    pub favorite_dance: [String; 6],
    pub favorite_itemwraps: [String; 7],
    // unused cosmetics
    pub favorite_callingcard: String,
    pub favorite_consumableemote: String,
    pub favorite_spray: Vec<String>,
    pub favorite_hat: String,
    pub favorite_battlebus: String,
    pub favorite_mapmarker: String,
    pub favorite_vehicledeco: String,
    pub favorite_victorypose: String,
}

#[derive(Serialize, Deserialize)]
pub struct Cosmetic {
    #[serde(rename = "templateId")]
    pub template_id: String,
    pub attributes: CosmeticAttributes,
    pub quantity: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CosmeticAttributes {
    pub max_level_bonus: i32,
    pub level: i32,
    pub item_seen: bool,
    pub xp: i32,
    pub variants: Vec<Variant>,
    #[serde(rename = "favorite")]
    pub favourite: bool,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Variant {
    pub channel: String,
    pub active: String,
    pub owned: Vec<String>,
}