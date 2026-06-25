use serde::Serialize;

use crate::{
    handlers::*,
    analysis::{GroupScrobble, albums::AlbumStat},
    navidrome::{Scrobble, AlbumGetArtist}
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

    let artist_info = session.navidrome_subsonic.get_artist(id).await?;

    let mut artist_info_albums: HashMap<String, AlbumGetArtist> = HashMap::new();
    for album in artist_info.album {
        let _ = artist_info_albums.insert(album.id.clone(), album);
    }

    let ids: Vec<String> = artist_info_albums.clone().into_keys().collect();

    let album_stat_albums = AlbumStat::group((scrobbles, &session.tracks_hashmap), Some(ids.clone()));

    let mut response_albums: Vec<AlbumResponse> = Vec::new();
    for id in ids {
        let stat = album_stat_albums.get(&id).ok_or(ApiError::Internal("Missing IDs".into()))?;
        let get_artist = artist_info_albums.get(&id).ok_or(ApiError::Internal("Missing IDs".into()))?;

        response_albums.push(AlbumResponse {
            id: id,
            name: stat.name.clone(),
            played_hours: stat.played_hours,
            plays: stat.plays,
            year: get_artist.year,
        });
    }

    let mbz_artist = match artist_info.music_brainz_id {
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
        name: artist_info.name,
        album_count: artist_info.album_count,
        albums: response_albums
    };

    return Ok(Json(serde_json::to_value(result).unwrap()));
}
