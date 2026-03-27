mod db_analyser;
mod navidrome;
mod util;

mod api;

use crate::{
    db_analyser::{Scrobble, build_scrobble},
    api::{ApiState, handlers::{login, scrobbles_to_text}}
};

use std::{sync::Arc};
use rusqlite::{Connection, OpenFlags};
use tower_http::cors::{Any, CorsLayer};
use axum::{Router, routing::{get, post}};

async fn start_backend(state: ApiState) {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // TODO: Add a route to the logged in user's recently played tracks
    let app = Router::new()
        .route("/dev/recent", get(scrobbles_to_text))
        .route("/login", post(login))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.expect("Failed to bind server");
    axum::serve(listener, app).await.expect("Failed to serve server");
}

#[tokio::main]
async fn main() {
    // Opening database
    let db = Connection::open_with_flags(
        "../../navidrome.db",
        OpenFlags::SQLITE_OPEN_READ_ONLY
    ).expect("Failed to open database");

    // Getting scrobbles
    let mut stmt = db.prepare("SELECT * FROM scrobbles ORDER BY submission_time DESC").expect("Couldn't prepare SQL query");
    let rows = stmt.query_and_then([], |row| build_scrobble(row)).expect("query_and_then failed");

    let mut scrobbles: Vec<Scrobble> = Vec::new();

    for scrobble in rows {
        if let Ok(v) = scrobble {
            scrobbles.push(v);
        }
    }

    let state = ApiState {
        scrobbles: Arc::new(scrobbles), ..Default::default()
    };

    start_backend(state).await;
}
