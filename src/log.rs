use env_logger::Builder;
use log::LevelFilter;

pub fn init_log() {
    let mut builder = Builder::from_default_env();
    builder.filter_level(LevelFilter::Warn);
    builder.filter_module("seaport", LevelFilter::Trace);
    builder.init();
}
