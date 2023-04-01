use crate::api::announcement::announcement;
use crate::api::download::download;
use crate::api::files::files;
use crate::api::index::index;
use crate::api::info::info;
use crate::data::AppData;
use axum::routing::get;
use axum::Router;
use std::sync::{Arc, Mutex};

pub fn create_router(data: Arc<Mutex<AppData>>) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/files", get(files))
        .route("/sync", get(download))
        .route("/api/info", get(info))
        .route("/api/download", get(download))
        .route("/api/announcement", get(announcement))
        .with_state(data)
}
