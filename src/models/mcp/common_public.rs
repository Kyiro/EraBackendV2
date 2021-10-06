use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Attributes {
    pub banner_color: String,
    pub homebase_name: String,
    pub banner_icon: String,
}