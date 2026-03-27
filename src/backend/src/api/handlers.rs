use axum::{
    extract::{Json, Query, State},
    http::StatusCode
};

use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;
use std::collections::HashMap;

use crate::api::{
    ApiState, LoginSession,
    login::{build_user_track_hashmap}
};

use crate::{
    navidrome::{Session},
    util::get_param_default
};

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
    url: String
}

pub fn return_json_status(status: &str, msg: &str, detail: &str) -> Json<serde_json::Value> {
    return Json(json!({"status": status, "msg": msg, "detail": detail}));
}

pub async fn login(State(state): State<ApiState>, Json(login_request): Json<LoginRequest>) -> StatusCode {
    let url = &login_request.url;
    let username = &login_request.username;
    let password = &login_request.password;

    let navidrome_session = Session::new(url, username, password).await;

    if let Err(e) = navidrome_session {
        if let Some(s) = e.status() {
            return s;
        } else {
            return StatusCode::NOT_FOUND;
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

    let mut json = return_json_status("success", "login successful", "");

    if let Some(obj) = json.as_object_mut() {
        obj.insert("token".to_string(), json!(uuid));
    }

    return StatusCode::OK;
}

pub async fn scrobbles_to_text(
    State(state): State<ApiState>,
    Query(query): Query<HashMap<String, String>>
) -> String {
    let mut limit = get_param_default(&query, "limit", 0) as usize;
    let offset = get_param_default(&query, "offset", 0) as usize;

    if limit == 0 {
        limit = state.scrobbles.len();
    }

    let mut result = String::new();
    for scrobble in state.scrobbles.iter().skip(offset).take(limit) {
        result.push_str(format!("{} - {}\n",
            scrobble.media_file_id,
            scrobble.user_id,
        ).as_str());
    }

    return result;
}
