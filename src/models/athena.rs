use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
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
    // variants saving soon(tm)
    pub character: String,
    pub backpack: String,
    pub pickaxe: String,
    pub glider: String,
    pub contrail: String,
    pub music: String,
    pub loading: String,
    pub dance: [String; 6],
    pub wrap: [String; 7]
}

impl Default for Locker {
    fn default() -> Self {
        Self {
            character: String::from("AthenaCharacter:cid_005_athena_commando_m_default"),
            backpack: String::new(),
            pickaxe: String::from("AthenaPickaxe:defaultpickaxe"),
            glider: String::from("AthenaGlider:defaultglider"),
            contrail: String::from(""),
            music: String::new(),
            loading: String::new(),
            dance: [
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
            ],
            wrap: [
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