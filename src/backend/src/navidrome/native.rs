use crate::{navidrome::*};

impl NavidromeNativeSession {
    pub async fn new(login_request: LoginRequest) -> Result<Self, NavidromeSessionError> {
        let client = match Client::builder().tls_danger_accept_invalid_certs(true).build() {
            Ok(v) => v,
            Err(e) => {
                return Err(NavidromeSessionError::Reqwest(e));
            }
        };

        let mut body = HashMap::new();
        body.insert("username", login_request.username);
        body.insert("password", login_request.password);

        let response = client
            .post(format!("{}/auth/login", login_request.url))
            .json(&body)
            .send()
            .await;

        let response = validate_reqwest_response(response)?;

        let login_response: LoginResponse = response
            .json()
            .await
            .expect("Required fields are missing from Navidrome's response");

        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            "x-nd-authorization",
            HeaderValue::from_str(format!("Bearer {}", login_response.token).as_str()).expect("Navidrome returned an invalid token")
        );

        let client = Client::builder()
            .tls_danger_accept_invalid_certs(true)
            .default_headers(default_headers)
            .build()
            .unwrap();

        return Ok(Self {
            url: login_request.url.to_string(),
            user_id: login_response.id,
            client: client,
            token: login_response.token,
        });
    }

    pub async fn build_track_hashmap(&self, scrobbles: &Vec<Scrobble>) -> Result<HashMap<String, SongData>, NavidromeSessionError> {
        let url = format!("{}/api/song/", self.url);

        let mut queries: Vec<(String, String)> = Vec::new();
        queries.push(("_start".into(), "0".into()));
        queries.push(("_end".into(), "-1".into()));

        let response = self.client
            .get(&url)
            .send()
            .await;

        let response = validate_reqwest_response(response)?;

        let response = response.json::<Vec<SongData>>().await.map_err(|e|
            NavidromeSessionError::Reqwest(e)
        )?;

        let media_file_ids: Vec<&String> = scrobbles.iter().map(|s| {&s.media_file_id}).collect();

        let mut result = HashMap::new();

        for song_data in response {
            if media_file_ids.contains(&&song_data.id) {
                result.insert(song_data.id.clone(), song_data);
            }
        }

        return Ok(result);
    }
}
