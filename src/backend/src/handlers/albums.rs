use crate::{
    handlers::*,
    navidrome::*,
    analysis::albums::AlbumStat
};

pub async fn most_played_albums(
    State(state): State<ApiState>,
    Query(query): Query<HashMap<String, String>>,
    auth: Auth,
    range: Range<u64>
) -> Result<Json<serde_json::Value>, ApiError> {
    let session = get_session_from_uuid(&auth.uuid, &state.sessions).await?;

    let scrobbles = Scrobble::filter_range(&session.scrobbles, range);

    let album_stat = AlbumStat::group(scrobbles, &session.tracks_hashmap, None);

    let mut limit = get_param_default(&query, "limit", album_stat.len());
    if limit > album_stat.len() {
        limit = album_stat.len() - 1
    }

    let mut all_albums: Vec<AlbumStat> = album_stat.into_values().collect();

    all_albums.sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});
    let select = all_albums[..limit].to_vec();

    return Ok(Json(serde_json::to_value(select).unwrap()));
}
