//! PPTX parser module.
//!
//! This module provides functionality for extracting text from Microsoft PowerPoint
//! PPTX presentation files. It uses the zip crate to extract slide XML files and
//! regex to extract text content.

use crate::errors::ParserError;
use regex::Regex;
use std::io::{Cursor, Read};
use zip::ZipArchive;

/// Parses a PPTX file and extracts text content from slides.
///
/// This function takes raw bytes of a PPTX presentation and extracts all text content
/// from each slide, organizing it by slide number.
///
/// # Arguments
///
/// * `data` - A byte slice containing the PPTX data
///
/// # Returns
///
/// * `Ok(String)` - The extracted text from the presentation with slide separators
/// * `Err(ParserError)` - If an error occurs during PPTX parsing
///
/// # Implementation Notes
///
/// * Treats PPTX as a ZIP archive and extracts slide XML files
/// * Uses regex to find text elements in the slide XML
/// * Organizes text by slide number with clear slide separators
/// * Handles XML content without requiring a full XML parser
pub(crate) fn parse_pptx(data: &[u8]) -> Result<String, ParserError> {
    // Create a cursor to read from the byte data
    let cursor = Cursor::new(data);

    // Create a zip archive from the cursor
    let mut archive = ZipArchive::new(cursor)?;

    // Create regex once, outside the loop
    let text_pattern = Regex::new(r"<a:t[^>]*>([^<]+)</a:t>")?;

    let mut text = String::new();
    let mut slide_count = 0;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;

        // Only process slide XML files
        if file.name().starts_with("ppt/slides/slide") && file.name().ends_with(".xml") {
            slide_count += 1;
            if slide_count > 1 {
                text.push_str("\n--- Slide ");
                text.push_str(&slide_count.to_string());
                text.push_str(" ---\n");
            }

            let mut content = String::new();
            file.read_to_string(&mut content)?;

            for cap in text_pattern.captures_iter(&content) {
                // Use get() instead of array indexing to be extra safe
                if let Some(matched) = cap.get(1) {
                    text.push_str(matched.as_str());
                    text.push('\n');
                }
            }
        }
    }

    Ok(text.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use parser_test_utils::read_test_file;

    #[test]
    fn parse_pptx_success() {
        let data = read_test_file("test_pptx_1.pptx");
        let result = parse_pptx(&data).unwrap();

        assert!(!result.is_empty());
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
}

#[cfg(test)]
pub mod benchmarks {
    use super::*;
    use criterion::{black_box, Criterion};
    use parser_test_utils::read_test_file;

    pub fn benchmark_parse_pptx(c: &mut Criterion) {
        let pptx_data = read_test_file("test_pptx_1.pptx");

        let mut group = c.benchmark_group("PPTX Parser");

        group.bench_function("parse_pptx", |b| {
            b.iter(|| parse_pptx(black_box(&pptx_data)))
        });

        group.finish();
    }
}
