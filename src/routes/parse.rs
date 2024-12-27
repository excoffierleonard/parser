use actix_web::{post, Error, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    text: String,
}

#[post("/parse")]
async fn parse_file() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(Response {
        text: "Hello, this is a test pdf for the parsing API.".to_string(),
    }))
}
