use serde::{Deserialize, Serialize};
use crate::client::message::GATEWAY_CHANNEL;

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscribe {
    command: String,
    identifier: String,
}

impl Subscribe {
    pub fn new() -> Self {
        Self {
            command: String::from("subscribe"),
            identifier: String::from(GATEWAY_CHANNEL),
        }
    }
}

