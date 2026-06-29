use serde::Serialize;

use crate::{
    handlers::*,
    analysis::{albums::AlbumStat, artist::ArtistStat},
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

    let artist = Artist::from_navidrome(
        session.navidrome_subsonic.get_artist(&id).await?,
        session.navidrome_native.album(&id).await?
    );

    let album_ids: Vec<String> = artist.albums.iter().map(|a| a.id.clone()).collect();
    let album_stat = AlbumStat::group(scrobbles, &session.tracks_hashmap, Some(album_ids.clone()));

    let mbz_artist = match artist.music_brainz_id {
        Some(v) => state.storage.get_artist(session.db_domain_id, v).await?,
        None => None
    };

    let mut artist_stat = ArtistStat::get(artist, album_stat, mbz_artist);

    artist_stat.albums.sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});

    let albums_response: Vec<AlbumResponse> = artist_stat.albums.iter().map(|a| {
        AlbumResponse {
            id: a.id.clone(),
            name: a.name.clone(),
            played_hours: a.played_hours,
            plays: a.plays,
            year: a.year
        }
    }).collect();

    let result = ArtistResponse {
        artist_type: artist_stat.artist_type,
        gender: artist_stat.gender,
        name: artist_stat.name,
        album_count: artist_stat.albums.len() as u64,
        albums: albums_response
    };

    return Ok(Json(serde_json::to_value(result).unwrap()));
}
