mod session;

use axum::http::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize, de::Error};
use uuid::Uuid;
use reqwest::{Client, ClientBuilder};

const MBZ_URL: &'static str = "https://api.listenbrainz.org/1";

pub struct MbzSession {
    client: Client,

    #[allow(unused)]
    token: Uuid
}

pub enum MbzError {
    Reqwest(reqwest::Error),
    ParseJson(serde_json::Error)
}

impl From<reqwest::Error> for MbzError {
    fn from(value: reqwest::Error) -> Self {
        return MbzError::Reqwest(value);
    }
}

impl From<serde_json::Error> for MbzError {
    fn from(value: serde_json::Error) -> Self {
        return MbzError::ParseJson(value);
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MbzArtist {
    pub gender: Option<String>,
    #[serde(rename = "type")]
    pub artist_type: String
}
