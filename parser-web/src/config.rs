//! Configuration for the web server.

use crate::errors::ApiError;
use dotenv::dotenv;
use std::env::var;

/// Configuration for the web server.
#[derive(Debug)]
pub struct Config {
    /// The port the server should listen on.
    pub port: u16,
    /// Whether to enable file serving.
    pub enable_file_serving: bool,
}

impl Config {
    /// Builds a new configuration from environment variables.
    pub fn build() -> Result<Self, ApiError> {
        dotenv().ok();

        let port = var("PARSER_APP_PORT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(8080);

        let enable_file_serving = var("ENABLE_FILE_SERVING")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(false);

        Ok(Self {
            port,
            enable_file_serving,
        })
    }
}
