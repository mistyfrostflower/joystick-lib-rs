use base64::Engine;
use base64::prelude::BASE64_URL_SAFE;
use chrono::Utc;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use std::str::FromStr;
use std::sync::Arc;
use meby::Meby::{Nope, Yes};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, RwLock};
use tokio_tungstenite::tungstenite::{ClientRequestBuilder, Message as WsMessage, Message};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async_tls_with_config};
use tracing::{debug, info, warn};

use tokio_tungstenite::tungstenite::http::Uri;

mod message;
pub mod model;

use crate::error::{JSOptionalResult, JSResult, JoystickErr};
use message::client_message::ClientMessage;
use message::client_message::subscribe::Subscribe;
use message::server_message::ServerMessage;
use model::events::{Event, Intent};

pub type TIMEZONE = Utc;

pub struct Client {
    pub(self) token: String,
    connected: RwLock<bool>,
    ws_write: Mutex<Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, WsMessage>>>,
    ws_read: Mutex<Option<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
    intents: Vec<Intent>,
}

impl Client {
    pub fn new(id: String, secret: &str, intents: Vec<Intent>) -> Arc<Self> {
        let token = BASE64_URL_SAFE.encode(format!("{id}:{secret}"));

        Arc::new(Self {
            token,
            ws_write: Mutex::new(None),
            ws_read: Mutex::new(None),
            connected: RwLock::new(false),
            intents,
        })
    }

    pub async fn disconnect(self: &Arc<Self>) {
        let mut connected = self.connected.write().await;
        if *connected {
            // already connected
            warn!("tried connecting while already connected");
            return;
        }

        *connected = false;
    }

    pub async fn connect(self: &Arc<Self>) -> JSResult<()> {
        info!("a");
        let mut connected = self.connected.write().await;
        info!("b");
        if *connected {
            // already connected
            warn!("tried connecting while already connected");
            return Err(JoystickErr::AlreadyConnected);
        }


        let uri =
            Uri::from_str(&(String::from("wss://joystick.tv/cable?token=") + self.token.as_str()))?;
        let req = ClientRequestBuilder::new(uri).with_sub_protocol("actioncable-v1-json");


        let (ws_stream, _resp) = connect_async_tls_with_config(req, None, false, None).await?;

        let (write, read) = ws_stream.split();

        {
            let mut write_lock = self.ws_write.lock().await;
            let mut read_lock = self.ws_read.lock().await;
            write_lock.replace(write);
            read_lock.replace(read);
        }

        *connected = true;
        drop(connected);

        self.raw_send(ClientMessage::Subscribe(Subscribe::new()))
            .await?;

        Ok(())
    }

    pub(crate) async fn raw_send(self: &Arc<Self>, msg: ClientMessage) -> JSResult<()> {
        if !*self.connected.read().await {
            return Err(JoystickErr::NotConnected);
        }

        let serialized = msg.to_string();

        {
            let mut lock = self.ws_write.lock().await;
            let ws_write = lock.as_mut().ok_or(JoystickErr::NotConnected)?;

            ws_write.send(WsMessage::text(serialized)).await?;
        }

        Ok(())
    }

    async fn _next_msg(self: &Arc<Self>) -> JSOptionalResult<ServerMessage> {
        if !*self.connected.read().await {
            return JoystickErr::NotConnected.into();
        }

        let next: WsMessage;
        {
            let mut lock = self.ws_read.lock().await;
            let ws_read = lock.as_mut().ok_or(JoystickErr::NotConnected)?;

            next = ws_read.next().await??;
        }

        match next {
            Message::Text(msg_bytes) => {
                let msg = msg_bytes.to_string();
                let try_parsed = ServerMessage::from_str(msg.clone());
                if let Some(parsed) = try_parsed {
                    Yes(parsed)
                } else {
                    debug!("could not parse message: {}", msg);
                    Yes(ServerMessage::UnknownMessage(msg))
                }
            }
            Message::Close(_) => {
                info!("websocket connection closed by server");
                Nope
            }
            _ => {
                debug!("other websocket msg");
                Nope
            }
        }
    }

    async fn _next_server_msg(self: &Arc<Self>) -> JSOptionalResult<ServerMessage> {
        if let Yes(msg) = self._next_msg().await {
            return Yes(msg);
        }
        Nope
    }

    pub async fn try_next_event(self: &Arc<Self>) -> JSOptionalResult<Event> {
        let server_message = self._next_server_msg().await;

        let event: Event = {
            let t_event: Option<Event> = server_message?.into();
            t_event?
        };

        if event.included_by_intents(&self.intents) {
            Yes(event)
        } else {
            Nope
        }
    }
}
