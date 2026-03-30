use std::fmt;

use serde::Deserialize;

use crate::models::MapRotationResponse;

#[derive(Debug)]
pub enum ApiError {
    Throttled(String),
    Http(reqwest::Error),
    Parse(serde_json::Error),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Throttled(msg) => write!(f, "throttled: {msg}"),
            ApiError::Http(e) => write!(f, "http error: {e}"),
            ApiError::Parse(e) => write!(f, "parse error: {e}"),
        }
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(e: reqwest::Error) -> Self {
        ApiError::Http(e)
    }
}

#[derive(Deserialize)]
struct ApiErrorResponse {
    #[serde(rename = "Error")]
    error: String,
}

pub async fn get_maprotation_raw(apex_api_token: &str) -> Result<MapRotationResponse, ApiError> {
    // @docs https://apexlegendsapi.com/#map-rotation
    let url = format!("https://api.mozambiquehe.re/maprotation?auth={apex_api_token}&version=2");
    let text = reqwest::get(&url).await?.text().await?;

    // Check for API-level errors (e.g. throttle) before attempting full parse
    if let Ok(err) = serde_json::from_str::<ApiErrorResponse>(&text) {
        return Err(ApiError::Throttled(err.error));
    }

    serde_json::from_str::<MapRotationResponse>(&text).map_err(|e| {
        eprintln!("Decode error: {e}\nRaw body: {text}");
        ApiError::Parse(e)
    })
}
