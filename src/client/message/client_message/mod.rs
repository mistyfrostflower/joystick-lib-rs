pub(crate) mod subscribe;
pub(crate) mod message;

use serde::{Deserialize, Serialize};
use subscribe::Subscribe;
use message::Message;

pub(crate) enum ClientMessage {
    /// subscribes to the joystick api
    Subscribe(Subscribe),
    /// api message
    Message(Message)
}

impl ClientMessage {
    pub fn to_string(self) -> String {
        match self {
            ClientMessage::Subscribe(sub) => {
                serde_json::to_string(&sub).unwrap()
            }
            ClientMessage::Message(msg) => {
                serde_json::to_string(&msg).unwrap()
            }
        }
    }
}





