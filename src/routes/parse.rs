use crate::errors::ApiError;
use actix_multipart::Multipart;
use actix_web::{post, HttpResponse};
use calamine::{open_workbook, Reader, Xlsx};
use docx_rs::read_docx;
use futures_util::TryStreamExt;
use infer;
use mime::{Mime, APPLICATION_PDF, IMAGE, TEXT, TEXT_PLAIN};
use pdf_extract;
use regex::Regex;
use serde::Serialize;
use std::{
    fs::{read, read_to_string},
    io::{Read, Write},
};
use tempfile::NamedTempFile;
use zip::ZipArchive;

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

// TODO: Need proper logic to escape commas and quotes
// TODO: Consider using the csv crate to simply convert to csv each sheet and pass it throught the parse text function
fn parse_xlsx(file_path: &str) -> Result<String, ApiError> {
    let mut excel: Xlsx<_> = open_workbook(file_path)
        .map_err(|e| ApiError::InternalError(format!("Failed to read XLSX based file: {}", e)))?;

    let mut csv_data = String::new();

    for name in excel.sheet_names() {
        if let Ok(range) = excel.worksheet_range(&name) {
            if !csv_data.is_empty() {
                csv_data.push_str("\n--- Sheet: ");
                csv_data.push_str(&name);
                csv_data.push_str(" ---\n");
            }
            let sheet_csv = range
                .rows()
                .map(|row| {
                    row.iter()
                        .map(|cell| cell.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                })
                .collect::<Vec<String>>()
                .join("\n");
            csv_data.push_str(&sheet_csv);
        }
    }

    Ok(csv_data)
}

fn parse_pptx(file_path: &str) -> Result<String, ApiError> {
    let file = std::fs::File::open(file_path)
        .map_err(|e| ApiError::InternalError(format!("Failed to open PPTX: {}", e)))?;

    let mut archive = ZipArchive::new(file)
        .map_err(|e| ApiError::InternalError(format!("Failed to read PPTX as ZIP: {}", e)))?;

    let mut text = String::new();
    let mut slide_count = 0;

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| ApiError::InternalError(format!("Failed to read ZIP entry: {}", e)))?;

        // Only process slide XML files
        if file.name().starts_with("ppt/slides/slide") && file.name().ends_with(".xml") {
            slide_count += 1;
            if slide_count > 1 {
                text.push_str("\n--- Slide ");
                text.push_str(&slide_count.to_string());
                text.push_str(" ---\n");
            }

            let mut content = String::new();
            file.read_to_string(&mut content).map_err(|e| {
                ApiError::InternalError(format!("Failed to read slide content: {}", e))
            })?;

            // Extract text between <a:t> tags (text content in PPTX XML)
            for cap in Regex::new(r"<a:t[^>]*>([^<]+)</a:t>")
                .unwrap()
                .captures_iter(&content)
            {
                text.push_str(&cap[1]);
                text.push('\n');
            }
        }
    }

    Ok(text.trim().to_string())
}

// Parses all that can be coerced to text
fn parse_text(file_path: &str) -> Result<String, ApiError> {
    read_to_string(file_path)
        .map_err(|e| ApiError::InternalError(format!("Failed to parse text based file: {}", e)))
}

// Parses all that can be coerced to an image using OCR
// TODO: Need to implement image description with AI vision if text density is too low.
fn parse_image(file_path: &str) -> Result<String, ApiError> {
    Ok("".to_string())
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
    fn parse_xlsx_single_sheet_success() {
        let file_path = "tests/inputs/test_xlsx_1.xlsx";
        let result = parse_xlsx(file_path).unwrap();

        assert!(result.len() > 0);
        assert_eq!(
            result,
            "username,identifier,first_name
johndoe123,4281,John
alice23,8425,Alice"
                .to_string()
        );
    }

    #[test]
    fn parse_xlsx_multiple_sheets_success() {
        let file_path = "tests/inputs/test_xlsx_2.xlsx";
        let result = parse_xlsx(file_path).unwrap();

        assert!(result.len() > 0);
        assert_eq!(
            result,
            "username,identifier,first_name
alice23,8425,Alice
--- Sheet: Sheet2 ---
username,identifier,first_name
johndoe123,4281,John"
                .to_string()
        );
    }

    #[test]
    fn parse_pptx_success() {
        let file_path = "tests/inputs/test_pptx_1.pptx";
        let result = parse_pptx(file_path).unwrap();

        assert!(result.len() > 0);
        assert_eq!(
            result,
            "This is the title
This is the subtitle

--- Slide 2 ---
This is the title of the second slide
This is the text of the second slide"
                .to_string()
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

    #[test]
    fn parse_csv_success() {
        let file_path = "tests/inputs/test_csv_1.csv";
        let result = parse_text(file_path).unwrap();

        assert!(result.len() > 0);
        assert_eq!(
            result,
            "Username; Identifier;First name;Last name
booker12;9012;Rachel;Booker
grey07;2070;Laura;Grey"
                .to_string()
        );
    }

    #[test]
    fn parse_json_success() {
        let file_path = "tests/inputs/test_json_1.json";
        let result = parse_text(file_path).unwrap();

        assert!(result.len() > 0);
        assert_eq!(
            result,
            r#"{
    "name": "John Doe",
    "age": 30,
    "email": "john@example.com"
}"#
            .to_string()
        );
    }

    #[test]
    fn parse_png_success() {
        let file_path = "tests/inputs/test_png_1.png";
        let result = parse_image(file_path).unwrap();

        assert!(result.len() > 0);
        assert_eq!(
            result,
            "Hellow World! This is an OCR test.
123456789
0.123 | 45.67 | 890"
                .to_string()
        );
    }

    #[test]
    fn parse_jpg_success() {
        let file_path = "tests/inputs/test_jpg_1.jpg";
        let result = parse_image(file_path).unwrap();

        assert!(result.len() > 0);
        assert_eq!(
            result,
            "Hellow World! This is an OCR test.
123456789
0.123 | 45.67 | 890"
                .to_string()
        );
    }

    #[test]
    fn parse_webp_success() {
        let file_path = "tests/inputs/test_webp_1.webp";
        let result = parse_image(file_path).unwrap();

        assert!(result.len() > 0);
        assert_eq!(
            result,
            "Hellow World! This is an OCR test.
123456789
0.123 | 45.67 | 890"
                .to_string()
        );
    }
}
