use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Ping {
    #[serde(rename = "type")]
    pub r#type: String,
    pub message: i64,
}