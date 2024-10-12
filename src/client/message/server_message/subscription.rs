use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug)]
pub struct SubscriptionResponse {
    #[serde(rename = "type")]
    r#type: String,
    identifier: String,
}
