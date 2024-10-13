
use chrono::{DateTime, FixedOffset};
use serde_json::Value;
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
            println!("could not convert timestamp: {} - {}", time_str, err);
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
}

impl ServerMessage {
    pub(crate) fn from_str(msg: String) -> Option<Self> {

        //println!("parsing...");
        
        // convert to untyped json
        let mut message: Value = serde_json::from_str(&msg).unwrap();
        if !message.is_object() {
            println!("recieved empty message / invalid json?");
            return None;
        }

        let msg_obj = message.as_object_mut()?;
        let m_type = msg_obj.get("type");
        if m_type.is_some() {
            let m_type = { 
                let try_type = m_type?.as_str();
                if try_type.is_none() {
                    println!("message type is not string?");
                }
                try_type?
            };
            
            return match m_type {
                // protocol messages
                "ping" => {
                    //println!("got ping");
                    Some(ServerMessage::Ping(serde_json::from_value(message).unwrap()))
                }
                "reject_subscription" => {
                    //println!("got reject sub");
                    Some(ServerMessage::Subscribe(serde_json::from_value(message).unwrap()))
                }
                "confirm_subscription" => {
                    //println!("got confirm sub");
                    Some(ServerMessage::Subscribe(serde_json::from_value(message).unwrap()))
                }
                "welcome" => {
                    None
                }
                &_ => {
                    println!("Unknown message type: {}", m_type);
                    None
                }
            };
        } else {
            // api messages
            let t_payload = msg_obj.get("message");
            if let Some(payload) = t_payload {
                if let Some(payload) = payload.as_object() {
                    let event_type = payload.get("event")?.as_str()?;
                    
                    
                    
                    return match event_type {
                        "ChatMessage" => {
                           // println!("got chat message");
                            Some(ServerMessage::Chat(serde_json::from_value(message).unwrap()))
                        }
                        "UserPresence" => {
                            //println!("got user presence");
                            Some(ServerMessage::UserPresence(serde_json::from_value(message).unwrap()))
                        }
                        "StreamEvent" => {
                            //println!("got stream event");
                            Some(ServerMessage::StreamEvent(serde_json::from_value(message).unwrap()))
                        }
                        _ => {
                            println!("Unknown message type: {}", event_type);
                            None
                        }
                    };
                }
            }
        }
        println!("Could not parse message: {}", msg.to_string());
        None
    }

    pub(crate) fn to_event(self) -> Option<Event> {
        
        match self {
            ServerMessage::StreamEvent(stream_event) => {
                //println!("stream event");
                None
            }
            ServerMessage::Ping(ping) => {
                //println!("ping");
                ping.into()
            }
            ServerMessage::Chat(chat_msg) => {
                //println!("chat");
                chat_msg.into()
            }
            ServerMessage::Subscribe(cable_sub) => {
                Some(Connected)
            }
            ServerMessage::UserPresence(user_presence) => {
                //println!("user presence");
                user_presence.into()
            }
        }
    }
}


