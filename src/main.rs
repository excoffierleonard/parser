use actix_web::{App, HttpServer};
use parser::routes::{greet, parse_file};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(greet).service(parse_file))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
