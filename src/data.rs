use serde::Serialize;
use serde_repr::Serialize_repr;

#[derive(Serialize)]
pub struct AppData {
    pub server_status: ServerStatus,
    pub server_file_info: ServerFileInfo,
    pub announcement: Announcement,
}

impl AppData {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for AppData {
    fn default() -> Self {
        AppData {
            server_status: ServerStatus::Stopped,
            server_file_info: ServerFileInfo::new(),
            announcement: Announcement::default(),
        }
    }
}

#[derive(Serialize)]
pub enum ServerStatus {
    Starting = 10,
    Started = 20,
    Stopping = 30,
    Stopped = 40,
}

#[derive(Serialize_repr)]
#[repr(i8)]
pub enum ScanStatus {
    Wait = 10,
    Scanning = 20,
    Failed = 30,
    Completed = 40,
}

#[derive(Serialize)]
pub struct ServerFileInfo {
    #[serde(rename = "status")]
    pub scan_status: ScanStatus,
    pub last_scan_finish_time: i64,
    pub files: Vec<FileInfo>,
}

impl ServerFileInfo {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for ServerFileInfo {
    fn default() -> Self {
        ServerFileInfo {
            scan_status: ScanStatus::Wait,
            last_scan_finish_time: 0,
            files: vec![],
        }
    }
}

#[derive(Serialize_repr)]
#[repr(i8)]
pub enum FileType {
    Unknown = 0,
    File = 1,
    Dir = 2,
    Symlink = 4,
}

#[derive(Serialize)]
pub struct FileInfo {
    pub relative_path: String,
    #[serde(rename = "type")]
    pub file_type: FileType,
    pub hash: String,
}

impl FileInfo {
    pub fn new() -> Self {
        FileInfo {
            relative_path: "".to_string(),
            file_type: FileType::Unknown,
            hash: "".to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct Announcement {
    pub content: String,
    hash: String,
}

impl Default for Announcement {
    fn default() -> Self {
        Announcement {
            content: "".to_string(),
            hash: "".to_string(),
        }
    }
}
