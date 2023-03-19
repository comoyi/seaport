use crate::data::{AppData, FileInfo, FileType, ScanStatus};
use crate::util;
use log::{debug, info, warn};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct Scanner {
    base_path: String,
}

impl Scanner {
    pub fn new() -> Self {
        Scanner {
            base_path: "".to_string(),
        }
    }

    pub fn set_base_path(&mut self, base_path: &str) {
        self.base_path = base_path.to_string();
    }

    pub fn start(&self, data: Arc<Mutex<AppData>>) {
        info!("start scanner");

        loop {
            thread::sleep(Duration::from_secs(3));

            self.scan(&self.base_path, data.clone());
        }
    }

    fn scan(&self, base_path: &str, data: Arc<Mutex<AppData>>) {
        let mut files: Vec<FileInfo> = vec![];
        let mut d_guard = data.lock().unwrap();
        d_guard.server_file_info.scan_status = ScanStatus::Scanning;
        drop(d_guard);

        debug!("{}", "scan start");

        let d = walkdir::WalkDir::new(base_path);
        d.into_iter().for_each(|entry_res| match entry_res {
            Ok(entry) => {
                let absolute_path = entry.path().to_str().unwrap();
                if absolute_path == base_path {
                    debug!("ignore base_path")
                } else {
                    let relative_path = absolute_path
                        .trim_start_matches(base_path)
                        .trim_start_matches("/");
                    let file_type;
                    let mut hash_sum = "".to_string();
                    if entry.path().is_symlink() {
                        file_type = FileType::Symlink;
                    } else if entry.path().is_dir() {
                        file_type = FileType::Dir;
                    } else if entry.path().is_file() {
                        file_type = FileType::File;
                        hash_sum = util::md5_file(absolute_path);
                    } else {
                        warn!("unexpected file type, relative_path: {}", relative_path);
                        return;
                    }

                    debug!("abs_path: {}, rel_path: {}", absolute_path, relative_path);
                    let mut file = FileInfo::new();
                    file.relative_path = relative_path.to_string();
                    file.file_type = file_type;
                    file.hash = hash_sum;
                    files.push(file);
                }
            }
            Err(_) => {}
        });

        let mut d_guard = data.lock().unwrap();
        d_guard.server_file_info.files = files;
        d_guard.server_file_info.scan_status = ScanStatus::Completed;
        drop(d_guard);

        let mut j = String::from("");
        let d_guard = data.lock().unwrap();
        let sfi = &d_guard.server_file_info;
        let jr = serde_json::to_string(sfi);
        drop(d_guard);
        match jr {
            Ok(js) => {
                j = js;
            }
            Err(_) => {
                warn!("json serialize failed");
            }
        }

        debug!("json: {}", j);
        debug!("{}", "scan completed");
    }
}
