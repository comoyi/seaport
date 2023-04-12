use chrono::{DateTime, FixedOffset, NaiveDateTime};
use std::path::Path;
use std::{fs, io};

pub fn md5_file<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    let file_data_r = fs::read(path);
    let file_data = match file_data_r {
        Ok(d) => d,
        Err(e) => {
            return Err(e);
        }
    };
    let s = md5::compute(file_data);
    Ok(format!("{:x}", s))
}

pub fn format_timestamp_to_datetime(timestamp: i64) -> String {
    let nt = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();
    let t = DateTime::<FixedOffset>::from_utc(nt, FixedOffset::east_opt(8 * 3600).unwrap());
    let st = t.format("%Y-%m-%d %H:%M:%S");
    st.to_string()
}
