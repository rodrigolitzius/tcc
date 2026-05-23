use axum::http::{HeaderMap, HeaderValue};
use rand::RngExt;
use reqwest::{self, Method};
use serde::Deserialize;
use std::collections::HashMap;
use rand::{distr::Alphanumeric};
use serde_json;

use crate::api::handlers::LoginRequest;
use crate::api::response::ApiError;
use crate::db_analyser::Scrobble;

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

impl NavidromeNativeSession {
    pub async fn new(login_request: LoginRequest) -> Result<Self, NavidromeSessionError> {
        let client = match reqwest::Client::builder().tls_danger_accept_invalid_certs(true).build() {
            Ok(v) => v,
            Err(e) => {
                return Err(NavidromeSessionError::Reqwest(e));
            }
        };

        let mut body = HashMap::new();
        body.insert("username", login_request.username);
        body.insert("password", login_request.password);

        let response = client
            .post(format!("{}/auth/login", login_request.url))
            .json(&body)
            .send()
            .await;

        let response = validate_login_response(response)?;

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

impl NavidromeSubsonicSession {
    // TODO: Actually test if this fails if the login request has invalid credentials
    pub async fn new(login_request: LoginRequest) -> Result<Self, NavidromeSessionError> {
        let salt: String = rand::rng()
            .sample_iter(Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();

        let hash = format!("{:x}", md5::compute(format!("{}{}", login_request.password, salt)));

        let mut default_params: Vec<(String, String)> = Vec::new();
        default_params.push(("u".to_string(), login_request.username));
        default_params.push(("s".to_string(), salt.clone()));
        default_params.push(("t".to_string(), hash.clone()));
        default_params.push(("c".to_string(), crate::APP_NAME.to_string()));
        default_params.push(("v".to_string(), "1.8.0".to_string()));

        let response = reqwest::Client::new()
            .request(Method::GET, format!("{}/rest/ping", login_request.url))
            .query(&default_params)
            .send()
            .await;

        let _response = validate_login_response(response)?;

        let result = Self {
            default_params: default_params,
            client: reqwest::Client::new(),
            salt: salt,
            token: hash
        };

        return Ok(result)
    }
}
