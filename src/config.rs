use std::env;

pub struct Config {
    pub(crate) port: String,
    pub(crate) wh_path: String,
}

impl Config {
    pub fn new() -> Self {
        let port = match env::var("PORT") {
            Ok(v) => v.to_string(),
            Err(_) => "10001".to_string(),
        };
        let wh_path = match env::var("WH_PATH") {
            Ok(v) => v.to_string(),
            Err(_) => "warehouse".to_string(),
        };
        Self { port, wh_path }
    }
}
