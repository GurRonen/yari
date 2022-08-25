use std::env;

use config::{Config, File, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub(crate) struct Server {
     pub(crate) http: Http   
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub(crate) struct Http {
    pub(crate) address: String,
    pub(crate) port : u16
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub(crate) struct Expiration {
    pub(crate) active: Active
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub(crate) struct Active {
    pub(crate) ticks_per_second: u16,
    pub(crate) number_of_random_keys_per_tick: usize
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub(crate) struct Settings {
    pub(crate) server: Server,
    pub(crate) expiration: Expiration
}

impl Settings {
    pub(crate) fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let config = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .build()?;

        println!("{:?}", &config);
        config.try_deserialize()
    }
}