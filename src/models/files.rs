use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;

pub struct Files {
    pub cosmetics: Vec<Item>,
    pub fortnite_game: Value,
    pub keychain: Vec<String>
}

impl Files {
    pub fn new() -> Self {
        Self {
            cosmetics: cosmetics(),
            fortnite_game: fortnite_game(),
            keychain: keychain()
        }
    }
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Item {
    #[serde(rename = "type")]
    pub item_type: String,
    pub id: String,
    pub variants: Vec<Variant>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Variant {
    pub channel: String,
    pub options: Vec<String>,
}

pub fn cosmetics() -> Vec<Item> {
    serde_json::from_str(
        &fs::read_to_string("cosmetics.json").unwrap_or(
            include_str!("../../resources/cosmetics.json").to_string()
        )
    ).unwrap_or(Vec::new())
}

pub fn fortnite_game() -> Value {
    serde_json::from_str(
        &fs::read_to_string("fortnite-game.json").unwrap_or(
            include_str!("../../resources/fortnite-game.json").to_string()
        )
    ).unwrap_or(json!({}))
}

pub fn keychain() -> Vec<String> {
    serde_json::from_str(
        &fs::read_to_string("keychain.json").unwrap_or(
            include_str!("../../resources/keychain.json").to_string()
        )
    ).unwrap_or(Vec::new())
}