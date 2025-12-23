use crate::client::Client;
use client::model::events::Intent;
use client::model::events::Intent::{Chat, StreamStart, UserJoin, UserLeave};
use dotenv::dotenv;
use std::env;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

pub mod client;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("could not create logging subscriber");

    dotenv().ok();

    info!("connecting");

    let client = {
        let bot_id = env::var("BOT_ID").expect("Could not load bot id from .env");
        let bot_token = env::var("BOT_TOKEN").expect("Could not load token from .env");
        let intents: Vec<Intent> = vec![UserLeave, UserJoin, Chat, StreamStart];

        Client::connect(bot_id, bot_token, intents).await
    };

    info!("listening");

    loop {
        if let Some(event) = client.try_next_event().await {
            info!("event: {:?}", event)
        }
    }
}
