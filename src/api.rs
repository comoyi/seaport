use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Router, Server};
use log::info;
use std::net::SocketAddr;

#[tokio::main]
pub async fn start() {
    info!("start api");
    let a = Router::new()
        .route("/", get(index))
        .route("/files", get(files))
        .route("/sync", get(sync))
        .route("/announcement", get(announcement));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    Server::bind(&addr)
        .serve(a.into_make_service())
        .await
        .unwrap();
}

async fn index() -> &'static str {
    "api index"
}

async fn files() -> impl IntoResponse {
    "{}"
}

async fn sync() -> impl IntoResponse {
    "sync"
}

async fn announcement() -> impl IntoResponse {
    "announcement"
}
