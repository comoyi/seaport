use crate::config::CONFIG;
use env_logger::Builder;
use log::LevelFilter;
use std::str::FromStr;

pub fn init_log() {
    let l = LevelFilter::from_str(&CONFIG.log_level).unwrap_or(LevelFilter::Off);
    // println!("log filter: {}", l);

    let mut builder = Builder::from_default_env();
    builder.filter_level(LevelFilter::Warn);
    builder.filter_module("seaport", l);
    builder.init();
}
