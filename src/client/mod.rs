use std::str::FromStr;
use crate::client::message::client_message::subscribe::Subscribe;
use crate::client::message::client_message::ClientMessage;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use base64::Engine;
use base64::prelude::BASE64_URL_SAFE;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::{Message as WsMessage, Message};
use tokio_tungstenite::{connect_async_tls_with_config, MaybeTlsStream, WebSocketStream};
use tungstenite::ClientRequestBuilder;
use tungstenite::http::Uri;
use crate::client::message::server_message::ServerMessage;

mod channel;
mod event;
mod message;

pub struct Client {
    token: String,
    ws_write: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, WsMessage>>>,
    ws_read: Arc<Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
}

impl Client {
    pub async fn connect(id: String, secret: String) -> Self {

        let token = BASE64_URL_SAFE.encode(id + ":" + &secret);

        let uri = Uri::from_str(&(String::from("wss://joystick.tv/cable?token=") + token.as_str())).unwrap();
        let req = ClientRequestBuilder::new(uri).with_sub_protocol("actioncable-v1-json");

        let (mut ws_stream, resp) = connect_async_tls_with_config(req, None, false, None).await.unwrap();

        let (write, read) = ws_stream.split();

        let ws_read = Arc::new(Mutex::new(read));
        let ws_write = Arc::new(Mutex::new(write));

        let mut client = Self {
            token,
            ws_write,
            ws_read,
        };

        client.raw_send(ClientMessage::Subscribe(Subscribe::new())).await;

        client
    }

    pub(crate) async fn raw_send(&mut self, msg: ClientMessage) {
        let mut ws_write = self.ws_write.lock().await;

        let serialized = msg.to_string();

        ws_write.send(WsMessage::text(serialized)).await.unwrap();
    }

    async fn _next_msg(&mut self) -> Option<ServerMessage> {
        let next: WsMessage;
        {
            let mut ws_read = self.ws_read.lock().await;
            next = ws_read.next().await.unwrap().unwrap();
        }

        match next {
            Message::Text(msg) => {
                Some(ServerMessage::from_str(msg)?)
            }
            Message::Close(_) => {
                println!("websocket connection closed by server");
                None
            }
            _ => {
                println!("other websocket msg");
                None
            }
        }
    }

    pub(crate) async fn get_next_event(&mut self) {
        if let Some(msg) = self._next_msg().await {
            println!("{:?}", msg);
        }


    }
}

