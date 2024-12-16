use crate::client::model::emote::Emote;

#[derive(Debug)]
pub struct MessageAuthor {
    pub slug: String,
    pub username: String,
    pub username_color: Option<String>,
    pub display_name_with_flair: String,
    pub signed_photo_url: String,
    pub signed_photo_thumb_url: String,
    pub is_streamer: bool,
    pub is_moderator: bool,
    pub is_subscriber: bool,
}

#[derive(Debug)]
pub struct ChatMessage {
    pub author: String,
    pub created_at: String,
    pub message_id: String,
    pub visibility: String,
    pub text: String,
    pub bot_command: Option<String>,
    pub bot_command_arg: Option<String>,
    pub emotes_used: Vec<Emote>,
    pub channel_id: String,
    pub mention: bool,
    pub mentioned_username: Option<String>,
}


