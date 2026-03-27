use axum::{
    extract::{FromRef, FromRequestParts, Json, Query, State},
    http::StatusCode
};

use serde_json::json;
use uuid::Uuid;
use std::{collections::HashMap, str::FromStr};

use crate::api::{
    ApiState, LoginSession,
    login::{build_user_track_hashmap, LoginRequest}
};

use crate::{
    navidrome::{Session},
    util::get_param_default
};

pub struct Auth{uuid: Uuid}

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
) -> Result<String, StatusCode> {
    let mut limit = get_param_default(&query, "limit", 0) as usize;
    let offset = get_param_default(&query, "offset", 0) as usize;

    if limit == 0 {
        limit = state.scrobbles.len();
    }

    let session = state.sessions.read().await;
    let session = match session.get(&auth.uuid) {
        Some(v) => v,
        None => {return Err(StatusCode::INTERNAL_SERVER_ERROR);}
    };

    let mut result = String::new();
    for scrobble in state.scrobbles.iter().skip(offset).take(limit) {
        if !(scrobble.user_id == session.navidrome_session.user_id) {continue;}

        let music_info = match session.tracks_hashmap.get(&scrobble.media_file_id) {
            Some(v) => v,
            None => {continue;}
        };

        result.push_str(format!("{} - {}\n",
            music_info["title"],
            music_info["artist"],
        ).as_str());
    }

    return Ok(result);
}

pub async fn login(
    State(state): State<ApiState>,
    Json(login_request): Json<LoginRequest>
) -> (StatusCode, Json<serde_json::Value>) {
    let url = &login_request.url;
    let username = &login_request.username;
    let password = &login_request.password;

    let navidrome_session = Session::new(url, username, password).await;

    if let Err(e) = navidrome_session {
        if let Some(s) = e.status() {
            return (s, Json(json!({"status": "error"})));
        } else {
            return (StatusCode::NOT_FOUND, Json(json!({"status": "error"})));
        }
    }

    // TODO: The build_user_track_hashmap function is SUPER SLOW, and blocks the servers response. Make it go vroom vroom
    let mut navidrome_session = navidrome_session.expect("The session should be valid from this point. Did the function not return?");
    let tracks_hashmap = build_user_track_hashmap(&state.scrobbles, &mut navidrome_session).await;
    let uuid = Uuid::new_v4();

    // let session_id = Uuid::new_v4();
    let login_session = LoginSession {
        navidrome_session, tracks_hashmap, uuid
    };

    state.sessions.write().await.insert(login_session.uuid, login_session);

    return (StatusCode::OK, Json(json!({"status": "success", "id": uuid})));
}
