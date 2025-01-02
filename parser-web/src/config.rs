use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub port: u16,
    pub enable_file_serving: bool,
}

impl Config {
    pub fn build() -> Result<Self, env::VarError> {
        dotenv().ok();

        let port = env::var("PARSER_APP_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .unwrap_or(8080);

        let enable_file_serving = env::var("ENABLE_FILE_SERVING")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .unwrap_or(false);

        Ok(Self {
            port,
            enable_file_serving,
        })
    }
}
