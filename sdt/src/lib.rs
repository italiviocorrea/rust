mod domain;
mod app;

pub use mongodb::{Client};
pub use mongodb::options::ClientOptions;
pub use std::net::TcpListener;
pub use crate::domain::entities::movie_schema::{Movie, create_name_unique};
pub use dotenv;
pub use crate::app::rest::start_server::start_server::run;
pub use crate::app::configs::config::Settings;

pub use crate::app::rest::routes::*;

