mod api;
mod app;
mod config;
mod data;
mod gui;
mod log;
mod scanner;

fn main() {
    config::init_config();
    log::init_log();
    app::start();
}
