use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub port: u16,
    pub url: String,
    pub service_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Azure {
    pub account: String,
    pub key: String,
    pub file_system: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Local {
    pub area_root: String,
    pub artifacts_root: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Service {
    pub storage: String,
    pub local: Local,
    pub azure: Azure,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: Server,
    pub service: Service,
    pub log: Log,
    pub env: Env,
}

#[allow(unused)]
const CONFIG_FILE_PATH: &str = "./config/Default.toml";
#[allow(unused)]
const CONFIG_FILE_PREFIX: &str = "./config/";

impl Settings {
    #[allow(unused)]
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = std::env::var("RUN_ENV").unwrap_or_else(|_| "Production".into());

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name(CONFIG_FILE_PATH))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(
                File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, run_mode)).required(false),
            )
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("ff").separator("__"))
            .build()?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub enum Env {
    Development,
    Testing,
    Production,
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Env::Development => write!(f, "Development"),
            Env::Testing => write!(f, "Testing"),
            Env::Production => write!(f, "Production"),
        }
    }
}

impl From<&str> for Env {
    fn from(env: &str) -> Self {
        match env {
            "Testing" => Env::Testing,
            "Production" => Env::Production,
            _ => Env::Development,
        }
    }
}
