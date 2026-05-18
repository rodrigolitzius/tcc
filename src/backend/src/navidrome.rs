use axum::http::{HeaderMap, HeaderValue};
use reqwest;
use serde::Deserialize;
use std::collections::HashMap;
use serde_json;

use crate::api::handlers::LoginRequest;
use crate::db_analyser::Scrobble;

#[allow(unused)]
pub struct Session {
    pub user_id: String,
    pub url: String,
    pub client: reqwest::Client,
    pub token: String
}

#[derive(Deserialize)]
struct LoginResponse {
    token: String,
    id: String
}

pub enum NavidromeSessionError {
    Reqwest(reqwest::Error),
    Unreachable(reqwest::Error),
    Unauthorized,
}

impl Session {
    pub async fn new(login_request: LoginRequest) -> Result<Self, NavidromeSessionError> {
        let client = match reqwest::Client::builder().tls_danger_accept_invalid_certs(true).build() {
            Ok(v) => v,
            Err(e) => {
                return Err(NavidromeSessionError::Reqwest(e));
            }
        };

        let login_request = LoginRequest {
            username: login_request.username,
            password: login_request.password,
            url: login_request.url.trim_end_matches('/').to_string()
        };

        let mut body = HashMap::new();
        body.insert("username", login_request.username);
        body.insert("password", login_request.password);

        let response = client
            .post(format!("{}/auth/login", login_request.url))
            .json(&body)
            .send()
            .await;

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

        let login_response: LoginResponse = response.json().await.expect("Required fields are missing from Navidrome's response");

        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            "x-nd-authorization",
            HeaderValue::from_str(format!("Bearer {}", login_response.token).as_str()).expect("Navidrome returned an invalid token")
        );

        let client = reqwest::Client::builder()
            .default_headers(default_headers)
            .build()
            .unwrap();

        return Ok(Self {
            url: login_request.url.to_string(),
            user_id: login_response.id,
            client: client,
            token: login_response.token,
        });
    }

    // TODO: Make a struct to deserialize from this json output
    pub async fn song(self: &Self, id: &str) -> Result<serde_json::Value, reqwest::Error> {
        let response = self.client.get(format!("{}/api/song/{}", self.url, id))
            .send()
            .await?
            .error_for_status()?;

        let json: serde_json::Value = response.json().await?;

        return Ok(json);
    }

    pub async fn build_track_hashmap(&self, scrobbles: &Vec<Scrobble>) -> HashMap<String, serde_json::Value> {
        let mut result = HashMap::new();

        for scrobble in scrobbles {
            if result.contains_key(&scrobble.media_file_id) {continue;}

            let song = self.song(&scrobble.media_file_id).await;
            if let Err(_) = song {continue;}

            result.insert(scrobble.media_file_id.clone(), song.unwrap());
        }

        return result;
    }
}
