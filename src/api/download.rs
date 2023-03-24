use axum::extract::Query;
use axum::response::IntoResponse;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DownloadQuery {
    file: String,
}

pub async fn download(Query(q): Query<DownloadQuery>) -> impl IntoResponse {}
