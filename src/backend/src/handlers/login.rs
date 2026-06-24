use crate::{
    handlers::*,
    navidrome::*
};

pub async fn login(
    State(state): State<ApiState>,
    Json(login_request): Json<RawLoginRequest>
) -> Result<Json<serde_json::Value>, ApiError> {
    let login_request: LoginRequest = login_request.into();

    let navidrome_native = NavidromeNativeSession::new(login_request.clone()).await?;
    let navidrome_subsonic = NavidromeSubsonicSession::new(login_request.clone()).await?;

    let mut scrobbles: Vec<Scrobble> = Vec::new();
    for scrobble in state.scrobbles.iter() {
        if scrobble.user_id != navidrome_native.user_id {continue;}

        scrobbles.push(scrobble.clone());
    }

    let tracks_hashmap = navidrome_native.build_track_hashmap(&scrobbles).await?;
    let uuid = Uuid::new_v4();

    let login_session = LoginSession {
        navidrome_native, navidrome_subsonic, tracks_hashmap, uuid, scrobbles
    };

    match state.storage.db.add_domain(login_request.url) {
        Ok(v) => v,
        Err(_) => {
            return Err(ApiError::DatabaseError("Could not add domain to database".into()));
        }
    };

    state.sessions.write().await.insert(login_session.uuid, login_session);

    return Ok(Json(json!({"id": uuid})));
}
