use crate::config::{Banner, CONFIG};
use axum::http::header::CONTENT_TYPE;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Serialize)]
struct BannerVo {
    banners: Vec<Banner>,
}

impl BannerVo {
    fn new(banners: Vec<Banner>) -> Self {
        Self { banners }
    }
}

pub async fn banner() -> impl IntoResponse {
    let mut banners = vec![];

    for x in &CONFIG.banners {
        let banner = Banner::new(&x.image_url, &x.description);
        banners.push(banner);
    }

    let vo = BannerVo::new(banners);

    let s = serde_json::to_string(&vo).unwrap();
    Response::builder()
        .header(CONTENT_TYPE, "application/json; charset=UTF-8")
        .body(s)
        .unwrap()
}
