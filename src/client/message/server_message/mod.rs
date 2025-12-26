
use chrono::{DateTime, FixedOffset};
use serde_json::Value;
use tracing::{error, trace, warn};
use crate::client::model::events::Event;
use crate::client::model::events::Event::Connected;

pub(crate) mod subscription;
pub(crate) mod ping;
pub(crate) mod chat;
pub(crate) mod user_presence;
pub(crate) mod stream_event;

pub(crate) fn timestamp_to_unix(time_str: String) -> Option<i64> {

    let time = {
        if time_str.ends_with("Z") {
            time_str.clone()
        } else {
            time_str.clone() + "Z"
        }
    };

    let utc_time = time.parse::<DateTime<FixedOffset>>();
    match utc_time {
        Ok(time) => { Some(time.timestamp_millis()) }
        Err(err) => {
            warn!("could not convert timestamp: {} - {}", time_str, err);
            None
        }
    }
}

#[derive(Debug)]
pub(crate) enum ServerMessage {
    StreamEvent(stream_event::StreamEvent),
    Ping(ping::Ping),
    Chat(chat::ChatMessageWrapper),
    Subscribe(subscription::SubscriptionResponse),
    UserPresence(user_presence::UserPresenceWrapper),
    UnknownMessage(String),
    Welcome,
}

impl ServerMessage {
    pub(crate) fn from_str(msg: String) -> Option<Self> {
        //trace!("parsing string into server message");
        
        // convert to untyped JSON
        let mut message: Value = serde_json::from_str(&msg).unwrap();
        if !message.is_object() {
            error!("received empty message / invalid json from joystick server, this is very odd");
            return None;
        }

        let msg_obj = message.as_object_mut()?;
        let m_type = msg_obj.get("type");
        if m_type.is_some() {
            // has type field, is a protocol message
            let m_type = { 
                let try_type = m_type?.as_str();
                if try_type.is_none() {
                    warn!("message has a type field but the value is not a string? this is very odd and does not follow the actionable spec");
                }
                try_type?
            };
            
            return match m_type {
                // protocol messages
                "ping" => {
                    //trace!("server message is ping");
                    Some(ServerMessage::Ping(serde_json::from_value(message).unwrap()))
                }
                "reject_subscription" => {
                    trace!("server message is reject subscription");
                    Some(ServerMessage::Subscribe(serde_json::from_value(message).unwrap()))
                }
                "confirm_subscription" => {
                    trace!("server message is confirm subscription");
                    Some(ServerMessage::Subscribe(serde_json::from_value(message).unwrap()))
                }
                "welcome" => {
                    Some(ServerMessage::Welcome)
                }
                &_ => {
                    warn!("Unknown protocol message type: {}", m_type);
                    None
                }
            };
        } else {
            // does not have a type field is an api message
            let t_payload = msg_obj.get("message");
            if let Some(payload) = t_payload {
                if let Some(payload) = payload.as_object() {
                    let event_type = payload.get("event")?.as_str()?;
                    
                    return match event_type {
                        "ChatMessage" => {
                            trace!("server message is chat message");
                            Some(ServerMessage::Chat(serde_json::from_value(message).unwrap()))
                        }
                        "UserPresence" => {
                            trace!("server message is user presence");
                            Some(ServerMessage::UserPresence(serde_json::from_value(message).unwrap()))
                        }
                        "StreamEvent" => {
                            trace!("server message is stream event");
                            Some(ServerMessage::StreamEvent(serde_json::from_value(message).unwrap()))
                        }
                        _ => {
                            warn!("Unknown message type: {}", event_type);
                            None
                        }
                    };
                }
            }
        }
        warn!("Could not parse message: {}", msg.to_string());
        None
    }
}

impl From<ServerMessage> for Option<Event> {
    fn from(value: ServerMessage) -> Option<Event> {
        match value {
            ServerMessage::StreamEvent(stream_event) => {
                trace!("converting server message into stream event");
                stream_event.into()
            }
            ServerMessage::Ping(ping) => {
                //trace!("converting server message into ping event");
                ping.into()
            }
            ServerMessage::Chat(chat_msg) => {
                trace!("converting server message into chat event");
                chat_msg.into()
            }
            ServerMessage::Subscribe(_cable_sub) => {
                trace!("converting server message connected event");
                Some(Connected)
            }
            ServerMessage::UserPresence(user_presence) => {
                trace!("converting server message into user presence event");
                user_presence.into()
            }
            ServerMessage::UnknownMessage(str) => {
                trace!("converting unknown server message into event");
                Some(Event::UnknownEvent(str))
            }
            ServerMessage::Welcome => {Some(Event::Welcome)}
        }
    }
}
