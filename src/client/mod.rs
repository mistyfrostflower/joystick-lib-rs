use std::str::FromStr;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use base64::Engine;
use base64::prelude::BASE64_URL_SAFE;
use chrono::Utc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::{Message as WsMessage, Message};
use tokio_tungstenite::{connect_async_tls_with_config, MaybeTlsStream, WebSocketStream};
use tracing::{debug, info};
use tungstenite::ClientRequestBuilder;
use tungstenite::http::Uri;

mod message;
pub mod model;

use message::server_message::ServerMessage;
use message::client_message::subscribe::Subscribe;
use message::client_message::ClientMessage;
use model::events::{Event, Intent};

pub type TIMEZONE = Utc;

pub struct Client {
    pub(self) id: String,
    pub(self) token: String,
    ws_write: Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, WsMessage>>,
    ws_read: Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
    intents: Vec<Intent>
}

impl Client {
    pub async fn connect(id: String, secret: String, intents: Vec<Intent> ) -> Arc<Self> {
        let token = BASE64_URL_SAFE.encode(id.clone() + ":" + &secret);

        let uri = Uri::from_str(&(String::from("wss://joystick.tv/cable?token=") + token.as_str())).unwrap();
        let req = ClientRequestBuilder::new(uri).with_sub_protocol("actioncable-v1-json");

        let (ws_stream, _resp) = connect_async_tls_with_config(req, None, false, None).await.unwrap();

        let (write, read) = ws_stream.split();

        let ws_read = Mutex::new(read);
        let ws_write = Mutex::new(write);

        let client = Arc::new(Self {
            id,
            token,
            ws_write,
            ws_read,
            intents,
        });

        client.raw_send(ClientMessage::Subscribe(Subscribe::new())).await;

        client
    }

    pub(crate) async fn raw_send(self: &Arc<Self>, msg: ClientMessage) {
        let mut ws_write = self.ws_write.lock().await;

        let serialized = msg.to_string();

        ws_write.send(WsMessage::text(serialized)).await.unwrap();
    }

    async fn _next_msg(self: &Arc<Self>) -> Option<ServerMessage> {
        let next: WsMessage;
        {
            let mut ws_read = self.ws_read.lock().await;
            next = ws_read.next().await.unwrap().unwrap();
        }

        match next {
            Message::Text(msg) => {
                //println!("string msg: {}", msg);
                let try_parsed = ServerMessage::from_str(msg.clone());
                if let Some(parsed) = try_parsed {
                    Some(parsed)
                } else {
                    debug!("could not parse message: {}", msg);
                    Some(ServerMessage::UnknownMessage(msg))
                }
            }
            Message::Close(_) => {
                info!("websocket connection closed by server");
                None
            }
            _ => {
                debug!("other websocket msg");
                None
            }
        }
    }

    async fn _next_server_msg(self: &Arc<Self>) -> Option<ServerMessage> {
        if let Some(msg) = self._next_msg().await {
            //println!("next server message: {:?}", msg);
            return Some(msg);
        }
        //println!("next server message: None");
        None
    }

    pub async fn try_next_event(self: &Arc<Self>) -> Option<Event> {
        let server_message = self._next_server_msg().await;

        //println!("server message: {:?}",server_message);

        let event = server_message?.to_event()?;
        
        if event.included_by_intents(&self.intents) {
            Some(event)
        } else {
            None
        }
        
    }

}

