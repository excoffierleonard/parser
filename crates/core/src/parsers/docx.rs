//! DOCX parser module.
//!
//! This module provides functionality for extracting text from Microsoft Word DOCX
//! documents using the docx_rs library.

use crate::errors::ParserError;
use docx_rs::read_docx;

/// Parses a DOCX file and extracts text content.
///
/// This function takes raw bytes of a DOCX document and extracts all text content,
/// organizing it by paragraphs with line breaks between them.
///
/// # Arguments
///
/// * `data` - A byte slice containing the DOCX data
///
/// # Returns
///
/// * `Ok(String)` - The extracted text from the DOCX file
/// * `Err(ParserError)` - If an error occurs during DOCX parsing
///
/// # Implementation Notes
///
/// * Uses the docx_rs library for DOCX parsing
/// * Extracts text by traversing document structure: documents → paragraphs → runs → text
/// * Joins paragraphs with newlines and trims whitespace from the result
/// * TODO: Consider simplifying the document traversal logic
pub(crate) fn parse_docx(data: &[u8]) -> Result<String, ParserError> {
    // Parse the DOCX document directly from bytes
    let docx = read_docx(data)?;

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
    use parser_test_utils::read_test_file;

    #[test]
    fn parse_docx_success() {
        let data = read_test_file("test_docx_1.docx");
        let result = parse_docx(&data).unwrap();

        assert!(!result.is_empty());
        assert_eq!(
            result,
            "Hello, this is a test docx for the parsing API.".to_string()
        );
    }
}

#[cfg(test)]
pub mod benchmarks {
    use super::*;
    use criterion::{black_box, Criterion};
    use parser_test_utils::read_test_file;

    pub fn benchmark_parse_docx(c: &mut Criterion) {
        let docx_data_1 = read_test_file("test_docx_1.docx");
        let docx_data_2 = read_test_file("test_docx_2.docx");

        let mut group = c.benchmark_group("DOCX Parser");
        
        group.bench_function("parse_docx (simple)", |b| {
            b.iter(|| parse_docx(black_box(&docx_data_1)))
        });
        
        group.bench_function("parse_docx (complex)", |b| {
            b.iter(|| parse_docx(black_box(&docx_data_2)))
        });
        
        group.finish();
    }
}
