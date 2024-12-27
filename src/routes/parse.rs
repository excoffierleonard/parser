use actix_web::{post, Error, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Response {
    name: String,
    text: String,
}

#[post("/parse")]
async fn parse_file() -> Result<HttpResponse, Error> {
    let response = Response {
        name: "test_pdf.pdf".to_string(),
        text: "Hello, this is a test pdf for the parsing API.".to_string(),
    };
    Ok(HttpResponse::Ok().json(response))
}
