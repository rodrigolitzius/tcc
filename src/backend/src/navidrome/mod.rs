pub mod native;
pub mod subsonic;

use axum::http::{HeaderMap, HeaderValue};
use rand::{distr::Alphanumeric};
use uuid::Uuid;
use std::collections::HashMap;
use reqwest::{self, Method};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use rand::RngExt;

use crate::handlers::LoginRequest;
use crate::api::{Range, error::*};

#[allow(unused)]
pub struct NavidromeNativeSession {
    pub user_id: String,
    pub url: String,
    pub client: reqwest::Client,
    pub token: String
}

#[allow(unused)]
pub struct NavidromeSubsonicSession {
    pub default_params: Vec<(String, String)>,
    pub url: String,
    pub client: reqwest::Client,
    pub salt: String,
    pub token: String
}

pub enum NavidromeSessionError {
    Reqwest(reqwest::Error),
    Unreachable(reqwest::Error),
    ParseJson(serde_json::Error),
    Unauthorized,
}

#[derive(Clone)]
#[allow(unused)]
pub struct Scrobble {
    pub media_file_id: String,
    pub user_id: String,
    pub submission_time: u64
}

#[derive(Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub id: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistGetArtist {
    pub name: String,
    pub album_count: u64,
    pub album: Vec<AlbumGetArtist>,
    pub music_brainz_id: Option<Uuid>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AlbumGetArtist {
    pub id: String,
    pub year: u64
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SongData {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub album_artist: String,
    pub album_id: String,
    pub duration: f64,
    pub participants: Participants
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Participants {
    #[serde(rename = "artist")]
    pub artists: Vec<Artist>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub missing: bool,
}

impl Scrobble {
    pub fn filter_range(scrobbles: &Vec<Scrobble>, range: Range<u64>) -> Vec<&Scrobble> {
        let mut refs: Vec<&Scrobble> = Vec::new();

        for scrobble in scrobbles {
            if range.contains(&scrobble.submission_time) {
                refs.push(&scrobble);
            }
        }

        return refs;
    }
}

impl From<NavidromeSessionError> for ApiError {
    fn from(value: NavidromeSessionError) -> Self {
        return match value {
            NavidromeSessionError::Reqwest(e) => ApiError::Internal(
                format!("Reqwest failed: {}", e.to_string())
            ),
            NavidromeSessionError::Unreachable(e) => ApiError::NavidromeUnreachable(
                format!("Navidrome could not be reached: {}", e.to_string())
            ),
            NavidromeSessionError::Unauthorized => ApiError::Unauthorized(
                "Invalid credentials".into()
            ),

            NavidromeSessionError::ParseJson(e) => ApiError::Internal(
                format!("Could not parse Navidrome's response: {}", e.to_string())
            )
        }
    }
}

impl From<serde_json::Error> for NavidromeSessionError {
    fn from(value: serde_json::Error) -> Self {
        return Self::ParseJson(value);
    }
}

pub fn validate_reqwest_response(response: Result<reqwest::Response, reqwest::Error>) -> Result<reqwest::Response, NavidromeSessionError> {
    let response = match response {
        Ok(v) => v,
        Err(e) => {
            return Err(NavidromeSessionError::Unreachable(e));
        }
    };

    let response = match response.error_for_status() {
        Ok(v) => v,
        Err(_) => {
            return Err(NavidromeSessionError::Unauthorized);
        }
    };

    return Ok(response);
}

use rusqlite::Row;

pub fn build_scrobble(row: &Row) -> Result<Scrobble, rusqlite::Error> {
    let media_file_id: String = row.get("media_file_id")?;
    let user_id: String = row.get("user_id")?;
    let submission_time: i64 = row.get("submission_time")?;

    return Ok(Scrobble {
        media_file_id, user_id,
        submission_time: submission_time as u64
    });
}
