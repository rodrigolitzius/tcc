use crate::storage::*;
use crate::mbz::MbzArtist;

impl Storage {
    pub fn new(db: InternalDB, mbz: Option<MbzSession>) -> Self {
        return Self {db, mbz};
    }

    pub async fn get_artist(&self, domain_id: i64, artist_id: uuid::Uuid) -> Result<Option<MbzArtist>, StorageError> {
        match self.db.get_artist(domain_id, artist_id)? {
            Some(v) => {
                return Ok(Some(serde_json::from_str::<MbzArtist>(v.as_str())?))
            }
            None => {}
        };

        match &self.mbz {
            Some(v) => {
                let artist = v.get_artist(artist_id).await?;

                match self.db.add_artist(domain_id, artist_id, serde_json::to_value(artist.clone())?) {
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("Failed to add artist to database: {e}");
                    }
                };

                return Ok(Some(artist));
            },
            None => {}
        }

        return Ok(None);
    }
}
