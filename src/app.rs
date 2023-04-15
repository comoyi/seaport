use crate::config::CONFIG;
use crate::data::{AppData, ServerStatus};
use crate::scanner::Scanner;
use crate::{api, gui};
use log::info;
use std::sync::{Arc, Mutex};
use std::thread;

pub const APP_NAME: &str = "Valheim Server Toolkit";

pub fn start() {
    info!("start app");

    let mut app_data = AppData::new();
    app_data.server_status = ServerStatus::Started;
    app_data.announcement.content = CONFIG.announcement.to_string();
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

    let api_thread = thread::spawn(move || {
        api::start(d2);
    });

    if CONFIG.gui {
        gui::start(d3);
    }

    api_thread.join().unwrap();
}
