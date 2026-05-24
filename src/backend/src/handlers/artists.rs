use crate::{
    handlers::*,
    navidrome::*,
};

pub async fn most_played_artists(
    State(state): State<ApiState>,
    auth: Auth,
    range: Range<u64>
) -> Result<Json<serde_json::Value>, ApiError> {
    let session = get_session_from_uuid(&auth.uuid, &state.sessions).await?;

    let mut artist_count: HashMap<String, ArtistCount> = HashMap::new();

    let scrobbles = Scrobble::filter_range(&session.scrobbles, range);

    for v in scrobbles.iter() {
        let song_data = match session.tracks_hashmap.get(&v.media_file_id) {
            Some(v) => v,
            None => continue
        };

        for artist in song_data["participants"]["artist"].as_array().unwrap().iter() {
            let artist: Artist = serde_json::from_value((artist).clone()).unwrap();

            match artist_count.get_mut(&artist.id) {
                Some(v) => (*v).plays += 1,
                None => {
                    artist_count.insert(
                        artist.id,
                        ArtistCount {name: artist.name, plays: 1}
                    );
                }
            };
        }
    }

    let mut all_artists: Vec<(String, String, u64)> = Vec::new();
    for (id, artist) in artist_count {
        all_artists.push((id, artist.name, artist.plays));
    }

    all_artists.sort_by(|a, b| { b.2.cmp(&a.2)});

    return Ok(Json(serde_json::to_value(all_artists).unwrap()));
}
