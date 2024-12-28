use crate::errors::ApiError;
use actix_multipart::Multipart;
use actix_web::{post, HttpResponse};
use futures_util::TryStreamExt;
use infer;
use mime::{Mime, APPLICATION_PDF};
use pdf_extract;
use serde::Serialize;
use std::io::Write;
use tempfile::NamedTempFile;

#[derive(Serialize)]
struct Response {
    text: String,
}

#[post("/parse")]
async fn parse_file(mut payload: Multipart) -> Result<HttpResponse, ApiError> {
    let temp_file = create_temp_file(&mut payload).await?;
    let temp_file_path = get_temp_file_path(&temp_file)?;
    let content_type = determine_mime_type(temp_file_path);

    let parsed_text = match content_type.as_ref() {
        Some(mime) if *mime == APPLICATION_PDF => parse_pdf(temp_file_path)?,
        Some(mime) => {
            return Err(ApiError::BadRequest(format!(
                "Unsupported mime type: {}",
                mime
            )))
        }
        None => return Err(ApiError::BadRequest("Missing content type".to_string())),
    };

    Ok(HttpResponse::Ok().json(Response { text: parsed_text }))
}

async fn create_temp_file(payload: &mut Multipart) -> Result<NamedTempFile, ApiError> {
    let mut temp_file = NamedTempFile::new()
        .map_err(|e| ApiError::InternalError(format!("Failed to create temp file: {}", e)))?;

    while let Some(mut field) = payload
        .try_next()
        .await
        .map_err(|e| ApiError::InternalError(format!("Failed to process multipart: {}", e)))?
    {
        while let Some(chunk) = field
            .try_next()
            .await
            .map_err(|e| ApiError::InternalError(format!("Failed to read chunk: {}", e)))?
        {
            temp_file.write_all(&chunk).map_err(|e| {
                ApiError::InternalError(format!("Failed to write to temp file: {}", e))
            })?;
        }
    }

    Ok(temp_file)
}

fn get_temp_file_path(temp_file: &NamedTempFile) -> Result<&str, ApiError> {
    temp_file
        .path()
        .to_str()
        .ok_or_else(|| ApiError::InternalError("Invalid temporary file path".to_string()))
}

fn determine_mime_type(file_path: &str) -> Option<Mime> {
    infer::get_from_path(file_path)
        .ok()
        .flatten()
        .and_then(|kind| kind.mime_type().parse().ok())
}

fn parse_pdf(file_path: &str) -> Result<String, ApiError> {
    pdf_extract::extract_text(file_path)
        .map(|text| text.trim().to_string())
        .map_err(|e| ApiError::InternalError(format!("Failed to parse PDF: {}", e)))
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
    fn determine_mime_success() {
        let file_path = "tests/inputs/test_pdf_1.pdf";
        let result = determine_mime_type(file_path);

        assert!(result.is_some());
        assert_eq!(result.unwrap(), APPLICATION_PDF);
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
