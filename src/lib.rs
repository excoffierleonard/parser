use actix_web::{get, post, web, Responder};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[post("/parse")]
async fn parse() -> impl Responder {
    format!("Default")
}
