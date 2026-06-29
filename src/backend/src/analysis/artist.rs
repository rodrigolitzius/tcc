use std::collections::HashMap;

use crate::{
    analysis::albums::AlbumStat, navidrome::Artist,
    mbz::MbzArtist
};

pub struct ArtistStat {
    pub name: String,
    pub artist_type: Option<String>,
    pub gender: Option<String>,
    pub albums: Vec<ArtistAlbumStat>
}

pub struct ArtistAlbumStat {
    pub id: String,
    pub name: String,
    pub played_hours: f64,
    pub plays: u64,
    pub year: u64
}

impl ArtistStat {
    pub fn get(
        artist: Artist,
        album_stat: HashMap<String, AlbumStat>,
        mbz_artist: Option<MbzArtist>
    ) -> ArtistStat {
        let mut albums: Vec<ArtistAlbumStat> = Vec::new();

        for album in &artist.albums {
            let stat = match album_stat.get(&album.id) {
                Some(v) => v,
                None => {continue;}
            };

            albums.push(ArtistAlbumStat {
                id: album.id.clone(),
                name: stat.name.clone(),
                played_hours: stat.played_hours,
                plays: stat.plays,
                year: album.year,
            });
        }

        let mut artist_type = Option::None;
        let mut gender = Option::None;

        match mbz_artist {
            Some(v) => {
                artist_type = Some(v.artist_type);
                gender = v.gender;
            },

            None => {}
        }

        return ArtistStat {
            name: artist.name,
            artist_type: artist_type,
            gender: gender,
            albums
        };
    }
}
