mod cors;
mod db;
mod dto;
mod error;
mod keys;
mod sendgrid;
mod token_middleware;

pub use cors::*;
pub use db::*;
pub use dto::*;
pub use error::*;
pub use keys::*;
pub use sendgrid::*;
pub use token_middleware::*;

use std::env;

pub fn get_server_address() -> &'static str {
    let env = env::var("ENVIRONMENT").unwrap_or_else(|_| "prod".to_string());

    if env == "dev" {
        "127.0.0.1:8080"
    } else {
        "127.0.0.1:5001"
    }
}

pub fn get_env() -> String {
    env::var("ENVIRONMENT").unwrap_or_else(|_| "prod".to_string())
}
