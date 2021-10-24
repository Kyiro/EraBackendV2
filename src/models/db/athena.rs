use bson::serde_helpers::uuid_as_binary;
use crate::models::mcp::athena;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    #[serde(with = "uuid_as_binary")]
    pub id: Uuid,
    pub locker: Locker,
    pub favourites: Vec<String>
}

impl Profile {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            locker: Locker::default(),
            favourites: Vec::new()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Locker {
    pub character: String,
    #[serde(default = "Vec::new")]
    pub character_variants: Vec<athena::Variant>,
    
    pub backpack: String,
    #[serde(default = "Vec::new")]
    pub backpack_variants: Vec<athena::Variant>,
    
    pub pickaxe: String,
    #[serde(default = "Vec::new")]
    pub pickaxe_variants: Vec<athena::Variant>,
    
    pub glider: String,
    #[serde(default = "Vec::new")]
    pub glider_variants: Vec<athena::Variant>,
    
    pub skydivecontrail: String,
    pub musicpack: String,
    pub loadingscreen: String,
    pub dance: [String; 6],
    pub itemwrap: [String; 7]
}

impl Default for Locker {
    fn default() -> Self {
        Self {
            character: String::from("AthenaCharacter:cid_005_athena_commando_m_default"),
            character_variants: Vec::new(),
            
            backpack: String::new(),
            backpack_variants: Vec::new(),
            
            pickaxe: String::from("AthenaPickaxe:defaultpickaxe"),
            pickaxe_variants: Vec::new(),
            
            glider: String::from("AthenaGlider:defaultglider"),
            glider_variants: Vec::new(),
            
            skydivecontrail: String::from(""),
            musicpack: String::new(),
            loadingscreen: String::new(),
            dance: [
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
            ],
            itemwrap: [
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
            ],
        }
    }
}

impl Locker {
    // i don't need more for this rn
    pub fn get(&self, item_type: &String) -> Option<&String> {
        match item_type.as_str() {
            "AthenaCharacter" => Some(&self.character),
            "AthenaBackpack" => Some(&self.backpack),
            "AthenaPickaxe" => Some(&self.pickaxe),
            "AthenaGlider" => Some(&self.glider),
            _ => None
        }
    }
    
    pub fn get_variants(&self, item_type: &String) -> Option<&Vec<athena::Variant>> {
        match item_type.as_str() {
            "AthenaCharacter" => Some(&self.character_variants),
            "AthenaBackpack" => Some(&self.backpack_variants),
            "AthenaPickaxe" => Some(&self.pickaxe_variants),
            "AthenaGlider" => Some(&self.glider_variants),
            _ => None
        }
    }
}