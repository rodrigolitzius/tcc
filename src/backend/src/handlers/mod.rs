pub mod login;
pub mod recent;
pub mod relay;
pub mod artists;
pub mod albums;

use std::{str::FromStr, collections::HashMap};
use tokio::sync::{RwLock, RwLockReadGuard};
use num_traits::Bounded;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use axum::{
    extract::{FromRef, FromRequestParts, Query, Json, State, Path},
    http::{StatusCode, HeaderMap, Method},
    body::Bytes
};
use reqwest::header;

use crate::{
    api::{ApiState, LoginSession, Range, error::*},
};

pub struct Auth{uuid: Uuid}

#[derive(Serialize, Clone)]
struct ArtistStat {
    name: String,
    id: String,
    plays: u64,
    played_hours: f64
}

#[derive(Deserialize)]
pub struct RawLoginRequest {
    pub username: String,
    pub password: String,
    pub url: String
}

#[derive(Deserialize)]
#[derive(Clone)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub url: String
}

impl From<RawLoginRequest> for LoginRequest {
    fn from(value: RawLoginRequest) -> Self {
        return LoginRequest {
            username: value.username,
            password: value.password,
            url: value.url.trim_end_matches('/').to_string()
        };
    }
}

// TODO: Make this implementation use ApiError
impl<S> FromRequestParts<S> for Auth
where
    ApiState: axum::extract::FromRef<S>,
    S: Send + Sync
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut axum::http::request::Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = ApiState::from_ref(state);

        let auth_header = match parts.headers.get("Authorization") {
            None => {return Err(ApiError::Unauthorized("Missing Authorization header".into()))},
            Some(v) => v
        };

        let header_string = match auth_header.to_str() {
            Ok(v) => v,
            Err(_) => {return Err(ApiError::Unauthorized("Invalid Authorization header".into()))}
        };

        let uuid = match Uuid::from_str(header_string) {
            Ok(v) => v,
            Err(_) => {return Err(ApiError::Unauthorized("Authorization header is not and UUID".into()));}
        };

        return match state.sessions.read().await.contains_key(&uuid) {
            true => Ok(Auth{uuid: uuid}),
            false => {return Err(ApiError::Unauthorized("You don't have permission to access this".into()))}
        }
    }
}

pub fn get_param_default<T>(hashmap: &HashMap<String, String>, key: &str, default: T) -> T
where T: FromStr {
    let limit = hashmap.get(key);
    if let None = limit { return default; }

    let limit: T = limit.unwrap().parse().unwrap_or(default);

    return limit;
}

async fn get_session_from_uuid<'a>(uuid: &Uuid, sessions: &'a RwLock<HashMap<Uuid, LoginSession>>) -> Result<RwLockReadGuard<'a, LoginSession>, ApiError> {
    let session = sessions.read().await;

    let result = RwLockReadGuard::try_map(session, |f| {
        f.get(uuid)
    });

    let result = match result {
        Ok(v) => v,
        Err(_) => {
            return Err(ApiError::Internal("Could not find token".into()));
        }
    };

    return Ok(result);
}

impl<S, T> FromRequestParts<S> for Range<T>
where
    S: Send + Sync,
    T: Bounded + FromStr
{
    type Rejection = ApiError;
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let Query(queries) = match Query::<HashMap<String, String>>::from_request_parts(parts, state).await {
            Ok(v) => v,
            Err(_) => return Err(ApiError::BadRequest("Invalid queries".into()))
        };

        let (start, end) = (
            get_param_default(&queries, "a", T::min_value()),
            get_param_default(&queries, "b", T::max_value()),
        );

        let range = Self {start, end};

        return Ok(range);
    }
}
