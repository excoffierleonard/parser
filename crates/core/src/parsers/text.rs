//! Text parser module.

use crate::errors::ParserError;
use std::str;

/// Parse anything that can be considered as text and return its content.
pub(crate) fn parse_text(data: &[u8]) -> Result<String, ParserError> {
    // Convert bytes to string, using UTF-8 encoding
    let text = str::from_utf8(data)?;
    Ok(text.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::read, path::PathBuf};

    #[test]
    fn parse_txt_success() {
        let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("inputs")
            .join("test_txt_1.txt");
        let data = read(&file_path).unwrap();
        let result = parse_text(&data).unwrap();

        assert!(result.len() > 0);
        assert_eq!(
            result,
            "Hello, this is a test txt for the parsing API.".to_string()
        );
    }

    #[test]
    fn parse_csv_success() {
        let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("inputs")
            .join("test_csv_1.csv");
        let data = read(&file_path).unwrap();
        let result = parse_text(&data).unwrap();

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
        let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("inputs")
            .join("test_json_1.json");
        let data = read(&file_path).unwrap();
        let result = parse_text(&data).unwrap();

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
}
