use serde::{Deserialize, Serialize};
use serde_json::Value;

mod subscription;
mod ping;
mod chat;
mod user_presence;
mod stream_event;

#[derive(Debug)]
pub(crate) enum ServerMessage {
    StreamEvent(stream_event::StreamEvent),
    Ping(ping::Ping),
    Chat(chat::ChatMessageWrapper),
    Subscribe(subscription::SubscriptionResponse),
    UserPresence(user_presence::UserPresenceWrapper),
}

impl ServerMessage {
    pub(crate) fn from_str(msg: String) -> Option<Self> {
        // convert to untyped json
        let mut message: Value = serde_json::from_str(&msg).unwrap();
        if !message.is_object() {
            return None;
        }

        let msg_obj = message.as_object_mut()?;
        let m_type = msg_obj.get("type");
        if m_type.is_some() {
            let m_type = m_type?.as_str()?;
            return match m_type {
                // protocol messages
                "ping" => {
                    Some(ServerMessage::Ping(serde_json::from_value(message).unwrap()))
                }
                "reject_subscription" => {
                    Some(ServerMessage::Subscribe(serde_json::from_value(message).unwrap()))
                }
                "confirm_subscription" => {
                    Some(ServerMessage::Subscribe(serde_json::from_value(message).unwrap()))
                }
                &_ => {
                    println!("Unknown message type: {}", m_type);
                    None
                }
            }
        } else {
            // api messages
            let t_payload = msg_obj.get("message");
            if let Some(payload) = t_payload {
                if let Some(payload) = payload.as_object() {
                    let event_type = payload.get("event")?.as_str()?;
                    return match event_type {
                        "ChatMessage" => {
                            Some(ServerMessage::Chat(serde_json::from_value(message).unwrap()))
                        }
                        "UserPresence" => {
                            Some(ServerMessage::UserPresence(serde_json::from_value(message).unwrap()))
                        }
                        "StreamEvent" => {
                            Some(ServerMessage::StreamEvent(serde_json::from_value(message).unwrap()))
                        }
                        _ => {
                            println!("Unknown message type: {}", event_type);
                            None
                        }
                    }
                }
            }
        }
        println!("Could not parse message: {}", msg.to_string());
        None
    }
}


