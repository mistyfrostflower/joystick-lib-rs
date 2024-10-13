use serde::{Deserialize, Serialize};
use crate::client::message::server_message::stream_event::StreamEvent;

#[derive(Serialize,Deserialize,Debug)]
pub struct StreamStart {
    pub channel_id: String,
}



