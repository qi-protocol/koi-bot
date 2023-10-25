use crate::{Error, Result};
use std::env;
use std::sync::OnceLock;

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|e| panic!("Fatal while loading config: {:?}", e))
    })
}

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Config {
    /// DB
    pub DB_URL: String,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Config {
            DB_URL: env::var("SERVICE_DB_URL")
                .map_err(|_| Error::ConfigMissingEnv("SERVICE_DB_URL"))?,
        })
    }
}
