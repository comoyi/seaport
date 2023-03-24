use crate::api::announcement::announcement;
use crate::api::files::files;
use crate::api::index::index;
use crate::api::sync::sync;
use crate::data::AppData;
use axum::routing::get;
use axum::Router;
use std::sync::{Arc, Mutex};

pub fn create_router(data: Arc<Mutex<AppData>>) -> Router {
    let r = Router::new()
        .route("/", get(index))
        .route("/files", get(files))
        .route("/sync", get(sync))
        .route("/announcement", get(announcement))
        .with_state(data);
    r
}
