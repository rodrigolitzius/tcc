mod internal_db;

use rusqlite::Connection;

pub struct InternalDB {
    pub path: String
}
