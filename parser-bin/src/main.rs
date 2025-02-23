use actix_web::{
    middleware::{Compress, Logger},
    App, HttpServer,
};
use env_logger::{init_from_env, Env};
use parser_web::{
    routes::{greet, parse_file, serve_files},
    Config,
};
use std::io::{Error, ErrorKind, Result};

#[actix_web::main]
async fn main() -> Result<()> {
    init_from_env(Env::default().default_filter_or("info"));

    let config = Config::build().map_err(|e| Error::new(ErrorKind::Other, e))?;

    HttpServer::new(move || {
        let mut app = App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .service(greet)
            .service(parse_file);

        // Conditionally add serve_files service
        if config.enable_file_serving {
            app = app.service(serve_files);
        }

        app
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await
}
