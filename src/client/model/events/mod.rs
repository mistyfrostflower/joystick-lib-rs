pub mod presence;
pub mod ping;
pub mod chat;
pub mod stream_start;
pub mod tip;
pub mod wheelspin;

use chat::ChatMessage;
use presence::{UserJoin, UserLeave};
use ping::Ping;
use crate::client::model::events::stream_start::StreamStart;
use crate::client::model::events::tip::Tipped;
use crate::client::model::events::wheelspin::Wheelspin;

#[derive(Debug, PartialEq)]
pub enum Intent {
    UserJoin,
    UserLeave,
    Chat,
    Ping,
    Connected,
    StreamStart,
    Tipped,
    Wheelspin,
}

#[derive(Debug)]
pub enum Event {
    UserJoin(UserJoin),
    UserLeave(UserLeave),
    Chat(ChatMessage),
    Ping(Ping),
    Connected,
    StreamStart(StreamStart),
    Tipped(Tipped),
    Wheelspin(Wheelspin)
}

impl Event {
    pub(crate) fn included_by_intents(&self, intents: &Vec<Intent>) -> bool {
        match self {
            Event::UserJoin(_) => {
                intents.contains(&Intent::UserJoin)
            }
            Event::UserLeave(_) => {
                intents.contains(&Intent::UserLeave)
            }
            Event::Chat(_) => {
                intents.contains(&Intent::Chat)
            }
            Event::Ping(_)=> {
                intents.contains(&Intent::Ping)
            },
            &Event::Connected => {
                intents.contains(&Intent::Connected)
            },
            &Event::StreamStart(_) | &Event::Tipped(_) | &Event::Wheelspin(_) => todo!()
        }
    }
}






