use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Streamer {
    pub slug: Option<String>,
    pub username: Option<String>,
    #[serde(rename = "usernameColor")]
    pub username_color: Option<String>,
    #[serde(rename = "signedPhotoUrl")]
    pub signed_photo_url: Option<String>,
    #[serde(rename = "signedPhotoThumbUrl")]
    pub signed_photo_thumb_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Author {
    pub slug: Option<String>,
    pub username: Option<String>,
    #[serde(rename = "usernameColor")]
    pub username_color: Option<String>,
    #[serde(rename = "displayNameWithFlair")]
    pub display_name_with_flair: Option<String>,
    #[serde(rename = "signedPhotoUrl")]
    pub signed_photo_url: Option<String>,
    #[serde(rename = "signedPhotoThumbUrl")]
    pub signed_photo_thumb_url: Option<String>,
    #[serde(rename = "isStreamer")]
    pub is_streamer: Option<bool>,
    #[serde(rename = "isModerator")]
    pub is_moderator: Option<bool>,
    #[serde(rename = "isSubscriber")]
    pub is_subscriber: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatMessage {
    pub event: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "messageId")]
    pub message_id: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub visibility: Option<String>,
    pub text: Option<String>,
    #[serde(rename = "botCommand")]
    pub bot_command: Option<String>,
    #[serde(rename = "botCommandArg")]
    pub bot_command_arg: Option<String>,
    #[serde(rename = "emotesUsed")]
    pub emotes_used: Option<Vec<String>>,
    pub author: Option<Author>,
    pub streamer: Option<Streamer>,
    #[serde(rename = "channelId")]
    pub channel_id: Option<String>,
    pub mention: Option<bool>,
    #[serde(rename = "mentionedUsername")]
    pub mentioned_username: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ChatMessageWrapper {
    pub identifier: String,
    pub message: ChatMessage,
}