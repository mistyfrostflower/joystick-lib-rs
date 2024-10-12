use serde::{Deserialize, Serialize};

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

