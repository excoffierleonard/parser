//! Text parser module.
//!
//! This module provides functionality for parsing plain text files, including TXT,
//! CSV, and JSON formats. It focuses on UTF-8 encoded text files.

use crate::errors::ParserError;
use std::str;

/// Parses UTF-8 encoded text files and returns their content.
///
/// This function handles various text-based formats such as plain text files,
/// CSV files, and JSON files by converting their binary content to UTF-8 strings.
///
/// # Arguments
///
/// * `data` - A byte slice containing the text file data
///
/// # Returns
///
/// * `Ok(String)` - The text content from the file
/// * `Err(ParserError)` - If the data isn't valid UTF-8 or another error occurs
///
/// # Implementation Notes
///
/// * Uses the standard library's UTF-8 validation
/// * Performs no additional formatting or processing beyond UTF-8 conversion
/// * Works with plain text, CSV, JSON, and other UTF-8 encoded text formats
pub(crate) fn parse_text(data: &[u8]) -> Result<String, ParserError> {
    // Convert bytes to string, using UTF-8 encoding
    let text = str::from_utf8(data)?;
    Ok(text.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use parser_test_utils::read_test_file;

    #[test]
    fn parse_txt_success() {
        let data = read_test_file("test_txt_1.txt");
        let result = parse_text(&data).unwrap();

        assert!(!result.is_empty());
        assert_eq!(
            result,
            "Hello, this is a test txt for the parsing API.".to_string()
        );
    }

    #[test]
    fn parse_csv_success() {
        let data = read_test_file("test_csv_1.csv");
        let result = parse_text(&data).unwrap();

        assert!(!result.is_empty());
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
        let data = read_test_file("test_json_1.json");
        let result = parse_text(&data).unwrap();

        assert!(!result.is_empty());
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

#[cfg(test)]
pub mod benchmarks {
    use super::*;
    use criterion::{black_box, Criterion};
    use parser_test_utils::read_test_file;

    pub fn benchmark_parse_text(c: &mut Criterion) {
        let txt_data = read_test_file("test_txt_1.txt");
        let csv_data = read_test_file("test_csv_1.csv");
        let json_data = read_test_file("test_json_1.json");

        let mut group = c.benchmark_group("Text Parser");

        group.bench_function("parse_text (TXT)", |b| {
            b.iter(|| parse_text(black_box(&txt_data)))
        });

        group.bench_function("parse_text (CSV)", |b| {
            b.iter(|| parse_text(black_box(&csv_data)))
        });

        group.bench_function("parse_text (JSON)", |b| {
            b.iter(|| parse_text(black_box(&json_data)))
        });

        group.finish();
    }
}
