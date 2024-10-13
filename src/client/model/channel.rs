use serde::{Deserialize, Serialize};

pub type ChannelId = String;

#[derive(Serialize,Deserialize,Debug)]
pub struct Channel {
    id: ChannelId,
    username: Option<String>,
    slug: Option<String>,
    username_color: Option<String>,
    photo_url: Option<String>,
    thumb_url: Option<String>,
}
