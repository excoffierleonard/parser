use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub port: u16,
}

impl Config {
    pub fn build() -> Result<Self, env::VarError> {
        dotenv().ok();

        let port = env::var("PARSER_APP_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .unwrap_or(8080);

        Ok(Self { port })
    }
}
