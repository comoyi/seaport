use crate::data::{AppData, FileInfo, FileType, ScanStatus, ServerStatus};
use crate::error::Error;
use crate::util;
use chrono::Local;
use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use log::{debug, info, warn};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
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

        let (tx, rx) = std::sync::mpsc::channel::<Event>();
        self.start_watcher(tx);
        self.start_worker(data, rx);
    }

    fn start_worker(&self, data: Arc<Mutex<AppData>>, rx: mpsc::Receiver<Event>) {
        let mut is_check = false;
        'outer: loop {
            let mut d_guard = data.lock().unwrap();
            match d_guard.server_status {
                ServerStatus::Started => {
                    drop(d_guard);
                    // skip for first time
                    if is_check {
                        // block until any change
                        while let Err(_) = rx.recv_timeout(Duration::from_secs(1)) {
                            let d_guard = data.lock().unwrap();
                            match d_guard.server_status {
                                ServerStatus::Stopping | ServerStatus::Stopped => {
                                    drop(d_guard);
                                    continue 'outer;
                                }
                                _ => {
                                    drop(d_guard);
                                }
                            }
                        }

                        // change status
                        let mut d_guard = data.lock().unwrap();
                        d_guard.server_file_info.scan_status = ScanStatus::Wait;
                        drop(d_guard);

                        // waiting for continuous change
                        while let Ok(_) = rx.recv_timeout(Duration::from_secs(3)) {}
                    }
                    is_check = true;
                    let scan_res = self.scan(&self.base_path, data.clone());
                    if let Err(e) = scan_res {
                        debug!("scan failed: {:?}", e);
                        let mut d_guard = data.lock().unwrap();
                        d_guard.server_file_info.scan_status = ScanStatus::Failed;
                        drop(d_guard);
                    }
                }
                _ => {
                    d_guard.server_file_info.scan_status = ScanStatus::Wait;
                    drop(d_guard);

                    is_check = false;
                }
            }
            thread::sleep(Duration::from_secs(1));
        }
    }

    fn scan(&self, base_path: &str, data: Arc<Mutex<AppData>>) -> Result<(), Error> {
        let mut files: Vec<FileInfo> = vec![];
        let mut d_guard = data.lock().unwrap();
        d_guard.server_file_info.scan_status = ScanStatus::Scanning;
        drop(d_guard);

        debug!("{}", "scan start");

        let d = walkdir::WalkDir::new(base_path);

        let iter = d.into_iter();
        for entry_res in iter {
            match entry_res {
                Ok(entry) => {
                    let absolute_path = entry.path().to_str().unwrap();
                    if absolute_path == base_path {
                        debug!("ignore base_path")
                    } else {
                        let relative_path = match Path::new(absolute_path).strip_prefix(base_path) {
                            Ok(p) => match p.to_str() {
                                None => {
                                    return Err(Error::ScanError);
                                }
                                Some(ps) => ps,
                            },
                            Err(_) => {
                                return Err(Error::ScanError);
                            }
                        };
                        let file_type;
                        let mut size = 0;
                        let mut hash_sum = "".to_string();
                        if entry.path().is_symlink() {
                            file_type = FileType::Symlink;
                            size = entry.metadata().unwrap().len();
                        } else if entry.path().is_dir() {
                            file_type = FileType::Dir;
                        } else if entry.path().is_file() {
                            file_type = FileType::File;
                            size = entry.metadata().unwrap().len();
                            hash_sum = util::md5_file(absolute_path);
                        } else {
                            warn!("ignored file type, relative_path: {}", relative_path);
                            continue;
                        }

                        debug!("abs_path: {}, rel_path: {}", absolute_path, relative_path);
                        let mut file = FileInfo::new();
                        file.relative_path = relative_path.to_string();
                        file.file_type = file_type;
                        file.size = size;
                        file.hash = hash_sum;
                        files.push(file);
                    }
                }
                Err(_) => {
                    return Err(Error::ScanError);
                }
            }
        }

        let mut d_guard = data.lock().unwrap();
        d_guard.server_file_info.files = files;
        d_guard.server_file_info.scan_status = ScanStatus::Completed;
        d_guard.server_file_info.last_scan_finish_time = Local::now().timestamp();
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

        Ok(())
    }

    fn start_watcher(&self, tx: Sender<Event>) {
        let path = self.base_path.to_string();
        thread::spawn(|| {
            futures::executor::block_on(async {
                if let Err(e) = async_watch(path, tx).await {
                    warn!("error: {:?}", e)
                }
            });
        });
    }
}

fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);

    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

async fn async_watch<P: AsRef<Path>>(path: P, tx: Sender<Event>) -> notify::Result<()> {
    let (mut watcher, mut rx) = async_watcher()?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.next().await {
        match res {
            Ok(event) => {
                // debug!("changed: {:?}", event);
                tx.send(event).unwrap();
            }
            Err(e) => {
                warn!("watch error: {:?}", e)
            }
        }
    }

    Ok(())
}
