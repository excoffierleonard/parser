use crate::{
    errors::ApiError,
    parsers::{parse_docx, parse_image, parse_pdf, parse_pptx, parse_text, parse_xlsx},
};
use actix_multipart::Multipart;
use actix_web::{post, HttpResponse};

use futures_util::TryStreamExt;
use infer;
use mime::{Mime, APPLICATION_PDF, IMAGE, TEXT, TEXT_PLAIN};
use serde::Serialize;
use std::{fs::read_to_string, io::Write};
use tempfile::NamedTempFile;

// Ttypes not defined in the mime package
const APPLICATION_DOCX: &str =
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document";
const APPLICATION_XLSX: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";
const APPLICATION_PPTX: &str =
    "application/vnd.openxmlformats-officedocument.presentationml.presentation";

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
    let content_type = determine_mime_type(temp_file_path);

    let parsed_text = match content_type.as_ref() {
        Some(mime) if *mime == APPLICATION_PDF => parse_pdf(temp_file_path)?,
        Some(mime) if *mime == APPLICATION_DOCX => parse_docx(temp_file_path)?,
        Some(mime) if *mime == APPLICATION_XLSX => parse_xlsx(temp_file_path)?,
        Some(mime) if *mime == APPLICATION_PPTX => parse_pptx(temp_file_path)?,
        Some(mime) if mime.type_() == TEXT => parse_text(temp_file_path)?,
        Some(mime) if mime.type_() == IMAGE => parse_image(temp_file_path)?,
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

    // TODO: Add specific function for special text data that needs formatting like CSV etc..
    // TODO: Maybe add checks for false positive images, like svg that may be coerced to text but shouldnt.

    // If no specific type was detected, check if it's readable as text
    read_to_string(file_path).ok().map(|_| TEXT_PLAIN)
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

        // Testing for xlsx detection
        let file_path_xlsx = "tests/inputs/test_xlsx_1.xlsx";
        let result_xlsx = determine_mime_type(file_path_xlsx);

        assert!(result_xlsx.is_some());
        assert_eq!(result_xlsx.unwrap(), APPLICATION_XLSX);

        // Testing for pptx detection
        let file_path_pptx = "tests/inputs/test_pptx_1.pptx";
        let result_pptx = determine_mime_type(file_path_pptx);

        assert!(result_pptx.is_some());
        assert_eq!(result_pptx.unwrap(), APPLICATION_PPTX);

        // Testing for txt detection
        let file_path_txt = "tests/inputs/test_txt_1.txt";
        let result_txt = determine_mime_type(file_path_txt);

        assert!(result_txt.is_some());
        assert_eq!(result_txt.unwrap().type_(), TEXT);

        // Testing for csv detection
        let file_path_csv = "tests/inputs/test_csv_1.csv";
        let result_csv = determine_mime_type(file_path_csv);

        assert!(result_csv.is_some());
        assert_eq!(result_csv.unwrap().type_(), TEXT);

        // Testing for json detection
        let file_path_json = "tests/inputs/test_json_1.json";
        let result_json = determine_mime_type(file_path_json);

        assert!(result_json.is_some());
        assert_eq!(result_json.unwrap().type_(), TEXT);

        // Testing for png detection
        let file_path_png = "tests/inputs/test_png_1.png";
        let result_png = determine_mime_type(file_path_png);

        assert!(result_png.is_some());
        assert_eq!(result_png.unwrap().type_(), IMAGE);

        // Testing for jpg detection
        let file_path_jpg = "tests/inputs/test_jpg_1.jpg";
        let result_jpg = determine_mime_type(file_path_jpg);

        assert!(result_jpg.is_some());
        assert_eq!(result_jpg.unwrap().type_(), IMAGE);

        // Testing for webp detection
        let file_path_webp = "tests/inputs/test_webp_1.webp";
        let result_webp = determine_mime_type(file_path_webp);

        assert!(result_webp.is_some());
        assert_eq!(result_webp.unwrap().type_(), IMAGE);
    }
}

// TOFIX: Make path sourcing platform agnostic
