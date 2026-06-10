use crate::mbz::*;

impl MbzSession {
    pub fn new(token: Uuid) -> Self {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            "Authorization",
            HeaderValue::from_str(format!("Token {}", token).as_str()).expect("Token is invalid")
        );

        let client = ClientBuilder::new()
            .default_headers(default_headers)
            .build().expect("Failed to build MBZ client");

        let result = Self {
            token: token,
            client: client
        };

        return result;
    }

    pub async fn get_artist(&self, id: Uuid) -> Result<MbzArtist, MbzError>{
        let response = self.client
            .get(format!("{MBZ_URL}/metadata/artist?artist_mbids={}&inc=artist", id))
            .send()
            .await?
            .error_for_status()
            .unwrap();

        let json = response.json::<serde_json::Value>().await?;

        let json = match json.as_array() {
            Some(v) => v,
            None => {
                return Err(MbzError::ParseJson(serde_json::Error::custom("Mbz response is not an array")))
            }
        };

        let json = match json.iter().nth(0) {
            Some(v) => v,
            None => {
                return Err(MbzError::ParseJson(serde_json::Error::custom("Mbz response is empty")))
            }
        };

        return Ok(
            serde_json::from_value::<MbzArtist>(json.clone())?
        );
    }
}
