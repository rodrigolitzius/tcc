mod storage;

use crate::{
    mbz::{MbzError, MbzSession},
    sqlite::InternalDB
};

pub struct Storage {
    pub db: InternalDB,
    pub mbz: Option<MbzSession>
}

#[allow(unused)]
#[derive(Debug)]
pub enum StorageError {
    Rusqlite(rusqlite::Error),
    Reqwest(reqwest::Error),
    ParseJson(serde_json::Error)
}

impl From<MbzError> for StorageError {
    fn from(value: MbzError) -> Self {
        return match value {
            MbzError::ParseJson(e) => StorageError::ParseJson(e),
            MbzError::Reqwest(e) => StorageError::Reqwest(e)
        }
    }
}

impl From<serde_json::Error> for StorageError {
    fn from(value: serde_json::Error) -> Self {
        return StorageError::ParseJson(value)
    }
}

impl From<rusqlite::Error> for StorageError {
    fn from(value: rusqlite::Error) -> Self {
        return StorageError::Rusqlite(value)
    }
}
