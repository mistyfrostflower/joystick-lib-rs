pub mod chat;
pub mod device_status;
pub mod follow;
pub mod ping;
pub mod presence;
pub mod stream_start;
pub mod tip;
pub mod wheelspin;

use crate::client::model::events::follow::Follow;
use crate::client::model::events::stream_start::StreamStart;
use crate::client::model::events::tip::Tipped;
use crate::client::model::events::wheelspin::Wheelspin;
use chat::ChatMessage;
use ping::Ping;
use presence::{UserJoin, UserLeave};

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum Intent {
    UserJoin,
    UserLeave,
    Chat,
    Ping,
    Connected,
    StreamStart,
    Tipped,
    Wheelspin,
    Follow,
    DeviceStatus,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Event {
    UserJoin(UserJoin),
    UserLeave(UserLeave),
    Chat(ChatMessage),
    Ping(Ping),
    Connected,
    StreamStart(StreamStart),
    Tipped(Tipped),
    Wheelspin(Wheelspin),
    Follow(Follow),
    DeviceStatus(device_status::DeviceStatus),
}

impl Event {
    pub(crate) fn included_by_intents(&self, intents: &Vec<Intent>) -> bool {
        match self {
            Event::UserJoin(_) => intents.contains(&Intent::UserJoin),
            Event::UserLeave(_) => intents.contains(&Intent::UserLeave),
            Event::Chat(_) => intents.contains(&Intent::Chat),
            Event::Ping(_) => intents.contains(&Intent::Ping),
            Event::Connected => intents.contains(&Intent::Connected),
            Event::StreamStart(_) => intents.contains(&Intent::StreamStart),
            Event::Tipped(_) => intents.contains(&Intent::Tipped),
            Event::Wheelspin(_) => intents.contains(&Intent::Wheelspin),
            Event::Follow(_) => intents.contains(&Intent::Follow),
            Event::DeviceStatus(_) => intents.contains(&Intent::DeviceStatus),
        }
    }
}
