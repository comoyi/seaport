use crate::config::CONFIG;
use axum::body::StreamBody;
use axum::extract::Query;
use axum::http::header::CONTENT_DISPOSITION;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use log::{debug, warn};
use serde::Deserialize;
use std::path::Path;
use tokio_util::io::ReaderStream;

#[derive(Deserialize)]
pub struct DownloadQuery {
    file: String,
}

pub async fn download(Query(q): Query<DownloadQuery>) -> Response {
    let rp = q.file;

    let base_path = &CONFIG.dir;
    let abs_path = Path::new(base_path).join(&rp);
    if !abs_path.starts_with(base_path) {
        return (StatusCode::NOT_FOUND, "err: 1").into_response();
    }
    if !abs_path.exists() {
        return (StatusCode::NOT_FOUND, "err: 2").into_response();
    }

    let rel_path_r = abs_path.strip_prefix(base_path);
    let rel_path = match rel_path_r {
        Ok(v) => v,
        Err(_) => {
            return (StatusCode::NOT_FOUND, "err: 3").into_response();
        }
    };

    let filename;
    let filename_opt = abs_path.file_name();
    match filename_opt {
        None => {
            return (StatusCode::NOT_FOUND, "err: 4").into_response();
        }
        Some(v) => {
            let s_o = v.to_str();
            match s_o {
                None => {
                    return (StatusCode::NOT_FOUND, "err: 5").into_response();
                }
                Some(v) => filename = String::from(v),
            }
        }
    }
    debug!(
        "downloading file,rp: {}, abs_path: {:?}, name: {}",
        rp, rel_path, filename
    );

    if abs_path.is_symlink() {
        let sym_dst = match abs_path.read_link() {
            Ok(pb) => {
                let o = pb.to_str();
                match o {
                    None => {
                        warn!("link is None, abs_path: {:?}", abs_path);
                        return (StatusCode::INTERNAL_SERVER_ERROR, "read symlink error")
                            .into_response();
                    }
                    Some(v) => v.to_string(),
                }
            }
            Err(e) => {
                warn!("read link failed, abs_path: {:?}, err: {}", abs_path, e);
                return (StatusCode::INTERNAL_SERVER_ERROR, "read symlink error").into_response();
            }
        };
        return (StatusCode::OK, sym_dst).into_response();
    } else if abs_path.is_dir() {
        return (StatusCode::BAD_REQUEST, "is dir").into_response();
    } else if abs_path.is_file() {
        let f = match tokio::fs::File::open(abs_path).await {
            Ok(file) => file,
            Err(_) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, "err: 1001").into_response();
            }
        };
        let st = ReaderStream::with_capacity(f, 1024 * 1024 * 10);
        let body = StreamBody::new(st);
        let hv_filename_r =
            HeaderValue::from_str(format!("attachment; filename=\"{}\"", filename).as_str());
        let hv_filename = match hv_filename_r {
            Ok(v) => v,
            Err(_) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, "err: 1002").into_response();
            }
        };
        let mut header = HeaderMap::new();
        header.insert(CONTENT_DISPOSITION, hv_filename);
        return (header, body).into_response();
    } else {
        return (StatusCode::BAD_REQUEST, "unexpect type").into_response();
    }
}
