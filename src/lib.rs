use actix_web::{get, web, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}
