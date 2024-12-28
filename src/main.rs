use actix_web::{
    middleware::{Compress, Logger},
    App, HttpServer,
};
use env_logger::Env;
use parser::routes::{greet, parse_file};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .service(greet)
            .service(parse_file)
    })
    .bind(("0.0.0.0", 8080))?
    .workers(num_cpus::get())
    .run()
    .await
}
