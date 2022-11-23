use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Jaeger {
    pub endpoint: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Settings {
    pub env: ENV,
    pub log: Log,
    pub server: Server,
    pub database: Database,
    pub jaeger: Jaeger,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Database {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

const CONFIG_FILE_PATH: &str = "config/Default";
const CONFIG_FILE_PREFIX: &str = "config/";

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {

        let run_mode = std::env::var("RUN_ENV").unwrap_or_else(|_| "Development".into());
        println!("{}",run_mode);

        let s = Config::builder()
            .add_source(File::with_name(CONFIG_FILE_PATH))
            .add_source(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, run_mode)))
            .add_source(Environment::with_prefix("sdt").separator("_"))
            .set_override("database.url", "mongodb://")?
            .set_override("env","DEV")?
            .build()?;
        s.try_deserialize()
    }
}

#[derive(Clone, Debug, Deserialize)]
#[allow(unused)]
pub enum ENV {
    DEV,
    HOM,
    PRD,
}

impl fmt::Display for ENV {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ENV::DEV => write!(f, "Development"),
            ENV::HOM => write!(f, "Homologation"),
            ENV::PRD => write!(f, "Production"),
        }
    }
}

impl From<&str> for ENV {
    fn from(env: &str) -> Self {
        match env {
            "Homologation" => ENV::HOM,
            "Production" => ENV::PRD,
            _ => ENV::DEV,
        }
    }
}
