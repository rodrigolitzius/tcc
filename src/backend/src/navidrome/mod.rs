pub mod native;
pub mod subsonic;

use rusqlite::Row;
use uuid::Uuid;

use crate::{api::{
    Range, error::ApiError
}, navidrome::{native::AlbumData, subsonic::ResponseArtist}};

#[derive(Clone)]
pub struct Artist {
    #[allow(unused)]
    pub id: String,
    pub name: String,
    pub music_brainz_id: Option<Uuid>,
    pub albums: Vec<Album>
}

#[derive(Clone)]
pub struct Album {
    pub id: String,
    #[allow(unused)]
    pub name: String,
    pub year: u64,
}

#[derive(Clone)]
pub struct Scrobble {
    pub media_file_id: String,
    pub user_id: String,
    pub submission_time: u64
}

pub enum NavidromeSessionError {
    Reqwest(reqwest::Error),
    Unreachable(reqwest::Error),
    ParseJson(serde_json::Error),
    Unauthorized,
}

impl Artist {
    pub fn from_navidrome(artist: ResponseArtist, albums: Vec<AlbumData>) -> Artist {
        let mut new_albums: Vec<Album> = Vec::new();
        for album in albums {
            new_albums.push(album.into());
        }

        let result = Artist {
            albums: new_albums,
            id: artist.id,
            music_brainz_id: artist.music_brainz_id,
            name: artist.name
        };

        return result;
    }
}

impl From<AlbumData> for Album {
    fn from(value: AlbumData) -> Self {
        return Self {
            id: value.id,
            name: value.name,
            year: value.year
        };
    }
}

impl Scrobble {
    pub fn filter_range(scrobbles: &Vec<Scrobble>, range: Range<u64>) -> Vec<&Scrobble> {
        let mut refs: Vec<&Scrobble> = Vec::new();

        for scrobble in scrobbles {
            if range.contains(&scrobble.submission_time) {
                refs.push(&scrobble);
            }
        }

        return refs;
    }
}

impl From<NavidromeSessionError> for ApiError {
    fn from(value: NavidromeSessionError) -> Self {
        return match value {
            NavidromeSessionError::Reqwest(e) => ApiError::Internal(
                format!("Reqwest failed: {}", e.to_string())
            ),
            NavidromeSessionError::Unreachable(e) => ApiError::NavidromeUnreachable(
                format!("Navidrome could not be reached: {}", e.to_string())
            ),
            NavidromeSessionError::Unauthorized => ApiError::Unauthorized(
                "Invalid credentials".into()
            ),

            NavidromeSessionError::ParseJson(e) => ApiError::Internal(
                format!("Could not parse Navidrome's response: {}", e.to_string())
            )
        }
    }
}

impl From<serde_json::Error> for NavidromeSessionError {
    fn from(value: serde_json::Error) -> Self {
        return Self::ParseJson(value);
    }
}

pub fn validate_reqwest_response(response: Result<reqwest::Response, reqwest::Error>) -> Result<reqwest::Response, NavidromeSessionError> {
    let response = match response {
        Ok(v) => v,
        Err(e) => {
            return Err(NavidromeSessionError::Unreachable(e));
        }
    };

    let response = match response.error_for_status() {
        Ok(v) => v,
        Err(_) => {
            return Err(NavidromeSessionError::Unauthorized);
        }
    };

    return Ok(response);
}

pub fn build_scrobble(row: &Row) -> Result<Scrobble, rusqlite::Error> {
    let media_file_id: String = row.get("media_file_id")?;
    let user_id: String = row.get("user_id")?;
    let submission_time: i64 = row.get("submission_time")?;

    return Ok(Scrobble {
        media_file_id, user_id,
        submission_time: submission_time as u64
    });
}
