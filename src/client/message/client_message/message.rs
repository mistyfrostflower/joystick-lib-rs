use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub command: String,
    pub identifier: String,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendMessage {
    action: String,
    text: String,
    channel_id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendWhisper {
    action: String,
    username: String,
    text: String,
    channel_id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteMessage {
    action: String,
    message_id: String,
    channel_id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MuteUser {
    action: String,
    message_id: String,
    channel_id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnmuteUser {
    action: String,
    username: String,
    channel_id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockUser {
    action: String,
    message_id: String,
    channel_id: String
}
