use serde::{Deserialize, Serialize};
use crate::client::model::events;
use crate::client::model::events::Event;
use crate::client::model::events::stream_start::StreamStart;

#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub(crate) enum StreamEvent {
    StreamStarted {
        id: String,
        event: String,
        text: String,
        #[serde(rename = "createdAt")]
        created_at: String,
        #[serde(rename = "channelId")]
        channel_id: String,
    },
    Tipped {
        id: String,
        event: String,
        text: String,
        metadata: String,
        #[serde(rename = "createdAt")]
        created_at: String,
        #[serde(rename = "channelId")]
        channel_id: String,
    },
    WheelSpinClaimed {
        id: String,
        event: String,
        text: String,
        metadata: String,
        #[serde(rename = "createdAt")]
        created_at: String,
        #[serde(rename = "channelId")]
        channel_id: String,
    },
    Followed {
        id: String,
        event: String,
        text: String,
        metadata: String,
        #[serde(rename = "createdAt")]
        created_at: String,
        #[serde(rename = "channelId")]
        channel_id: String,
    },
    DeviceConnected {
        id: String,
        event: String,
        text: String,
        metadata: String,
        #[serde(rename = "createdAt")]
        created_at: String,
        #[serde(rename = "channelId")]
        channel_id: String,
    },
}

impl Into<Option<Event>> for StreamEvent {
    fn into(self) -> Option<Event> {
        match self {
            StreamEvent::StreamStarted {channel_id, ..} => {
                Some(Event::StreamStart(StreamStart {
                    channel_id,
                }))
            }
            StreamEvent::Tipped { channel_id, metadata, .. } => {
                if let Ok(meta) = serde_json::from_str::<events::tip::TipMeta>(&metadata) {
                    Some(Event::Tipped(events::tip::Tipped {
                        tipper: meta.who,
                        amount: meta.how_much,
                        redeem: meta.tip_menu_item,
                        channel_id,
                    }))
                } else {
                    None
                }
            }
            StreamEvent::WheelSpinClaimed { channel_id, metadata, .. } => {
                if let Ok(meta) = serde_json::from_str::<events::wheelspin::WheelspinMeta>(&metadata) {
                    Some(Event::Wheelspin(events::wheelspin::Wheelspin {
                        tipper: meta.who,
                        amount: meta.how_much,
                        redeem: meta.prize,
                        channel_id,
                    }))
                } else {
                    None
                }
            }
            StreamEvent::Followed { .. } => {
                todo!()
            }
            StreamEvent::DeviceConnected { .. } => {
                todo!()
            }
        }
    }
}


