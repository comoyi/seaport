use crate::config::CONFIG;
use crate::data::AppData;
use crate::scanner::Scanner;
use crate::{api, gui};
use log::info;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn start() {
    info!("start app");

    let app_data = AppData::new();
    let d = Arc::new(Mutex::new(app_data));
    let d1 = Arc::clone(&d);
    let d2 = Arc::clone(&d);
    let d3 = Arc::clone(&d);

    thread::spawn(move || {
        let mut scanner = Scanner::new();
        let base_path = &CONFIG.dir;
        scanner.set_base_path(base_path);
        scanner.start(d1);
    });

    thread::spawn(move || {
        api::start(d2);
    });

    gui::start(d3);
}
