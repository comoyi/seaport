use crate::{api, gui, scanner};
use log::info;

pub fn start() {
    info!("app start");

    scanner::start();
    api::start();
    gui::start();
}
