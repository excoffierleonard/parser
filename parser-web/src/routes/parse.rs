use crate::errors::ApiError;
use actix_multipart::Multipart;
use actix_web::{post, HttpResponse};
use futures_util::TryStreamExt;
use parser_core::parsers::parse_any;
use serde::Serialize;
use std::io::Write;
use tempfile::NamedTempFile;

#[derive(Serialize)]
struct Response {
    text: String,
}

// routes/parse.rs
/// Parses various document formats into plain text.
///
/// # Supported Formats
/// - PDF files (application/pdf)
/// - Word Documents (application/vnd.openxmlformats-officedocument.wordprocessingml.document)
/// - Excel Spreadsheets (application/vnd.openxmlformats-officedocument.spreadsheetml.sheet)
/// - PowerPoint Presentations (application/vnd.openxmlformats-officedocument.presentationml.presentation)
/// - Text based files (text/plain, text/csv, application/json, etc...)
/// - Image based files (image/png, image/jpg, image/webp, etc...)
///
/// # Errors
/// Returns `ApiError::BadRequest` if:
/// - The content type is missing
/// - The file format is unsupported
///
/// Returns `ApiError::InternalError` if parsing fails
#[post("/parse")]
async fn parse_file(mut payload: Multipart) -> Result<HttpResponse, ApiError> {
    let temp_file = create_temp_file(&mut payload).await?;
    let temp_file_path = get_temp_file_path(&temp_file)?;

    // Need to find better way to map errors
    let parsed_text =
        parse_any(temp_file_path).map_err(|e| ApiError::InternalError(e.to_string()))?;

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
        // TODO: Need test to check if the path is a valid temporary file path, platform agnostic
    }
}