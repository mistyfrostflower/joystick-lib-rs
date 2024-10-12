use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct UserPresence {
    pub id: String,
    pub event: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub text: String,
    #[serde(rename = "channelId")]
    pub channel_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct UserPresenceWrapper {
    pub identifier: String,
    pub message: UserPresence,
}