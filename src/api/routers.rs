use crate::api::announcement::announcement;
use crate::api::download::download;
use crate::api::files::files;
use crate::api::index::index;
use crate::data::AppData;
use axum::routing::get;
use axum::Router;
use std::sync::{Arc, Mutex};

pub fn create_router(data: Arc<Mutex<AppData>>) -> Router {
    let r = Router::new()
        .route("/", get(index))
        .route("/files", get(files))
        .route("/api/experimental/download", get(download))
        .route("/sync", get(download))
        .route("/announcement", get(announcement))
        .with_state(data);
    r
}
