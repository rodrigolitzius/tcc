use crate::sqlite::*;

impl InternalDB {
    pub fn new(path: String) -> Result<Self, rusqlite::Error> {
        let value = Self {path};
        value.init()?;

        return Ok(value);
    }

    pub fn open(&self) -> Result<Connection, rusqlite::Error> {
        return Ok(Connection::open(self.path.clone())?);
    }

    fn init(&self) -> Result<(), rusqlite::Error> {
        let _changed = self.open()?.execute_batch("
            CREATE TABLE IF NOT EXISTS domain (
                id INTEGER PRIMARY KEY,
                domain TEXT NOT NULL UNIQUE
            );

            CREATE TABLE IF NOT EXISTS artist (
                id TEXT NOT NULL UNIQUE,
                domain_id INTEGER NOT NULL,
                json TEXT NOT NULL,

                FOREIGN KEY (domain_id) REFERENCES domain(id)
            );"
        )?;

        return Ok(());
    }

    pub fn add_domain(&self, domain: String) -> Result<(), rusqlite::Error> {
        let _changed = self.open()?.execute(format!("
            INSERT INTO domain (domain)
            VALUES ('{}')
            ", domain).as_str(), []
        );

        return Ok(());
    }

    pub fn add_artist(&self, domain_id: u64, artist_id: uuid::Uuid, json: serde_json::Value) -> Result<(), rusqlite::Error> {
        let db = self.open()?;
        db.execute(format!("
            INSERT INTO artist (id, domain_id, json) VALUES ('{}', {}, '{}')
        ", artist_id, domain_id, json).as_str(), [])?;

        return Ok(());
    }

    pub fn get_artist(&self, domain_id: u64, artist_id: uuid::Uuid) -> Result<Option<String>, rusqlite::Error> {
        let db = self.open()?;
        let mut stmt = db.prepare(format!("
            SELECT json FROM artist
            WHERE domain_id={} AND id='{}'
            LIMIT 1
        ", domain_id, artist_id).as_str())?;

        let queried_artists = stmt.query_map([], |row| {
            let json: String = row.get("json")?;
            return Ok(json);
        })?;

        let mut artists: Vec<String> = Vec::new();
        for artist in queried_artists {
            artists.push(artist?);
        }

        return Ok(artists.into_iter().nth(0));
    }
}
