use actix_web::{get, web, Error, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    message: String,
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> Result<HttpResponse, Error> {
    let response = Response {
        message: format!("Hello {name}!"),
    };

    Ok(HttpResponse::Ok().json(response))
}
