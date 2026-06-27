use serde::Serialize;

use crate::{
    handlers::*,
    analysis::{GroupScrobble, albums::AlbumStat},
    navidrome::{Scrobble, Artist}
};

#[derive(Serialize)]
struct ArtistResponse {
    name: String,
    album_count: u64,
    artist_type: Option<String>,
    gender: Option<String>,
    albums: Vec<AlbumResponse>
}

#[derive(Serialize)]
struct AlbumResponse {
    id: String,
    name: String,
    year: u64,
    plays: u64,
    played_hours: f64
}

pub async fn artist_info(
    State(state): State<ApiState>,
    Path(id): Path<String>,
    auth: Auth,
    range: Range<u64>
) -> Result<Json<serde_json::Value>, ApiError> {
    let session = get_session_from_uuid(&auth.uuid, &state.sessions).await?;
    let scrobbles = Scrobble::filter_range(&session.scrobbles, range);

    let artist_info = session.navidrome_subsonic.get_artist(&id).await?;
    let artist_albums = session.navidrome_native.album(&id).await?;

    let artist = Artist::from_navidrome(artist_info, artist_albums);

    let album_ids: Vec<String> = artist.albums.iter().map(|a| a.id.clone()).collect();

    let album_stat = AlbumStat::group((scrobbles, &session.tracks_hashmap), Some(album_ids.clone()));

    let mut response_albums: Vec<AlbumResponse> = Vec::new();

    let mut missing_albums: usize = 0;
    for album in &artist.albums {
        let stat = match album_stat.get(&album.id) {
            Some(v) => v,
            None => {
                missing_albums += 1;
                continue;
            }
        };

        response_albums.push(AlbumResponse {
            id: album.id.clone(),
            name: stat.name.clone(),
            played_hours: stat.played_hours,
            plays: stat.plays,
            year: album.year,
        });
    }

    if missing_albums == artist.albums.len() {
        return Err(ApiError::Internal("Artist has no albums played".into()))
    }

    let mbz_artist = match artist.music_brainz_id {
        Some(v) => state.storage.get_artist(session.db_domain_id, v).await?,
        None => None
    };

    let mut artist_type = Option::None;
    let mut gender = Option::None;

    match mbz_artist {
        Some(v) => {
            artist_type = Some(v.artist_type);
            gender = v.gender;
        },

        None => {}
    }

    response_albums.sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});

    let result = ArtistResponse {
        artist_type: artist_type,
        gender: gender,
        name: artist.name,
        album_count: artist.albums.len() as u64,
        albums: response_albums
    };

    return Ok(Json(serde_json::to_value(result).unwrap()));
}
