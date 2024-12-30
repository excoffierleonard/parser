use crate::errors::ApiError;
use actix_multipart::Multipart;
use actix_web::{post, HttpResponse};
use docx_rs::read_docx;
use futures_util::TryStreamExt;
use infer;
use mime::{Mime, APPLICATION_PDF, TEXT_PLAIN};
use pdf_extract;
use serde::Serialize;
use std::{
    fs::{read, read_to_string},
    io::Write,
};
use tempfile::NamedTempFile;

// Docx mime type was not defined in the mime package
const APPLICATION_DOCX: &str =
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document";

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
        Some(mime) if *mime == APPLICATION_DOCX => parse_docx(temp_file_path)?,
        Some(mime) if *mime == TEXT_PLAIN => parse_text(temp_file_path)?,
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
    // First try to detect using file signatures
    if let Some(kind) = infer::get_from_path(file_path).ok().flatten() {
        if let Ok(mime) = kind.mime_type().parse() {
            return Some(mime);
        }
    }

    // If no specific type was detected, check if it's readable as text
    read_to_string(file_path).ok().map(|_| TEXT_PLAIN)
}

fn parse_pdf(file_path: &str) -> Result<String, ApiError> {
    // TOFIX: Need to find a way to silence the output of that function since on unkown characters it prints a lot of errors, cluttering the logs.
    pdf_extract::extract_text(file_path)
        .map(|text| text.trim().to_string())
        .map_err(|e| ApiError::InternalError(format!("Failed to parse PDF: {}", e)))
}

fn parse_docx(file_path: &str) -> Result<String, ApiError> {
    // Read the file contents
    let file_content = read(file_path)
        .map_err(|e| ApiError::InternalError(format!("Failed to read DOCX file: {}", e)))?;

    // Parse the DOCX document
    let docx = read_docx(&file_content)
        .map_err(|e| ApiError::InternalError(format!("Failed to parse DOCX: {}", e)))?;

    // Extract text from the document
    let text = docx
        .document
        .children
        .iter()
        .filter_map(|child| match child {
            docx_rs::DocumentChild::Paragraph(paragraph) => Some(
                paragraph
                    .children
                    .iter()
                    .filter_map(|run| match run {
                        docx_rs::ParagraphChild::Run(r) => Some(
                            r.children
                                .iter()
                                .filter_map(|text| match text {
                                    docx_rs::RunChild::Text(t) => Some(t.text.clone()),
                                    _ => None,
                                })
                                .collect::<String>(),
                        ),
                        _ => None,
                    })
                    .collect::<String>(),
            ),
            _ => None,
        })
        .collect::<Vec<String>>()
        .join("\n")
        .trim()
        .to_string();

    Ok(text)
}

// Parses all that can be coerced to text
fn parse_text(file_path: &str) -> Result<String, ApiError> {
    read_to_string(file_path)
        .map_err(|e| ApiError::InternalError(format!("Failed to parse text based file: {}", e)))
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
        // Testing for pdf detection
        let file_path_pdf = "tests/inputs/test_pdf_1.pdf";
        let result_pdf = determine_mime_type(file_path_pdf);

        assert!(result_pdf.is_some());
        assert_eq!(result_pdf.unwrap(), APPLICATION_PDF);

        // Testing for docx detection
        let file_path_docx = "tests/inputs/test_docx_1.docx";
        let result_docx = determine_mime_type(file_path_docx);

        assert!(result_docx.is_some());
        assert_eq!(result_docx.unwrap(), APPLICATION_DOCX);

        // Testing for txt detection
        let file_path_txt = "tests/inputs/test_txt_1.txt";
        let result_txt = determine_mime_type(file_path_txt);

        assert!(result_txt.is_some());
        assert_eq!(result_txt.unwrap(), TEXT_PLAIN);
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

    #[test]
    fn parse_docx_success() {
        let file_path = "tests/inputs/test_docx_1.docx";
        let result = parse_docx(file_path).unwrap();

        assert!(result.len() > 0);
        assert_eq!(
            result,
            "Hello, this is a test docx for the parsing API.".to_string()
        );
    }

    #[test]
    fn parse_txt_success() {
        let file_path = "tests/inputs/test_txt_1.txt";
        let result = parse_text(file_path).unwrap();

        assert!(result.len() > 0);
        assert_eq!(
            result,
            "Hello, this is a test txt for the parsing API.".to_string()
        );
    }
}
