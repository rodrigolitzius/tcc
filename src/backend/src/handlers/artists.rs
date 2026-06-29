use crate::{
    handlers::*,
    navidrome::*,
    analysis::artists::ArtistStat
};

pub async fn most_played_artists(
    State(state): State<ApiState>,
    Query(query): Query<HashMap<String, String>>,
    auth: Auth,
    range: Range<u64>
) -> Result<Json<serde_json::Value>, ApiError> {
    let session = get_session_from_uuid(&auth.uuid, &state.sessions).await?;

    let scrobbles = Scrobble::filter_range(&session.scrobbles, range);

    let artist_stat = ArtistStat::group(scrobbles, &session.tracks_hashmap);

    let mut limit = get_param_default(&query, "limit", artist_stat.len());
    if limit > artist_stat.len() {
        limit = artist_stat.len() - 1
    }

    let mut all_artists: Vec<ArtistStat> = artist_stat.into_values().collect();

    all_artists.sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});
    let select = all_artists[..limit].to_vec();

    return Ok(Json(serde_json::to_value(select).unwrap()));
}
