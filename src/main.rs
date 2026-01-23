use actix_web::{
    App, HttpServer,
    middleware::{Compress, Logger},
};
use dotenvy::dotenv;
use env_logger::Env;
use std::{env, io::Result};

mod core;
mod web;

use web::{parse_file, serve_files};

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    dotenv().ok();

    let port = env::var("PARSER_APP_PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(8080);

    HttpServer::new(|| {
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .service(parse_file)
            .service(serve_files)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
