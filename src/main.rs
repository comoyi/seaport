use crate::config::CONFIG;

mod api;
mod app;
mod config;
mod data;
mod error;
mod gui;
mod log;
mod scanner;
mod util;
mod version;

fn main() {
    // for init config
    let _ = &CONFIG.log_level;
    // config::init_config();

    log::init_log();

    CONFIG.print_config();

    app::start();
}
