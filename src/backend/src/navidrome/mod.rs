pub mod native;
pub mod subsonic;

use axum::http::{HeaderMap, HeaderValue};
use rand::{distr::Alphanumeric};
use std::collections::HashMap;
use reqwest::{self, Method};
use serde::Deserialize;
use reqwest::Client;
use rand::RngExt;
use serde_json;

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
    pub client: reqwest::Client,
    pub salt: String,
    pub token: String
}

pub enum NavidromeSessionError {
    Reqwest(reqwest::Error),
    Unreachable(reqwest::Error),
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
struct LoginResponse {
    token: String,
    id: String
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
            )
        }
    }
}

pub fn validate_login_response(response: Result<reqwest::Response, reqwest::Error>) -> Result<reqwest::Response, NavidromeSessionError> {
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
