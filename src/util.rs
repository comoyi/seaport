use std::fs;

pub fn md5_file(path: &str) -> String {
    let f = fs::read(path).unwrap();
    let s = md5::compute(f);
    format!("{:x}", s)
}
