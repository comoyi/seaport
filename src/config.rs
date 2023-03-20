use config::builder::DefaultState;
use config::ConfigBuilder;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::path::Path;

lazy_static! {
    pub static ref CONFIG: Config = init_config();
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub log_level: String,
    pub port: u16,
    pub dir: String,
    pub title: String,
}

pub fn init_config() -> Config {
    let mut b = config::Config::builder();

    b = set_default(b);

    let cps = vec![
        "config.toml",
        // "config/config.toml"
    ];
    for cp_str in cps {
        let cp = Path::new(cp_str);
        if cp.exists() {
            // println!("Add config file: {:?}", cp);
            b = b.add_source(config::File::from(cp))
        }
    }

    let c = b.build().unwrap();
    let conf_result = c.try_deserialize::<Config>();
    let conf;
    match conf_result {
        Ok(c) => {
            conf = c;
        }
        Err(e) => {
            println!("load config failed: {}", e.to_string());
            panic!("load config failed: {}", e.to_string());
        }
    }
    // println!("{:?}", conf);
    conf
}

fn set_default(b: ConfigBuilder<DefaultState>) -> ConfigBuilder<DefaultState> {
    b.set_default("log_level", "TRACE")
        .unwrap()
        .set_default("port", 8080)
        .unwrap()
        .set_default("dir", "")
        .unwrap()
        .set_default("title", "")
        .unwrap()
}
