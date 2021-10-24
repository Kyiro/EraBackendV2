use crate::models::{db, files, mcp};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EquipBattleRoyaleCustomization {
    pub item_to_slot: String,
    pub slot_name: String,
    #[serde(rename = "indexWithinSlot")]
    pub index: Option<i32>,
    #[serde(rename = "variantUpdates")]
    pub variants: Option<Vec<Variant>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetItemFavoriteStatusBatch {
    pub item_fav_status: Vec<bool>,
    pub item_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Variant {
    pub channel: String,
    pub active: String,
    pub owned: Vec<String>,
}

impl Variant {
    pub fn new(updates: Vec<Self>, cvariants: Vec<files::Variant>) -> Vec<Self> {
        let mut variants: Vec<Variant> = Vec::new();
        
        for v in cvariants {
            if &v.channel == "JerseyColor" {
                continue;
            }
            
            variants.push(Variant {
                channel: v.channel.clone(),
                // could be better but works :/
                active: match updates.iter().find(|u| u.channel == v.channel) {
                    Some(data) => data.active.clone(),
                    None => v.options.get(0).unwrap().clone(),
                },
                owned: v.options,
            });
        }
        
        for update in updates.into_iter() {
            if let None = variants.iter().find(|v| v.channel == update.channel) {
                variants.push(update);
            }
        }
        
        variants
    }
}

impl mcp::FullProfile {
    pub fn new_athena(
        athena: db::athena::Profile,
        cosmetics: Vec<files::Item>,
        season: usize,
        user: db::user::User
    ) -> Self {
        let mut full_profile = Self::new(user, "athena");
        
        full_profile.profile.stats.attributes = mcp::StatsAttributes::Athena(Attributes {
            past_seasons: Vec::new(),
            season_match_boost: 120,
            mfa_reward_claimed: true,
            rested_xp_overflow: 0,
            quest_manager: json!({
                "dailyLoginInterval": "2021-06-24T11:24:14.414Z",
                "dailyQuestRerolls": 1
            }),
            book_level: 100,
            season_num: season,
            book_xp: 999999,
            permissions: Vec::new(),
            season: json!({
                "numWins": 0,
                "numHighBracket": 0,
                "numLowBracket": 0
            }),
            battlestars: 9999,
            vote_data: json!({}),
            book_purchased: true,
            lifetime_wins: 999,
            party_assist_quest: String::new(),
            purchased_battle_pass_tier_offers: json!({}),
            rested_xp_exchange: 1,
            level: 100,
            xp_overflow: 0,
            rested_xp: 0,
            rested_xp_mult: 4.55,
            account_level: 9999,
            competitive_identity: json!({}),
            inventory_limit_bonus: 0,
            daily_rewards: json!({}),
            xp: 9999999,
            season_friend_match_boost: 40,
            // cosmetics
            favorite_character: athena.locker.character.clone(),
            favorite_backpack: athena.locker.backpack.clone(),
            favorite_pickaxe: athena.locker.pickaxe.clone(),
            favorite_glider: athena.locker.glider.clone(),
            favorite_skydivecontrail: athena.locker.skydivecontrail.clone(),
            favorite_musicpack: athena.locker.musicpack.clone(),
            favorite_loadingscreen: athena.locker.loadingscreen.clone(),
            favorite_dance: athena.locker.dance.clone(),
            favorite_itemwraps: athena.locker.itemwrap.clone(),
            // unused cosmetics
            favorite_callingcard: String::new(),
            favorite_consumableemote: String::new(),
            favorite_spray: Vec::new(),
            favorite_hat: String::new(),
            favorite_battlebus: String::new(),
            favorite_mapmarker: String::new(),
            favorite_vehicledeco: String::new(),
            favorite_victorypose: String::new(),
        });
        
        for i in cosmetics {
            let template = format!("{}:{}", i.item_type, i.id);
            let variants = {
                // maybe clean this up some day?
                let mut variants = Vec::<Variant>::new();
                
                if let Some(item) = athena.locker.get(&i.item_type) {
                    if item == &template {
                        if let Some(item_variants) = athena.locker.get_variants(&i.item_type) {
                            variants = item_variants.clone();
                        }
                    }
                }
                
                variants
            };
            
            full_profile.profile.items.insert(
                template.clone(),
                mcp::Item::Cosmetic(Cosmetic {
                    template_id: template.clone(),
                    attributes: CosmeticAttributes {
                        max_level_bonus: 0,
                        level: 1,
                        item_seen: true,
                        xp: 0,
                        variants: Variant::new(variants, i.variants),
                        favourite: athena.favourites.contains(&template)
                    },
                    quantity: 1
                })
            );
        }
        
        full_profile
    }
}