use crate::data::{FileInfo, FileType, ScanStatus, ServerFileInfo};
use crate::util;
use log::{debug, info, warn};

pub fn start() {
    info!("start scanner");
    let base_path = "/tmp/a";

    let mut server_file_info = ServerFileInfo::new();
    let mut files: Vec<FileInfo> = vec![];

    server_file_info.scan_status = ScanStatus::Scanning;

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
    server_file_info.files = files;
    server_file_info.scan_status = ScanStatus::Completed;
    let mut j = String::from("");
    let jr = serde_json::to_string(&server_file_info);
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
