use joysticktv::client::Client;
use joysticktv::client::model::events::Intent;
use joysticktv::client::model::events::Intent::{Chat, StreamStart, UserJoin, UserLeave};
use dotenv::dotenv;
use std::env;
use meby::Meby;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("could not create logging subscriber");

    dotenv().ok();

    info!("building");

    let client = {
        let bot_id = env::var("BOT_ID").expect("Could not load bot id from .env");
        let bot_token = env::var("BOT_TOKEN").expect("Could not load token from .env");
        let intents: Vec<Intent> = vec![UserLeave, UserJoin, Chat, StreamStart];
        Client::new(bot_id, &bot_token, intents)
    };

    info!("connecting");

    client.connect().await.expect("Could not connect");

    info!("listening");

    loop {
        if let Meby::Yes(event) = client.try_next_event().await {
            info!("event: {:?}", event)
        }
    }
}
