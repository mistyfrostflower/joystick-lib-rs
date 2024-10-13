use serde::{Deserialize, Serialize};
use crate::client::model::{events,emote};
use crate::client::model::events::Event;

#[derive(Serialize, Deserialize, Debug)]
struct Emote {
    pub code: String,
    #[serde(rename = "signedUrl")]
    pub signed_url: String,
    #[serde(rename = "signedThumbnailUrl")]
    pub signed_thumbnail_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Streamer {
    pub slug: String,
    pub username: String,
    #[serde(rename = "usernameColor")]
    pub username_color: Option<String>,
    #[serde(rename = "signedPhotoUrl")]
    pub signed_photo_url: String,
    #[serde(rename = "signedPhotoThumbUrl")]
    pub signed_photo_thumb_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Author {
    pub slug: String,
    pub username: String,
    #[serde(rename = "usernameColor")]
    pub username_color: Option<String>,
    #[serde(rename = "displayNameWithFlair")]
    pub display_name_with_flair: String,
    #[serde(rename = "signedPhotoUrl")]
    pub signed_photo_url: String,
    #[serde(rename = "signedPhotoThumbUrl")]
    pub signed_photo_thumb_url: String,
    #[serde(rename = "isStreamer")]
    pub is_streamer: bool,
    #[serde(rename = "isModerator")]
    pub is_moderator: bool,
    #[serde(rename = "isSubscriber")]
    pub is_subscriber: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatMessage {
    pub event: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "messageId")]
    pub message_id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    pub visibility: String,
    pub text: String,
    #[serde(rename = "botCommand")]
    pub bot_command: Option<String>,
    #[serde(rename = "botCommandArg")]
    pub bot_command_arg: Option<String>,
    #[serde(rename = "emotesUsed")]
    pub emotes_used: Vec<Emote>,
    pub author: Author,
    pub streamer: Streamer,
    #[serde(rename = "channelId")]
    pub channel_id: String,
    pub mention: bool,
    #[serde(rename = "mentionedUsername")]
    pub mentioned_username: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ChatMessageWrapper {
    pub identifier: String,
    message: ChatMessage,
}

impl Into<Option<Event>> for ChatMessageWrapper {
    fn into(self) -> Option<Event> {
        //println!("parsing chat message into event");
        let mut emotes:Vec<emote::Emote> = Vec::with_capacity(self.message.emotes_used.len());

        for emote in self.message.emotes_used {
            let e = emote::Emote {
                code: emote.code,
                url: emote.signed_url,
                thumb_url: emote.signed_thumbnail_url,
            };
            emotes.push(e)
        }

        let e = events::chat::ChatMessage {
            author: self.message.author.slug,
            created_at: self.message.created_at,
            message_id: self.message.message_id,
            visibility: self.message.visibility,
            text: self.message.text,
            bot_command: self.message.bot_command,
            bot_command_arg: self.message.bot_command_arg,
            emotes_used: emotes,
            channel_id: self.message.channel_id,
            mention: self.message.mention,
            mentioned_username: self.message.mentioned_username,
        };

        Some(Event::Chat(e))
    }
}

