use crate::data::AppData;
use axum::extract::State;
use axum::response::IntoResponse;
use std::sync::{Arc, Mutex};

pub async fn announcement(State(data): State<Arc<Mutex<AppData>>>) -> impl IntoResponse {
    let a = &data.lock().unwrap().announcement;
    serde_json::to_string(a).unwrap()
}
