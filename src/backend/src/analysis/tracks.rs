use std::collections::HashMap;

use serde::Serialize;

use crate::{
    navidrome::{Scrobble, native::SongData}
};

#[derive(Serialize, Clone)]
pub struct TrackStat {
    pub name: String,
    pub artist: String,
    pub id: String,
    pub plays: u64,
    pub played_hours: f64
}

impl TrackStat {
    pub fn group(
        scrobbles: Vec<&Scrobble>,
        track_hashmap: &HashMap<String, SongData>
    ) -> HashMap<String, TrackStat> {
        let mut track_stat: HashMap<String, TrackStat> = HashMap::new();

        for scrobble in scrobbles {
            let song_data = match track_hashmap.get(&scrobble.media_file_id) {
                Some(v) => v,
                None => continue
            };

            let duration_hour = song_data.duration / (60.0*60.0);

            match track_stat.get_mut(&song_data.id.clone()) {
                Some(v) => {
                    (*v).plays += 1;
                    (*v).played_hours += duration_hour
                },
                None => {
                    track_stat.insert(
                        song_data.id.clone(),
                        TrackStat {
                            name: song_data.title.clone(),
                            artist: song_data.artist.clone(),
                            id: song_data.id.clone(),
                            plays: 1,
                            played_hours: duration_hour
                        }
                    );
                }
            };
        }

        return track_stat;
    }
}
