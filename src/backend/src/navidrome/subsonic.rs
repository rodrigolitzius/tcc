use std::str::FromStr;

use serde::de::Error;

use crate::{navidrome::*};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawArtistGetArtist {
    name: String,
    album_count: u64,
    album: Vec<AlbumGetArtist>,
    music_brainz_id: String
}

impl NavidromeSubsonicSession {
    // TODO: Actually test if this fails if the login request has invalid credentials
    pub async fn new(login_request: LoginRequest) -> Result<Self, NavidromeSessionError> {
        let salt: String = rand::rng()
            .sample_iter(Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();

        let hash = format!("{:x}", md5::compute(format!("{}{}", login_request.password, salt)));

        let mut default_params: Vec<(String, String)> = Vec::new();
        default_params.push(("u".to_string(), login_request.username));
        default_params.push(("s".to_string(), salt.clone()));
        default_params.push(("t".to_string(), hash.clone()));
        default_params.push(("c".to_string(), crate::APP_NAME.to_string()));
        default_params.push(("v".to_string(), "1.8.0".to_string()));
        default_params.push(("f".to_string(), "json".to_string()));

        let url = format!("{}/rest/ping", login_request.url);

        let client = match Client::builder().tls_danger_accept_invalid_certs(true).build() {
            Ok(v) => v,
            Err(e) => {
                return Err(NavidromeSessionError::Reqwest(e));
            }
        };

        let response = client
            .request(Method::GET, url)
            .query(&default_params)
            .send()
            .await;

        let _response = validate_reqwest_response(response)?;

        let result = Self {
            default_params: default_params,
            url: login_request.url,
            client: client,
            salt: salt,
            token: hash
        };

        return Ok(result)
    }

    pub async fn get_artist(&self, id: String) -> Result<ArtistGetArtist, NavidromeSessionError> {
        let url = format!("{}/rest/getArtist?id={}", self.url, id);

        let mut client_queries: Vec<(String, String)> = Vec::new();
        client_queries.push(("id".to_string(), id));

        let response = self.client
            .get(url)
            .query(&self.default_params)
            .query(&client_queries)
            .send()
            .await;

        let response = validate_reqwest_response(response)?;

        let artist: serde_json::Value = response.json().await.unwrap();
        let artist = artist
            .get("subsonic-response").ok_or(
                NavidromeSessionError::ParseJson(
                    serde_json::Error::missing_field("The result does not contain the \"subsonic-response\" field")
                )
            )?
            .get("artist").ok_or(
                NavidromeSessionError::ParseJson(
                    serde_json::Error::missing_field("The result does not contain the \"artist\" field")
                )
            )?;

        let artist: RawArtistGetArtist = serde_json::from_value::<RawArtistGetArtist>(artist.clone())?;

        let artist = ArtistGetArtist {
            name: artist.name,
            album_count: artist.album_count,
            album: artist.album,
            music_brainz_id: match Uuid::from_str(&artist.music_brainz_id) {
                Ok(v) => Some(v),
                Err(_) => None,
            }
        };

        return Ok(artist);
    }
}
