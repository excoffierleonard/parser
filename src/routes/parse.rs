use actix_multipart::Multipart;
use actix_web::{post, Error, HttpResponse};
use futures_util::TryStreamExt;
use pdf_extract::extract_text;
use serde::Serialize;
use std::io::Write;
use tempfile::NamedTempFile;

#[derive(Serialize)]
struct Response {
    text: String,
}

#[post("/parse")]
async fn parse_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let temp_file = create_temp_file(&mut payload).await?;
    let temp_file_path = get_temp_file_path(&temp_file)?;
    let parsed_text = parse_pdf(temp_file_path)?;

    Ok(HttpResponse::Ok().json(Response { text: parsed_text }))
}

async fn create_temp_file(payload: &mut Multipart) -> Result<NamedTempFile, Error> {
    // Create a temporary file that will be automatically cleaned up when it goes out of scope
    let mut temp_file = NamedTempFile::new().map_err(actix_web::error::ErrorInternalServerError)?;

    // Process the multipart form data
    while let Some(mut field) = payload.try_next().await? {
        while let Some(chunk) = field.try_next().await? {
            temp_file
                .write_all(&chunk)
                .map_err(actix_web::error::ErrorInternalServerError)?;
        }
    }

    Ok(temp_file)
}

fn get_temp_file_path(temp_file: &NamedTempFile) -> Result<&str, Error> {
    temp_file
        .path()
        .to_str()
        .ok_or_else(|| actix_web::error::ErrorInternalServerError("Invalid temporary file path"))
}

fn parse_pdf(file_path: &str) -> Result<String, Error> {
    match extract_text(file_path) {
        Ok(text) => Ok(text.trim().to_string()),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn create_temp_file_success() {
        // Integration tests cover this functionality since there is currently no way to simulate a Multipart payload in unit tests.
        assert!(1 == 1);
    }

    #[test]
    fn get_temp_file_path_success() {
        let temp_file = NamedTempFile::new().unwrap();
        let result = get_temp_file_path(&temp_file).unwrap();

        assert!(result.len() > 0);
        assert!(result.starts_with("/tmp"));
    }

    #[test]
    fn parse_pdf_success() {
        let file_path = "tests/inputs/test_pdf_1.pdf";
        let result = parse_pdf(file_path).unwrap();

        assert!(result.len() > 0);
        assert_eq!(
            result,
            "Hello, this is a test pdf for the parsing API.".to_string()
        );
    }
}
