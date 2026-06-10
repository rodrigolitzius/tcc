mod navidrome;
mod handlers;
mod api;
mod mbz;
mod sqlite;
mod storage;

use std::str::FromStr;
use axum::{Router, routing::{get, post}};
use tower_http::cors::{Any, CorsLayer};
use rusqlite::{Connection, OpenFlags};
use uuid::Uuid;
use clap::{Parser};

use crate::{
    handlers::{login::*, recent::*, relay::*, artists::*},
    navidrome::{Scrobble, build_scrobble},
    api::{ApiState}
};

const APP_NAME: &'static str = "Navalyze";

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    db_location: String,

    #[arg(short, long)]
    mbz_token: Option<Uuid>
}

async fn start_backend(state: ApiState) {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/recent", get(recent))
        .route("/relay/{*tail}", get(relay))
        .route("/most-played/artists", get(most_played_artists))
        .route("/login", post(login))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.expect("Failed to bind server");
    axum::serve(listener, app).await.expect("Failed to serve server");
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Opening database
    let navidrome_db = Connection::open_with_flags(
        args.db_location,
        OpenFlags::SQLITE_OPEN_READ_ONLY
    ).expect("Failed to open navidrome's database");

    // Getting scrobbles
    let mut stmt = navidrome_db.prepare("SELECT * FROM scrobbles ORDER BY submission_time DESC").expect("Couldn't prepare SQL query");
    let rows = stmt.query_and_then([], |row| build_scrobble(row)).expect("query_and_then failed");

    let mut scrobbles: Vec<Scrobble> = Vec::new();

    for scrobble in rows {
        if let Ok(v) = scrobble {
            scrobbles.push(v);
        }
    }

    let mbz_session = match args.mbz_token {
        Some(v) => Some(mbz::MbzSession::new(v)),
        None => None
    };

    let state = ApiState::new(scrobbles, mbz_session).expect("Failed to initialize API state");

    // // This is the code i used for testing mbz integration
    // let artist = state.storage.get_artist(
    //     1, uuid::Uuid::from_str("a8c6ef99-6a24-4c68-83e5-35aa1eca32fa").unwrap()
    // ).await.unwrap();

    // match artist {
    //     Some(v) => println!("{:?}", v),
    //     None => println!("No artists")
    // }

    start_backend(state).await;
}
