use actix_web::{
    middleware::{Compress, Logger},
    App, HttpServer,
};
use dotenvy::dotenv;
use env_logger::{self, Env};
use parser_web::{parse_file, serve_files};
use std::{env, io::Result};

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    dotenv().ok();

    let port = env::var("PARSER_APP_PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(8080);

    let enable_file_serving = env::var("ENABLE_FILE_SERVING")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(false);

    HttpServer::new(move || {
        let mut app = App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .service(parse_file);

        // Conditionally add serve_files service
        if enable_file_serving {
            app = app.service(serve_files);
        }

        app
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
