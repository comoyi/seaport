use crate::api::announcement::announcement;
use crate::api::banner::banner;
use crate::api::download::{download, download_legacy, download_path};
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
        .route("/sync", get(download_legacy))
        .route("/announcement", get(announcement))
        .route("/api/v1/info", get(info))
        .route("/api/v1/files", get(files))
        .route("/api/v1/download", get(download))
        .route("/api/v1/download/*relative_file_path", get(download_path))
        .route("/api/v1/announcement", get(announcement))
        .route("/api/v1/banner", get(banner))
        .with_state(data)
}
