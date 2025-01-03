//! DOCX parser module.

use crate::errors::ParserError;
use docx_rs::read_docx;
use std::{fs::read, path::Path};

/// Parse a DOCX file and extract text from it.
pub fn parse_docx(file_path: &Path) -> Result<String, ParserError> {
    // Read the file contents
    let file_content = read(file_path)?;

    // Parse the DOCX document
    let docx = read_docx(&file_content)?;

    // Extract text from the document
    // TODO: Maybe simplify this monstrosity?
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
    use std::path::PathBuf;

    #[test]
    fn parse_docx_success() {
        let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("inputs")
            .join("test_docx_1.docx");
        let result = parse_docx(&file_path).unwrap();

        assert!(result.len() > 0);
        assert_eq!(
            result,
            "Hello, this is a test docx for the parsing API.".to_string()
        );
    }
}
