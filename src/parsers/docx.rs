use crate::errors::ApiError;
use docx_rs::read_docx;
use std::fs::read;

pub fn parse_docx(file_path: &str) -> Result<String, ApiError> {
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
