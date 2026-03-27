use rusqlite::Row;

#[allow(unused)]
pub struct Scrobble {
    pub media_file_id: String,
    pub user_id: String,
    pub submission_time: i64
}

pub fn build_scrobble(row: &Row) -> Result<Scrobble, rusqlite::Error> {
    let media_file_id: String = row.get("media_file_id")?;
    let user_id: String = row.get("user_id")?;
    let submission_time: i64 = row.get("submission_time")?;

    return Ok(Scrobble {
        media_file_id, user_id, submission_time
    });
}
