use actix_web::{post, Responder};

#[post("/parse")]
async fn parse_file() -> impl Responder {
    format!("Default")
}
