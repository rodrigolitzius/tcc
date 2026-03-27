pub mod handlers;
pub mod login;

use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    navidrome::{Session},
    db_analyser::Scrobble
};

use uuid::Uuid;

#[allow(unused)]
pub struct LoginSession {
    pub navidrome_session: Session,
    pub uuid: uuid::Uuid,
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
