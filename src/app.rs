use crate::{api, gui, scanner};
use log::info;

pub fn start() {
    info!("start app");

    scanner::start();
    api::start();
    gui::start();
}
