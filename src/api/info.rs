use crate::config::{Address, DataNode, CONFIG};
use axum::http::header::CONTENT_TYPE;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Serialize, Default)]
struct InfoVo {
    data_nodes: Vec<DataNode>,
}

pub async fn info() -> impl IntoResponse {
    let mut vo = InfoVo::default();
    let mut data_nodes = vec![];
    for x in &CONFIG.data_nodes {
        let data_node = DataNode {
            name: x.name.to_string(),
            address: Address {
                protocol: x.address.protocol.to_string(),
                host: x.address.host.to_string(),
                port: x.address.port,
            },
        };
        data_nodes.push(data_node);
    }
    vo.data_nodes = data_nodes;

    let s = serde_json::to_string(&vo).unwrap();
    Response::builder()
        .header(CONTENT_TYPE, "application/json; charset=UTF-8")
        .body(s)
        .unwrap()
}
