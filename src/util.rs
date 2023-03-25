use chrono::{DateTime, FixedOffset, NaiveDateTime};
use std::{fs, io};

pub fn md5_file(path: &str) -> Result<String, io::Error> {
    let f_r = fs::read(path);
    let f;
    match f_r {
        Ok(file) => f = file,
        Err(e) => {
            return Err(e);
        }
    }
    let s = md5::compute(f);
    Ok(format!("{:x}", s))
}

pub fn format_timestamp_to_datetime(timestamp: i64) -> String {
    let nt = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();
    let t = DateTime::<FixedOffset>::from_utc(nt, FixedOffset::east_opt(8 * 3600).unwrap());
    let st = t.format("%Y-%m-%d %H:%M:%S");
    st.to_string()
}
