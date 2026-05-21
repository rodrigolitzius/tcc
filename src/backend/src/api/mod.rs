pub mod handlers;
pub mod response;

use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    db_analyser::Scrobble, navidrome::{NavidromeNativeSession, NavidromeSubsonicSession}
};

#[allow(unused)]
pub struct LoginSession {
    pub navidrome_native: NavidromeNativeSession,
    pub navidrome_subsonic: NavidromeSubsonicSession,
    pub uuid: uuid::Uuid,
    pub scrobbles: Vec<Scrobble>,
    pub tracks_hashmap: HashMap<String, serde_json::Value>,
}

#[derive(Clone)]
pub struct ApiState {
    pub scrobbles: Arc<Vec<Scrobble>>,
    pub sessions: Arc<RwLock<HashMap<Uuid, LoginSession>>>
}

impl Default for ApiState {
    fn default() -> Self {
        return Self {
            scrobbles: Arc::new(Vec::new()),
            sessions: Arc::new(RwLock::new(HashMap::new()))
        };
    }
}
