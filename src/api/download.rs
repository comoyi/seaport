use crate::config::CONFIG;
use axum::body::StreamBody;
use axum::extract::{Path as QueryPath, Query};
use axum::http::header::{CONTENT_DISPOSITION, CONTENT_LENGTH, CONTENT_RANGE};
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use log::{debug, warn};
use serde::Deserialize;
use std::io::SeekFrom;
use std::path::Path;
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tokio_util::io::ReaderStream;

#[derive(Deserialize)]
pub struct DownloadQuery {
    file: String,
}

pub async fn download(
    QueryPath(relative_file_path): QueryPath<String>,
    headers: HeaderMap,
) -> Response {
    let range_o = headers.get(header::RANGE);
    let mut range_option = None;
    if range_o.is_some() {
        let range = range_o
            .and_then(|v| v.to_str().ok())
            .map(|v| v.to_string())
            .unwrap();
        range_option = Some(range);
    }
    do_download(relative_file_path, range_option).await
}
pub async fn download_legacy(Query(q): Query<DownloadQuery>) -> Response {
    do_download(q.file, None).await
}

pub async fn do_download(rp: String, range: Option<String>) -> Response {
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
        "downloading file, rp: {}, rel_path: {:?}, abs_path: {:?}, name: {}",
        rp, rel_path, abs_path, filename
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
        let mut f = match tokio::fs::File::open(&abs_path).await {
            Ok(file) => file,
            Err(_) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, "err: 1001").into_response();
            }
        };
        let file_len = f.metadata().await.unwrap().len();
        let mut range_start = 0;
        let mut range_end = 0;
        let mut range_len = file_len;
        if let Some(ra) = &range {
            let range_r = parse_range(ra, file_len);
            if range_r.is_err() {
                return (StatusCode::RANGE_NOT_SATISFIABLE, "err: range invalid").into_response();
            }
            let rg = range_r.ok().unwrap();
            range_start = rg.start;
            range_end = rg.end;
        }
        if range_end > 0 {
            range_len = range_end - range_start + 1;
            if range_start + range_len > file_len {
                return (StatusCode::RANGE_NOT_SATISFIABLE, "err: range out of range")
                    .into_response();
            }
        }
        debug!(
            "file, rp: {}, rel_path: {:?}, abs_path: {:?}, name: {}, range: {:?}, range_start: {}, range_end: {}, range_len: {}",
            rp, rel_path, abs_path, filename, range, range_start, range_end, range_len
        );
        let _ = f.seek(SeekFrom::Start(range_start)).await;
        let file_chunk = f.take(range_len);
        let st = ReaderStream::with_capacity(file_chunk, 1024 * 1024 * 10);
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
        header.insert(CONTENT_LENGTH, HeaderValue::from(range_len));
        if range.is_some() {
            let hv_content_range_r = HeaderValue::from_str(
                format!("bytes {}-{}/{}", range_start, range_end, file_len).as_str(),
            );
            let hv_content_range = match hv_content_range_r {
                Ok(v) => v,
                Err(_) => {
                    return (StatusCode::INTERNAL_SERVER_ERROR, "err: 1003").into_response();
                }
            };
            header.insert(CONTENT_RANGE, hv_content_range);
        }
        if range.is_some() {
            return (StatusCode::PARTIAL_CONTENT, header, body).into_response();
        }
        return (header, body).into_response();
    } else {
        return (StatusCode::BAD_REQUEST, "unexpect type").into_response();
    }
}

struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Self {
            start: start,
            end: end,
        }
    }
}

struct RangeError;

fn parse_range(range: &str, file_len: u64) -> Result<Range, RangeError> {
    let prefix = "bytes=";
    if !range.starts_with(prefix) {
        return Err(RangeError);
    }
    let range_sub_o = range.strip_prefix(prefix);
    if range_sub_o.is_none() {
        return Err(RangeError);
    }
    let range_sub = range_sub_o.unwrap();
    let s: Vec<_> = range_sub.split("-").collect();
    if s.len() != 2 {
        return Err(RangeError);
    }
    let start;
    let end;
    if s[0] == "" {
        start = 0;
    } else {
        start = s[0].parse().unwrap();
    }
    if s[1] == "" {
        end = file_len - 1;
    } else {
        end = s[1].parse().unwrap();
    }
    if start > end {
        return Err(RangeError);
    }
    Ok(Range::new(start, end))
}
