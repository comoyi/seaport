use crate::config::CONFIG;
use axum::extract::Query;
use axum::response::IntoResponse;
use serde::Deserialize;
use std::{fs, path};

#[derive(Deserialize)]
pub struct SyncQuery {
    file: String,
}

pub async fn sync(Query(q): Query<SyncQuery>) -> impl IntoResponse {
    let rp = q.file;

    let base_path = &CONFIG.dir;
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
