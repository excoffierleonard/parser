use crate::errors::ApiError;
use regex::Regex;
use std::io::Read;
use zip::ZipArchive;

pub fn parse_pptx(file_path: &str) -> Result<String, ApiError> {
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
