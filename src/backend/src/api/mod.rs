pub mod error;

use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    mbz::MbzSession, navidrome::{NavidromeNativeSession, NavidromeSubsonicSession, Scrobble, SongData}, sqlite::InternalDB, storage::Storage
};

#[allow(unused)]
pub struct LoginSession {
    pub navidrome_native: NavidromeNativeSession,
    pub navidrome_subsonic: NavidromeSubsonicSession,
    pub uuid: uuid::Uuid,
    pub scrobbles: Vec<Scrobble>,
    pub tracks_hashmap: HashMap<String, SongData>,
    pub db_domain_id: i64
}

#[derive(Clone)]
pub struct ApiState {
    pub scrobbles: Arc<Vec<Scrobble>>,
    pub sessions: Arc<RwLock<HashMap<Uuid, LoginSession>>>,
    pub storage: Arc<Storage>
}

pub struct Range<T> {
    pub start: T,
    pub end: T
}

impl<T> Range<T>
where T: PartialOrd {
    pub fn contains(&self, other: &T) -> bool {
        return (self.start <= *other) && (self.end >= *other);
    }
}

impl ApiState {
    pub fn new(scrobbles: Vec<Scrobble>, mbz: Option<MbzSession>) -> Result<Self, rusqlite::Error> {
        let result = Self {
            scrobbles: Arc::new(scrobbles),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            storage: Arc::new(Storage::new(InternalDB::new("data.db".into())?, mbz))
        };

        return Ok(result);
    }
}
