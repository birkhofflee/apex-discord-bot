use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct MapRotationResponse {
    pub battle_royale: Option<ModeMapRotation>,
    pub ranked: Option<ModeMapRotation>,
    pub ltm: Option<ModeMapRotation>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ModeMapRotation {
    pub current: CurrentMap,
    pub next: NextMap,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
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

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
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
