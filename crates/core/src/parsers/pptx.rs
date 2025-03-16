//! PPTX parser module.

use crate::errors::ParserError;
use regex::Regex;
use std::io::{Cursor, Read};
use zip::ZipArchive;

/// Parse a PPTX file and extract text from it.
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
