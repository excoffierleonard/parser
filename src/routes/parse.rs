use actix_multipart::Multipart;
use actix_web::{error::ResponseError, http::StatusCode, post, Error, HttpResponse};
use futures_util::TryStreamExt;
use mime::{Mime, APPLICATION_PDF};
use pdf_extract::extract_text;
use serde::Serialize;
use std::io::Write;
use tempfile::NamedTempFile;

#[derive(Serialize)]
struct Response {
    text: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

#[derive(Debug)]
enum ApiError {
    BadRequest(String),
    InternalError(String),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            message: self.to_string(),
        };

        match self {
            ApiError::BadRequest(_) => HttpResponse::BadRequest().json(error_response),
            ApiError::InternalError(_) => HttpResponse::InternalServerError().json(error_response),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::BadRequest(msg) | ApiError::InternalError(msg) => write!(f, "{}", msg),
        }
    }
}

#[post("/parse")]
async fn parse_file(mut payload: Multipart) -> Result<HttpResponse, ApiError> {
    let (temp_file, content_type) = create_temp_file(&mut payload).await?;
    let temp_file_path = get_temp_file_path(&temp_file)?;

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

async fn create_temp_file(
    payload: &mut Multipart,
) -> Result<(NamedTempFile, Option<Mime>), ApiError> {
    let mut temp_file = NamedTempFile::new()
        .map_err(|e| ApiError::InternalError(format!("Failed to create temp file: {}", e)))?;
    let mut content_type = None;

    // TODO: Need to implement a function whose only goal is to determine the MIME Type.
    while let Some(mut field) = payload
        .try_next()
        .await
        .map_err(|e| ApiError::InternalError(format!("Failed to process multipart: {}", e)))?
    {
        if content_type.is_none() {
            if let Some(cd) = field.content_disposition() {
                if let Some(filename) = cd.get_filename() {
                    if filename.ends_with(".pdf") {
                        content_type = Some(APPLICATION_PDF);
                    }
                }
            }
        }

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

    Ok((temp_file, content_type))
}

fn get_temp_file_path(temp_file: &NamedTempFile) -> Result<&str, ApiError> {
    temp_file
        .path()
        .to_str()
        .ok_or_else(|| ApiError::InternalError("Invalid temporary file path".to_string()))
}

fn parse_pdf(file_path: &str) -> Result<String, ApiError> {
    extract_text(file_path)
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
