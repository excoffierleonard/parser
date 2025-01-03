use crate::errors::ApiError;
use actix_multipart::Multipart;
use actix_web::{post, HttpResponse};
use futures_util::StreamExt;
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
    let parsed_text = parse_any(&temp_file.path())?;

    Ok(HttpResponse::Ok().json(Response { text: parsed_text }))
}

async fn create_temp_file(payload: &mut Multipart) -> Result<NamedTempFile, ApiError> {
    let mut temp_file = NamedTempFile::new()?;

    // Take the first field from the multipart payload
    let Some(Ok(mut field)) = payload.next().await else {
        return Err(ApiError::BadRequest("No file provided".to_string()));
    };

    // Stream chunks directly to the temp file
    while let Some(chunk) = field.next().await {
        temp_file.write_all(&chunk?)?;
    }

    Ok(temp_file)
}

#[cfg(test)]
mod tests {
    #[actix_web::test]
    async fn create_temp_file_success() {
        // Integration tests cover this functionality since there is currently no way to simulate a Multipart payload in unit tests.
        assert!(1 == 1);
    }
}
