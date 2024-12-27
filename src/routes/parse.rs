use actix_web::{post, Error, HttpResponse};
use pdf_extract::extract_text;
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    text: String,
}

#[post("/parse")]
async fn parse_file() -> Result<HttpResponse, Error> {
    let temp_file_path = ;
    let parsed_text = parse_pdf(temp_file_path).await.unwrap();

    Ok(HttpResponse::Ok().json(Response { text: parsed_text }))
}

async fn parse_pdf(file_path: &str) -> Result<String, Error> {
    match extract_text(file_path) {
        Ok(text) => Ok(text.trim().to_string()),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn parse_a_pdf() {
        let file_path = "tests/inputs/test_pdf_1.pdf";
        let result = parse_pdf(file_path).await.unwrap();

        assert_eq!(
            result,
            "Hello, this is a test pdf for the parsing API.".to_string()
        );
    }
}
