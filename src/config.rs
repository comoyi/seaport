use log::debug;
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct Config {
    log_level: String,
    port: i64,
}

pub fn init_config() {
    let c = config::Config::builder()
        // .add_source(config::File::from(Path::new("config.toml")))
        .add_source(config::File::from(Path::new("config/config.toml")))
        .build()
        .unwrap();
    let conf = c.try_deserialize::<Config>().unwrap();
    // println!("{:?}", conf);
}
