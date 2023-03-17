use serde::Serialize;
use serde_repr::Serialize_repr;

struct Data {}

pub enum ServerStatus {
    Starting,
    Started,
    Stopping,
    Stopped,
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
    pub scan_status: ScanStatus,
    pub files: Vec<FileInfo>,
}

impl ServerFileInfo {
    pub fn new() -> Self {
        ServerFileInfo {
            scan_status: ScanStatus::Wait,
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
