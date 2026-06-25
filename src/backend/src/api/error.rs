use axum::{Json, response::IntoResponse};
use reqwest::StatusCode;
use serde_json::json;

use crate::storage::StorageError;

pub enum ApiError {
    // Login
    NavidromeUnreachable(String),

    // Other
    Internal(String),
    Unauthorized(String),
    BadRequest(String),
    DatabaseError(String)
}

impl From<StorageError> for ApiError {
    fn from(value: StorageError) -> Self {
        return match value {
            StorageError::Reqwest(e) => ApiError::Internal(
                format!("Could not connect to MusicBrainz: {}", e.to_string())
            ),

            StorageError::ParseJson(e) => ApiError::Internal(
                format!("Could not parse MusicBrainz response: {}", e.to_string())
            ),

            StorageError::Rusqlite(e) => ApiError::Internal(
                format!("Internal database error: {}", e.to_string())
            )
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (code, data) = match self {
            ApiError::NavidromeUnreachable(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg)
        };

        return (
            code,
            Json(json!({"error": data}))
        ).into_response()
    }
}
