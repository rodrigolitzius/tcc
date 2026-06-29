use std::{collections::HashMap};

use serde::Serialize;

use crate::{
    navidrome::{Scrobble, native::SongData}
};

#[derive(Serialize, Clone)]
pub struct ArtistStat {
    pub name: String,
    pub id: String,
    pub plays: u64,
    pub played_hours: f64
}

impl ArtistStat {
    pub fn group(
        scrobbles: Vec<&Scrobble>,
        track_hashmap: &HashMap<String, SongData>
    ) -> HashMap<String, ArtistStat> {
        let mut artist_stat: HashMap<String, ArtistStat> = HashMap::new();

        for scrobble in scrobbles {
            let song_data = match track_hashmap.get(&scrobble.media_file_id) {
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

        return artist_stat;
    }
}
