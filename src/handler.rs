use std::sync::Arc;

use serenity::all::ActivityData;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::user::OnlineStatus;
use serenity::prelude::*;
use tokio::sync::watch;

use crate::models::MapRotationResponse;

pub struct MapStatusHandler {
    pub mode: &'static str,
    pub rx: watch::Receiver<Option<Arc<MapRotationResponse>>>,
}

#[async_trait]
impl EventHandler for MapStatusHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} ({}) is connected!", ready.user.name, self.mode);

        let mode = self.mode;
        let mut rx = self.rx.clone();

        tokio::spawn(async move {
            loop {
                // Wait for the shared data to change
                if rx.changed().await.is_err() {
                    break;
                }
                let rotation = rx.borrow().clone();
                if let Some(rotation) = rotation {
                    let map_rotation = match mode {
                        "ranked" => &rotation.ranked,
                        "ltm" => match rotation.ltm.as_ref() {
                            Some(ltm) => ltm,
                            None => continue,
                        },
                        _ => &rotation.battle_royale,
                    };
                    let status = format!(
                        "{} ({} → {})",
                        map_rotation.current.map,
                        map_rotation.current.remaining_timer,
                        map_rotation.next.map
                    );
                    ctx.set_presence(Some(ActivityData::playing(&status)), OnlineStatus::Online);
                }
            }
        });
    }
}
