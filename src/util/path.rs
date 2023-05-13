use log::debug;
use std::env::current_exe;
use std::error::Error;

pub fn get_exe_dir() -> Result<String, Box<dyn Error>> {
    let exe_path_r = current_exe();
    let exe_path;
    match exe_path_r {
        Ok(p) => {
            exe_path = p;
        }
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("get current_exe_path failed!, err: {}", e),
            )));
        }
    }
    debug!("exe_path: {:?}", exe_path);
    let exe_dir_o = exe_path.parent();
    let exe_dir;
    match exe_dir_o {
        None => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "get exe_dir failed!",
            )));
        }
        Some(p) => {
            exe_dir = p;
        }
    }
    debug!("exe_dir: {:?}", exe_dir);

    let exe_dir_o2 = exe_dir.to_str();
    let base_dir;
    match exe_dir_o2 {
        None => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "convert exe_dir failed!",
            )));
        }
        Some(p) => {
            base_dir = p.to_string();
        }
    }
    Ok(base_dir)
}
