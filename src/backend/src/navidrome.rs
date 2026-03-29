use axum::http::{HeaderMap, HeaderValue};
use reqwest;
use serde::Deserialize;
use std::collections::HashMap;
use serde_json;

use crate::api::login::LoginRequest;

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

impl Session {
    pub async fn new(login_request: LoginRequest) -> Result<Self, reqwest::Error> {
        let client = reqwest::Client::builder().tls_danger_accept_invalid_certs(true).build()?;

        let mut body = HashMap::new();
        body.insert("username", login_request.username);
        body.insert("password", login_request.password);

        let response = client
            .post(format!("{}/auth/login", login_request.url))
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

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
}
