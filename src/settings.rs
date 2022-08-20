use std::env;

use config::{Config, File, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Server {
     pub http: Http   
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Http {
    pub address: String,
    pub port : u16
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub server: Server
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let config = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .build()?;

        println!("{:?}", &config);
        config.try_deserialize()
    }
}