use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SubscriptionResponse {
    #[serde(rename = "type")]
    r#type: String,
    identifier: String,
}
