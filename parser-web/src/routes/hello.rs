use crate::{errors::ApiError, responses::GreetingResponse};
use actix_web::{get, web};

/// Greets the user with a friendly message.
#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> Result<GreetingResponse, ApiError> {
    Ok(GreetingResponse {
        message: format!("Hello {}!", name),
    })
}
