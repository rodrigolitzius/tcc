use std::collections::HashMap;


use crate::{
    db_analyser::{Scrobble},
    navidrome::{Session}
};

pub async fn build_user_track_hashmap(scrobbles: &Vec<Scrobble>, session: &mut Session) -> HashMap<String, serde_json::Value> {
    let mut result = HashMap::new();

    for scrobble in scrobbles {
        if scrobble.user_id == session.user_id {
            let song = session.song(&scrobble.media_file_id).await;
            if let Err(_) = song {continue;}

            result.insert(scrobble.media_file_id.clone(), song.unwrap());
        }
    }

    return result;
}
