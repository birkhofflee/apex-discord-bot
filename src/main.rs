mod api;
mod handler;
mod models;

use std::env;
use std::sync::Arc;

use serenity::prelude::*;
use tokio::sync::watch;

use api::{get_maprotation_raw, ApiError};
use handler::MapStatusHandler;
use models::MapRotationResponse;

#[tokio::main]
async fn main() {
    let apex_api_token = env::var("APEX_API_TOKEN").expect("APEX_API_TOKEN required");
    let token_rank = env::var("DISCORD_TOKEN_RANK").expect("DISCORD_TOKEN_RANK required");
    let token_pubs = env::var("DISCORD_TOKEN_PUBS").expect("DISCORD_TOKEN_PUBS required");

    let (tx, rx) = watch::channel::<Option<Arc<MapRotationResponse>>>(None);

    // Single poller — one API call, shared by all bots
    tokio::spawn(async move {
        loop {
            match get_maprotation_raw(&apex_api_token).await {
                Ok(rotation) => {
                    let _ = tx.send(Some(Arc::new(rotation)));
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
                Err(ApiError::Throttled(msg)) => {
                    eprintln!("Throttled: {msg}");
                    tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
                }
                Err(e) => {
                    eprintln!("Error fetching map rotation: {e}");
                    tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;
                }
            }
        }
    });

    let intents = GatewayIntents::GUILD_MESSAGES;

    let mut ranked_client = Client::builder(&token_rank, intents)
        .event_handler(MapStatusHandler {
            mode: "ranked",
            rx: rx.clone(),
        })
        .await
        .expect("Err creating ranked client");

    let mut pubs_client = Client::builder(&token_pubs, intents)
        .event_handler(MapStatusHandler {
            mode: "battle_royale",
            rx,
        })
        .await
        .expect("Err creating pubs client");

    tokio::join!(
        async {
            if let Err(why) = ranked_client.start().await {
                eprintln!("Err starting ranked client: {why:?}");
            }
        },
        async {
            if let Err(why) = pubs_client.start().await {
                eprintln!("Err starting pubs client: {why:?}");
            }
        },
    );
}
