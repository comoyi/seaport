use lazy_static::lazy_static;
use serde::Deserialize;
use std::path::Path;

lazy_static! {
    pub static ref CONFIG: Config = init_config();
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub log_level: String,
    pub port: u16,
    pub dir: String,
}

pub fn init_config() -> Config {
    let mut b = config::Config::builder();
    let cp1 = Path::new("config.toml");
    let cp2 = Path::new("config/config.toml");
    if cp1.exists() {
        // println!("Add config file: {:?}", cp1);
        b = b.add_source(config::File::from(cp1))
    }
    if cp2.exists() {
        // println!("Add config file: {:?}", cp2);
        b = b.add_source(config::File::from(cp2));
    }
    let c = b.build().unwrap();
    let conf = c.try_deserialize::<Config>().unwrap();
    // println!("{:?}", conf);
    conf
}
