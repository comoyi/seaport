use crate::data::AppData;
use axum::extract::State;
use axum::http::header::CONTENT_TYPE;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Router, Server};
use log::info;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

#[tokio::main]
pub async fn start(data: Arc<Mutex<AppData>>) {
    info!("start api");
    let a = Router::new()
        .route("/", get(index))
        .route("/files", get(files))
        .route("/sync", get(sync))
        .route("/announcement", get(announcement))
        .with_state(data);
    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    Server::bind(&addr)
        .serve(a.into_make_service())
        .await
        .unwrap();
}

async fn index() -> &'static str {
    "api index"
}

async fn files(State(data): State<Arc<Mutex<AppData>>>) -> impl IntoResponse {
    let d_guard = data.lock().unwrap();
    let sfi = &d_guard.server_file_info;
    let s = serde_json::to_string(sfi).unwrap();
    drop(d_guard);
    Response::builder()
        .header(CONTENT_TYPE, "application/json; charset=UTF-8")
        .body(s)
        .unwrap()
}

async fn sync() -> impl IntoResponse {
    "sync"
}

async fn announcement() -> impl IntoResponse {
    "announcement"
}
