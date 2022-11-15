use crate::cohost::CohostApi;
use axum::Json;
use hyper::StatusCode;
use serde_json::{json, Value};

pub struct State {
    pub api: CohostApi,
    pub domain: String,
}

pub async fn json_error(err: anyhow::Error) -> (StatusCode, Json<Value>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({
            "error": err.to_string(),
        })),
    )
}
