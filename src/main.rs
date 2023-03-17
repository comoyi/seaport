mod api;
mod gui;
mod log;

fn main() {
    log::init_log();
    api::start();
    gui::start();
}
