use axum::{
    extract::{FromRef, FromRequestParts, Json, Query, State},
    http::StatusCode
};

use serde_json::json;
use serde::Deserialize;
use uuid::Uuid;
use std::{collections::HashMap, str::FromStr};

use crate::{api::{
    ApiState, LoginSession, response::*
}, db_analyser::Scrobble};

use crate::{
    navidrome::{Session, NavidromeSessionError},
    util::get_param_default
};

pub struct Auth{uuid: Uuid}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub url: String
}

impl<S> FromRequestParts<S> for Auth
where
    ApiState: axum::extract::FromRef<S>,
    S: Send + Sync
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut axum::http::request::Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = ApiState::from_ref(state);

        let auth_header = match parts.headers.get("Authorization") {
            None => {return Err(StatusCode::UNAUTHORIZED)},
            Some(v) => v
        };

        let header_string = match auth_header.to_str() {
            Ok(v) => v,
            Err(_) => {return Err(StatusCode::UNAUTHORIZED)}
        };

        let uuid = match Uuid::from_str(header_string) {
            Ok(v) => v,
            Err(_) => {return Err(StatusCode::UNAUTHORIZED);}
        };

        return match state.sessions.read().await.contains_key(&uuid) {
            true => Ok(Auth{uuid: uuid}),
            false => Err(StatusCode::UNAUTHORIZED)
        }
    }
}

pub async fn recent(
    State(state): State<ApiState>,
    Query(query): Query<HashMap<String, String>>,
    auth: Auth
) -> Result<Json<serde_json::Value>, ApiError> {
    let mut limit = get_param_default(&query, "limit", 0) as usize;
    let offset = get_param_default(&query, "offset", 0) as usize;

    if limit == 0 {
        limit = state.scrobbles.len();
    }

    let session = state.sessions.read().await;
    let session = match session.get(&auth.uuid) {
        Some(v) => v,
        None => {
            return Err(ApiError::Internal("Could not find token".into()));
        }
    };

    let mut result: Vec<serde_json::Value> = Vec::new();
    for scrobble in session.scrobbles.iter().skip(offset).take(limit) {
        let music_info = match session.tracks_hashmap.get(&scrobble.media_file_id) {
            Some(v) => v,
            None => {continue;}
        };

        result.push(json!({
            "title": music_info["title"],
            "artist": music_info["artist"],
            "album": music_info["album"],
        }));
    }

    return Ok(Json(serde_json::to_value(result).unwrap()));
}

pub async fn login(
    State(state): State<ApiState>,
    Json(login_request): Json<LoginRequest>
) -> Result<Json<serde_json::Value>, ApiError> {
    let navidrome_session = Session::new(login_request).await;

    let navidrome_session = match navidrome_session {
        Ok(v) => v,
        Err(e) => {
            match e {
                NavidromeSessionError::Reqwest(e2) => {
                    return Err(ApiError::Internal(e2.to_string()))
                },
                NavidromeSessionError::Unreachable(e2) => {
                    return Err(ApiError::LoginNavidromeUnreachable(e2.to_string()));
                },
                NavidromeSessionError::Unauthorized => {
                    return Err(ApiError::Unauthorized("Incorrect credentials".into()));
                }
            }
        }
    };

    let mut scrobbles: Vec<Scrobble> = Vec::new();
    for scrobble in state.scrobbles.iter() {
        if scrobble.user_id != navidrome_session.user_id {continue;}

        scrobbles.push(scrobble.clone());
    }

    // TODO: The build_user_track_hashmap function is SUPER SLOW, and blocks the servers response. Make it go vroom vroom
    let tracks_hashmap = navidrome_session.build_track_hashmap(&scrobbles).await;
    let uuid = Uuid::new_v4();

    let login_session = LoginSession {
        navidrome_session, tracks_hashmap, uuid, scrobbles
    };

    state.sessions.write().await.insert(login_session.uuid, login_session);

    return Ok(Json(json!({"id": uuid})));
}
