use serde::{Deserialize, Serialize};
use crate::client::model::channel::ChannelId;


#[derive(Serialize, Deserialize, Debug)]
pub struct UserJoin {
    pub channel_id: ChannelId,
    pub username: String,
    pub event_id: String,
    pub event_time: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLeave {
    pub channel_id: ChannelId,
    pub username: String,
    pub event_id: String,
    pub event_time: Option<i64>,
}


