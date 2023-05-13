use crate::data::AppData;
use axum::extract::State;
use axum::http::header::CONTENT_TYPE;
use axum::response::{IntoResponse, Response};
use std::sync::{Arc, Mutex};

pub async fn files(State(data): State<Arc<Mutex<AppData>>>) -> impl IntoResponse {
    let d_guard = data.lock().unwrap();
    let sfi = &d_guard.server_file_info;
    let s = serde_json::to_string(sfi).unwrap();
    drop(d_guard);
    Response::builder()
        .header(CONTENT_TYPE, "application/json; charset=UTF-8")
        .body(s)
        .unwrap()
}
