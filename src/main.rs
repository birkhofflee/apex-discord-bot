use std::env;

use serenity::all::ActivityData;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::user::OnlineStatus;
use serenity::prelude::*;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MapRotationResponse {
    pub battle_royale: ModeMapRotation,
    pub ranked: ModeMapRotation,
    pub ltm: ModeMapRotation,
}

#[derive(Debug, Deserialize)]
pub struct ModeMapRotation {
    pub current: CurrentMap,
    pub next: NextMap,
}

#[derive(Debug, Deserialize)]
pub struct CurrentMap {
    pub start: u64,
    pub end: u64,
    #[serde(rename = "readableDate_start")]
    pub readable_date_start: String,
    #[serde(rename = "readableDate_end")]
    pub readable_date_end: String,
    pub map: String,
    pub code: String,
    #[serde(rename = "DurationInSecs")]
    pub duration_in_secs: u32,
    #[serde(rename = "DurationInMinutes")]
    pub duration_in_minutes: u32,
    pub asset: String,
    #[serde(rename = "remainingSecs")]
    pub remaining_secs: u32,
    #[serde(rename = "remainingMins")]
    pub remaining_mins: u32,
    #[serde(rename = "remainingTimer")]
    pub remaining_timer: String,
    #[serde(rename = "isActive")]
    pub is_active: Option<bool>,
    #[serde(rename = "eventName")]
    pub event_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NextMap {
    pub start: u64,
    pub end: u64,
    #[serde(rename = "readableDate_start")]
    pub readable_date_start: String,
    #[serde(rename = "readableDate_end")]
    pub readable_date_end: String,
    pub map: String,
    pub code: String,
    #[serde(rename = "DurationInSecs")]
    pub duration_in_secs: u32,
    #[serde(rename = "DurationInMinutes")]
    pub duration_in_minutes: u32,
    #[serde(rename = "isActive")]
    pub is_active: Option<bool>,
    #[serde(rename = "eventName")]
    pub event_name: Option<String>,
}

struct MapStatusHandler {
    mode: &'static str,
    apex_api_token: String,
}

#[async_trait]
impl EventHandler for MapStatusHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} ({}) is connected!", ready.user.name, self.mode);

        let apex_api_token = self.apex_api_token.clone();
        let mode = self.mode;

        tokio::spawn(async move {
            loop {
                match get_maprotation_raw(&apex_api_token).await {
                    Ok(rotation) => {
                        let map_rotation = match mode {
                            "ranked" => &rotation.ranked,
                            "ltm" => &rotation.ltm,
                            _ => &rotation.battle_royale,
                        };
                        let status = format!(
                            "{} ({} → {})",
                            map_rotation.current.map,
                            map_rotation.current.remaining_timer,
                            map_rotation.next.map
                        );
                        ctx.set_presence(
                            Some(ActivityData::playing(&status)),
                            OnlineStatus::Online,
                        );
                    }
                    Err(e) => eprintln!("Error fetching {mode} map rotation: {e}"),
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        });
    }
}

#[tokio::main]
async fn main() {
    let apex_api_token = env::var("APEX_API_TOKEN").expect("APEX_API_TOKEN required");
    let token_rank = env::var("DISCORD_TOKEN_RANK").expect("DISCORD_TOKEN_RANK required");
    let token_pubs = env::var("DISCORD_TOKEN_PUBS").expect("DISCORD_TOKEN_PUBS required");

    let intents = GatewayIntents::GUILD_MESSAGES;

    let mut ranked_client = Client::builder(&token_rank, intents)
        .event_handler(MapStatusHandler {
            mode: "ranked",
            apex_api_token: apex_api_token.clone(),
        })
        .await
        .expect("Err creating ranked client");

    let mut pubs_client = Client::builder(&token_pubs, intents)
        .event_handler(MapStatusHandler {
            mode: "battle_royale",
            apex_api_token,
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

async fn get_maprotation_raw(apex_api_token: &str) -> Result<MapRotationResponse, reqwest::Error> {
    // @docs https://apexlegendsapi.com/#map-rotation
    let url = format!("https://api.mozambiquehe.re/maprotation?auth={apex_api_token}&version=2");
    let text = reqwest::get(&url).await?.text().await?;
    match serde_json::from_str::<MapRotationResponse>(&text) {
        Ok(r) => Ok(r),
        Err(e) => {
            eprintln!("Decode error: {e}\nRaw body: {text}");
            // Re-raise as a reqwest error by re-parsing (this will fail and propagate)
            reqwest::get(&url).await?.json::<MapRotationResponse>().await
        }
    }
}
