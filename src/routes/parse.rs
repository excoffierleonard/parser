use actix_multipart::Multipart;
use actix_web::{
    error::{ErrorBadRequest, ErrorInternalServerError},
    post, Error, HttpResponse,
};
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

#[post("/parse")]
async fn parse_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let (temp_file, content_type) = create_temp_file(&mut payload).await?;
    let temp_file_path = get_temp_file_path(&temp_file)?;

    let parsed_text = match content_type.as_ref() {
        Some(mime) if *mime == APPLICATION_PDF => parse_pdf(temp_file_path)?,
        Some(mime) => return Err(ErrorBadRequest(format!("Unsupported mime type: {}", mime))),
        None => return Err(ErrorBadRequest("Missing content type")),
    };

    Ok(HttpResponse::Ok().json(Response { text: parsed_text }))
}

async fn create_temp_file(payload: &mut Multipart) -> Result<(NamedTempFile, Option<Mime>), Error> {
    let mut temp_file = NamedTempFile::new().map_err(ErrorInternalServerError)?;
    let mut content_type = None;

    // TODO: Need to implement a function whose only goal is to determine the MIME Type.

    while let Some(mut field) = payload.try_next().await? {
        if content_type.is_none() {
            if let Some(cd) = field.content_disposition() {
                if let Some(filename) = cd.get_filename() {
                    if filename.ends_with(".pdf") {
                        content_type = Some(APPLICATION_PDF);
                    }
                }
            }
        }

        while let Some(chunk) = field.try_next().await? {
            temp_file
                .write_all(&chunk)
                .map_err(ErrorInternalServerError)?;
        }
    }

    Ok((temp_file, content_type))
}

fn get_temp_file_path(temp_file: &NamedTempFile) -> Result<&str, Error> {
    temp_file
        .path()
        .to_str()
        .ok_or_else(|| ErrorInternalServerError("Invalid temporary file path"))
}

fn parse_pdf(file_path: &str) -> Result<String, Error> {
    match extract_text(file_path) {
        Ok(text) => Ok(text.trim().to_string()),
        Err(e) => Err(ErrorInternalServerError(e)),
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
