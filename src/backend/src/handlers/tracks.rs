use crate::{
    handlers::*,
    navidrome::*,
    analysis::tracks::TrackStat
};

pub async fn most_played_tracks(
    State(state): State<ApiState>,
    Query(query): Query<HashMap<String, String>>,
    auth: Auth,
    range: Range<u64>
) -> Result<Json<serde_json::Value>, ApiError> {
    let session = get_session_from_uuid(&auth.uuid, &state.sessions).await?;

    let scrobbles = Scrobble::filter_range(&session.scrobbles, range);

    let tracks_stat = TrackStat::group(scrobbles, &session.tracks_hashmap);

    let mut limit = get_param_default(&query, "limit", tracks_stat.len());
    if limit > tracks_stat.len() {
        limit = tracks_stat.len() - 1
    }

    let mut all_tracks: Vec<TrackStat> = tracks_stat.into_values().collect();

    all_tracks.sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});
    let select = all_tracks[..limit].to_vec();

    return Ok(Json(serde_json::to_value(select).unwrap()));
}
