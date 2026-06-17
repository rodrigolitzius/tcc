use crate::{
    handlers::*,
    navidrome::*,
};

pub async fn most_played_artists(
    State(state): State<ApiState>,
    Query(query): Query<HashMap<String, String>>,
    auth: Auth,
    range: Range<u64>
) -> Result<Json<serde_json::Value>, ApiError> {
    let session = get_session_from_uuid(&auth.uuid, &state.sessions).await?;

    let mut artist_stat: HashMap<String, ArtistStat> = HashMap::new();

    let scrobbles = Scrobble::filter_range(&session.scrobbles, range);

    for scrobble in scrobbles.iter() {
        let song_data = match session.tracks_hashmap.get(&scrobble.media_file_id) {
            Some(v) => v,
            None => continue
        };

        let duration_hour = song_data.duration / (60.0*60.0);

        for artist in song_data.participants.artists.iter() {
            match artist_stat.get_mut(&artist.id) {
                Some(v) => {
                    (*v).plays += 1;
                    (*v).played_hours += duration_hour
                },
                None => {
                    artist_stat.insert(
                        artist.id.clone(),
                        ArtistStat {
                            id: artist.id.clone(),
                            name: artist.name.clone(),
                            plays: 1,
                            played_hours: duration_hour
                        }
                    );
                }
            };
        }
    }

    let mut limit = get_param_default(&query, "limit", artist_stat.len());
    if limit > artist_stat.len() {
        limit = artist_stat.len() - 1
    }

    let mut all_artists: Vec<ArtistStat> = artist_stat.into_values().collect();

    all_artists.sort_by(|a, b| { b.played_hours.total_cmp(&a.played_hours)});
    let select = all_artists[..limit].to_vec();

    return Ok(Json(serde_json::to_value(select).unwrap()));
}
