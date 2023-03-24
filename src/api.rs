mod announcement;
mod download;
mod files;
mod index;
mod routers;
mod sync;

use crate::api::routers::create_router;
use crate::config::CONFIG;
use crate::data::AppData;
use axum::Server;
use log::info;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

#[tokio::main]
pub async fn start(data: Arc<Mutex<AppData>>) {
    info!("start api");
    let a = create_router(data);
    let addr = SocketAddr::from(([0, 0, 0, 0], CONFIG.port));
    Server::bind(&addr)
        .serve(a.into_make_service())
        .await
        .unwrap();
}
