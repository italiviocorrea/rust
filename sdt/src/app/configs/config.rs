use std::collections::HashMap;
use std::env;
use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;
use slog::{o, Drain, Logger};
use slog_async;
use slog_envlogger;
use slog_term;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Database {
    pub url: String,
    pub host: String,
    pub port: String,
    pub pwd: String,
    pub user: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Server {
    pub address: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Settings {
    pub database: Database,
    pub server: Server,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        let cfg = Config::builder()
            .add_source(
                File::with_name(&format!("./config/{}", run_mode))
                    .required(false),
            )
            .add_source(Environment::with_prefix("SDT"))
            .build().unwrap();

        cfg.try_deserialize()
    }

    pub fn configure_log() -> Logger {
        let decorator = slog_term::TermDecorator::new().build();
        let console_drain = slog_term::FullFormat::new(decorator).build().fuse();
        let console_drain = slog_envlogger::new(console_drain);
        let console_drain = slog_async::Async::new(console_drain).build().fuse();
        Logger::root(console_drain, o!("v" => env!("CARGO_PKG_VERSION")))
    }
}