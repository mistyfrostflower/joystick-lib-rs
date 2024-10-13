use serde::{Deserialize, Serialize};
use crate::client::message::server_message::user_presence::UserPresenceWrapper;
use crate::client::model::events::Event;
use crate::client::model;

#[derive(Serialize, Deserialize, Debug)]
pub struct Ping {
    #[serde(rename = "type")]
    pub r#type: String,
    pub message: i64,
}

impl Into<Option<Event>> for Ping {
    fn into(self) -> Option<Event> {
        Some(Event::Ping(model::events::ping::Ping { timestamp: self.message }))
    }
}

