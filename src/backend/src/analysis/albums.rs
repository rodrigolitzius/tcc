use std::{collections::HashMap};

use serde::Serialize;

use crate::{
    navidrome::{Scrobble, native::SongData}
};

#[derive(Serialize, Clone)]
pub struct AlbumStat {
    pub name: String,
    pub artist: String,
    pub id: String,
    pub plays: u64,
    pub played_hours: f64
}

impl AlbumStat {
    pub fn group(
        scrobbles: Vec<&Scrobble>,
        track_hashmap: &HashMap<String, SongData>,
        include: Option<Vec<String>>
    ) -> HashMap<String, AlbumStat> {
        let mut album_stat: HashMap<String, AlbumStat> = HashMap::new();

        for scrobble in scrobbles.iter() {
            let song_data = match track_hashmap.get(&scrobble.media_file_id) {
                Some(v) => v,
                None => continue
            };

            match &include {
                Some(v) => {
                    if !v.contains(&song_data.album_id.clone()) {continue}
                }
                None => {}
            }

            let duration_hour = song_data.duration / (60.0*60.0);

            match album_stat.get_mut(&song_data.album_id.clone()) {
                Some(v) => {
                    (*v).plays += 1;
                    (*v).played_hours += duration_hour
                },
                None => {
                    album_stat.insert(
                        song_data.album_id.clone(),
                        AlbumStat {
                            name: song_data.album.clone(),
                            artist: song_data.artist.clone(),
                            id: song_data.album_id.clone(),
                            plays: 1,
                            played_hours: duration_hour
                        }
                    );
                }
            };
        }

        return album_stat;
    }
}
