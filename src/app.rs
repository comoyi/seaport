use crate::data::{FileInfo, ServerFileInfo};
use crate::scanner::Scanner;
use crate::{api, gui};
use log::info;

pub fn start() {
    info!("start app");

    let mut server_file_info = ServerFileInfo::new();
    let mut files: Vec<FileInfo> = vec![];
    server_file_info.files = files;

    let mut scanner = Scanner::new();
    let base_path = "/tmp/a";
    scanner.set_base_path(base_path);
    scanner.start(&mut server_file_info);

    api::start();
    gui::start();
}
