use serde::{Deserialize, Serialize};
use crate::client::Event;
use crate::client::model::events::presence::{UserJoin, UserLeave};
use crate::client::message::server_message::timestamp_to_unix;

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
    identifier: String,
    message: UserPresence,
}

impl Into<Option<Event>> for UserPresenceWrapper {
    fn into(self) -> Option<Event> {
        if self.message.r#type == "enter_stream" {
            Some(Event::UserJoin(UserJoin {
                channel_id: self.message.channel_id,
                username: self.message.text,
                event_id: self.message.id,
                event_time: timestamp_to_unix(self.message.created_at),
            }))
        } else if self.message.r#type == "leave_stream" {
            Some(Event::UserLeave(UserLeave {
                channel_id: self.message.channel_id,
                username: self.message.text,
                event_id: self.message.id,
                event_time: timestamp_to_unix(self.message.created_at),
            }))
        } else {
            None
        }
    }
}
