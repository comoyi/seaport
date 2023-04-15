use crate::util;
use config::builder::DefaultState;
use config::ConfigBuilder;
use lazy_static::lazy_static;
use log::debug;
use serde::{Deserialize, Serialize};
use std::path::Path;

lazy_static! {
    pub static ref CONFIG: Config = init_config();
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub log_level: String,
    pub port: u16,
    pub dir: String,
    pub title: String,
    pub announcement: String,
    pub data_nodes: Vec<DataNode>,
}

impl Config {
    pub fn print_config(&self) {
        debug!("data_nodes: {:?}", CONFIG.data_nodes);
        for (i, x) in CONFIG.data_nodes.iter().enumerate() {
            debug!(
                "data_node[{}]: name: {:20}, addr: {:30}",
                i,
                x.name,
                x.get_address_string(),
            );
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataNode {
    pub name: String,
    pub address: Address,
}

impl DataNode {
    pub fn get_address_string(&self) -> String {
        self.address.to_address_string()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    pub protocol: String,
    pub host: String,
    pub port: u16,
}
impl Address {
    pub fn to_address_string(&self) -> String {
        format!("{}://{}:{}", self.protocol, self.host, self.port)
    }
}

pub fn init_config() -> Config {
    let mut b = config::Config::builder();

    b = set_default(b);

    let exe_dir_r = util::path::get_exe_dir();
    let exe_dir = match exe_dir_r {
        Ok(exe_dir) => exe_dir,
        Err(e) => {
            panic!("get exe_dir failed, err: {}", e);
        }
    };
    let config_path = Path::new(&exe_dir).join("config.toml");
    let cps = vec![config_path];
    for cp_str in cps {
        let cp = Path::new(&cp_str);
        if cp.exists() {
            // println!("Add config file: {:?}", cp);
            b = b.add_source(config::File::from(cp))
        }
    }

    let c = b.build().unwrap();
    let conf_result = c.try_deserialize::<Config>();
    let conf = match conf_result {
        Ok(c) => c,
        Err(e) => {
            println!("load config failed: {}", e);
            panic!("load config failed: {}", e);
        }
    };
    conf
}

fn set_default(b: ConfigBuilder<DefaultState>) -> ConfigBuilder<DefaultState> {
    b.set_default("log_level", "TRACE")
        .unwrap()
        .set_default("port", 8080)
        .unwrap()
        .set_default("dir", "")
        .unwrap()
        .set_default("title", "")
        .unwrap()
        .set_default("announcement", "")
        .unwrap()
}
