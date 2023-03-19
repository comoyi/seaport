use crate::data::AppData;
use axum::extract::{Query, State};
use axum::http::header::CONTENT_TYPE;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Router, Server};
use log::info;
use serde::Deserialize;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::{fs, path};

#[tokio::main]
pub async fn start(data: Arc<Mutex<AppData>>) {
    info!("start api");
    let a = Router::new()
        .route("/", get(index))
        .route("/files", get(files))
        .route("/sync", get(sync))
        .route("/announcement", get(announcement))
        .with_state(data);
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
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

#[derive(Deserialize)]
struct SyncQuery {
    file: String,
}

async fn sync(Query(q): Query<SyncQuery>) -> impl IntoResponse {
    let rp = q.file;

    let base_path = "/tmp/a";
    let absolute_path = path::Path::new(base_path).join(rp);
    if absolute_path.exists() {
        if absolute_path.starts_with(base_path) {
            if absolute_path.is_symlink() {
                let r = absolute_path.read_link();
                match r {
                    Ok(pb) => {
                        let ldp = pb.to_str();
                        match ldp {
                            None => {
                                return vec![];
                            }
                            Some(link_dst_path) => {
                                return Vec::from(link_dst_path);
                            }
                        }
                    }
                    Err(_) => {
                        return vec![];
                    }
                }
            } else if absolute_path.is_dir() {
                return vec![];
            } else if absolute_path.is_file() {
                return fs::read(absolute_path).unwrap();
            } else {
                return vec![];
            }
        }
    }
    vec![]
}

async fn announcement(State(data): State<Arc<Mutex<AppData>>>) -> impl IntoResponse {
    let a = &data.lock().unwrap().announcement;
    serde_json::to_string(a).unwrap()
}
